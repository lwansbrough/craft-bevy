use bevy::{core::{AsBytes, Byteable}, prelude::*, render::{render_graph::{SystemNode, CommandQueue, Node, ResourceSlots}, renderer::{BufferId, BufferInfo, BufferMapMode, BufferUsage, RenderContext, RenderResourceBinding, RenderResourceBindings, RenderResourceContext, RenderResourceId}, texture::{Extent3d, SamplerDescriptor, TextureDescriptor, TextureFormat, TextureUsage}}, utils::{HashMap, HashSet}};
use crate::VoxelVolume;

#[derive(Debug)]
pub struct VoxelVolumeNode {
    command_queue: CommandQueue
}

impl VoxelVolumeNode {
    pub fn new() -> Self {
        VoxelVolumeNode {
            command_queue: Default::default()
        }
    }
}

impl Node for VoxelVolumeNode {
    fn update(
        &mut self,
        _world: &World,
        render_context: &mut dyn RenderContext,
        _input: &ResourceSlots,
        _output: &mut ResourceSlots,
    ) {
        self.command_queue.execute(render_context);
    }
}

impl SystemNode for VoxelVolumeNode {
    fn get_system(&self) -> Box<dyn System<In = (), Out = ()>> {
        let system = voxel_node_system.system().config(|config| {
            config.0 = Some(VoxelVolumeNodeState {
                command_queue: self.command_queue.clone(),
                ..Default::default()
            })
        });
        Box::new(system)
    }
}

#[derive(Default)]
pub struct VoxelEntities {
    entities: HashSet<Entity>,
}

#[derive(Default)]
pub struct VoxelVolumeNodeState {
    command_queue: CommandQueue,
    voxel_entities: HashMap<Handle<VoxelVolume>, VoxelEntities>,
    voxel_staging_buffers: HashMap<Handle<VoxelVolume>, BufferId>,
    voxel_buffers: HashMap<Handle<VoxelVolume>, BufferId>,
}

pub const VOXEL_VOLUME_BUFFER_ID: u64 = 0;
pub const VOXEL_VOLUME_STAGING_BUFFER_ID: u64 = 1;

fn remove_resource_save(
    render_resource_context: &dyn RenderResourceContext,
    handle: &Handle<VoxelVolume>,
    index: u64,
) {
    if let Some(RenderResourceId::Buffer(buffer)) =
        render_resource_context.get_asset_resource(&handle, index)
    {
        render_resource_context.remove_buffer(buffer);
        render_resource_context.remove_asset_resource(handle, index);
    }
}
fn remove_current_voxel_resources(
    render_resource_context: &dyn RenderResourceContext,
    handle: &Handle<VoxelVolume>,
) {
    remove_resource_save(render_resource_context, handle, VOXEL_VOLUME_BUFFER_ID);
}

// Reference: https://github.com/bevyengine/bevy/blob/b17f8a4bce5551b418654fffb1fe97ff0f9852f0/crates/bevy_render/src/render_graph/nodes/render_resources_node.rs#L620

pub fn voxel_node_system(
    mut state: Local<VoxelVolumeNodeState>,
    render_resource_context: Res<Box<dyn RenderResourceContext>>,
    mut render_resource_bindings: ResMut<RenderResourceBindings>,
    // TODO: this write on RenderResourceBindings will prevent this system from running in parallel with other systems that do the same
    voxel_volumes: Res<Assets<VoxelVolume>>,
    mut voxel_events: EventReader<AssetEvent<VoxelVolume>>,
    // mut queries: QuerySet<(
    //     Query<&mut RenderPipelines, With<Handle<VoxelVolume>>>,
    //     Query<(Entity, &Handle<VoxelVolume>, &mut RenderPipelines), Changed<Handle<VoxelVolume>>>,
    // )>,
    mut queries: Query<(&Handle<VoxelVolume>, &mut RenderPipelines)>,
) {
    let state = &mut state;
    let mut changed_voxel_volumes = HashSet::default();
    let render_resource_context = &**render_resource_context;

    for event in voxel_events.iter() {
        match event {
            AssetEvent::Created { ref handle } => {
                changed_voxel_volumes.insert(handle.clone_weak());
            }
            AssetEvent::Modified { ref handle } => {
                changed_voxel_volumes.insert(handle.clone_weak());
                remove_current_voxel_resources(render_resource_context, handle);
            }
            AssetEvent::Removed { ref handle } => {
                remove_current_voxel_resources(render_resource_context, handle);
                // if voxel volume was modified and removed in the same update, ignore the modification
                // events are ordered so future modification events are ok
                changed_voxel_volumes.remove(handle);
            }
        }
    }

    let mut changed = HashSet::<Handle<VoxelVolume>>::default();

    // update changed voxel data
    for changed_voxel_volume_handle in changed_voxel_volumes.iter() {

        if changed.contains(changed_voxel_volume_handle) {
            continue;
        }
        
        if let Some(voxel_volume) = voxel_volumes.get(changed_voxel_volume_handle) {
            changed.insert(changed_voxel_volume_handle.clone_weak());
            // TODO: check for individual buffer changes in non-interleaved mode
            let size = voxel_volume.byte_len();

            if state.voxel_staging_buffers.contains_key(changed_voxel_volume_handle) {
                let staging_buffer = *state.voxel_staging_buffers.get(changed_voxel_volume_handle).unwrap();
                render_resource_context.map_buffer(staging_buffer, BufferMapMode::Write)
            } else {
                let voxel_buffer = render_resource_context.create_buffer(
                    BufferInfo {
                        size,
                        buffer_usage: BufferUsage::COPY_DST | BufferUsage::STORAGE,
                        ..Default::default()
                    }
                );
                state.voxel_buffers.insert(changed_voxel_volume_handle.clone_weak(), voxel_buffer);
    
                render_resource_context.set_asset_resource(
                    changed_voxel_volume_handle,
                    RenderResourceId::Buffer(voxel_buffer),
                    VOXEL_VOLUME_BUFFER_ID,
                );
    
                let staging_buffer = render_resource_context.create_buffer(BufferInfo {
                    size,
                    buffer_usage: BufferUsage::COPY_SRC | BufferUsage::MAP_WRITE,
                    mapped_at_creation: true,
                });
                state.voxel_staging_buffers.insert(changed_voxel_volume_handle.clone_weak(), staging_buffer);

                render_resource_context.set_asset_resource(
                    changed_voxel_volume_handle,
                    RenderResourceId::Buffer(staging_buffer),
                    VOXEL_VOLUME_STAGING_BUFFER_ID,
                );
            };

            if let Some(voxel_volume) = voxel_volumes.get(changed_voxel_volume_handle) {
                let voxels_bytes = &voxel_volume.to_bytes();
                let voxels_size = voxels_bytes.len();
    
                let voxel_staging_buffer = *state.voxel_staging_buffers.get(changed_voxel_volume_handle).unwrap();
                let voxel_buffer = *state.voxel_buffers.get(changed_voxel_volume_handle).unwrap();
    
                render_resource_context.write_mapped_buffer(
                    voxel_staging_buffer,
                    0..voxels_size as u64,
                    &mut |data, _renderer| {
                        data[0..voxels_size].copy_from_slice(&voxels_bytes[..]);
                    },
                );
                render_resource_context.unmap_buffer(voxel_staging_buffer);
    
                state.command_queue.copy_buffer_to_buffer(
                    voxel_staging_buffer,
                    0,
                    voxel_buffer,
                    0,
                    voxels_size as u64,
                );
    
                for (voxel_volume_handle, mut render_pipelines) in queries.iter_mut() {
                    if voxel_volume_handle == changed_voxel_volume_handle {
                        render_pipelines.bindings.set(
                            super::super::storage::VOXEL_VOLUME,
                            RenderResourceBinding::Buffer {
                                buffer: voxel_buffer,
                                range: 0..voxels_size as u64,
                                dynamic_index: None,
                            },
                        );
                    }
                }
            }
        }
    }

    // disabled because changes aren't detected on Changed
    // for (entity, handle, mut render_pipelines) in queries.q1_mut().iter_mut() {
    //     let voxel_entities = state
    //         .voxel_entities
    //         .entry(handle.clone_weak())
    //         .or_insert_with(VoxelEntities::default);
        
    //     voxel_entities.entities.insert(entity);

    //     if let Some(voxel_volume) = voxel_volumes.get(handle) {
    //         println!("writing buffer");
    //         let voxels_bytes = &voxel_volume.to_bytes();
    //         let voxels_size = voxels_bytes.len();

    //         let voxel_staging_buffer = *state.voxel_staging_buffers.get(handle).unwrap();
    //         let voxel_buffer = *state.voxel_buffers.get(handle).unwrap();

    //         render_resource_context.write_mapped_buffer(
    //             voxel_staging_buffer,
    //             0..voxels_size as u64,
    //             &mut |data, _renderer| {
    //                 data[0..voxels_size].copy_from_slice(&voxels_bytes[..]);
    //             },
    //         );
    //         render_resource_context.unmap_buffer(voxel_staging_buffer);

    //         state.command_queue.copy_buffer_to_buffer(
    //             voxel_staging_buffer,
    //             0,
    //             voxel_buffer,
    //             0,
    //             voxels_size as u64,
    //         );

    //         render_pipelines.bindings.set(
    //             super::super::storage::VOXEL_VOLUME,
    //             RenderResourceBinding::Buffer {
    //                 buffer: voxel_buffer,
    //                 range: 0..voxels_size as u64,
    //                 dynamic_index: None,
    //             },
    //         );
    //     }
    // }
}
