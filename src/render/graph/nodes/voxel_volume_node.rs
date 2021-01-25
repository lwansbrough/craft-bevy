use bevy::{core::{AsBytes, Byteable}, ecs::{ResMut, Res, Local, Commands, System, World, Resources}, prelude::*, render::{render_graph::{SystemNode, CommandQueue, Node, ResourceSlots}, renderer::{BufferId, BufferInfo, BufferUsage, RenderContext, RenderResourceBinding, RenderResourceBindings, RenderResourceContext, RenderResourceId}}, utils::{HashMap, HashSet}};

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
        _resources: &Resources,
        render_context: &mut dyn RenderContext,
        _input: &ResourceSlots,
        _output: &mut ResourceSlots,
    ) {
        self.command_queue.execute(render_context);
    }
}

impl SystemNode for VoxelVolumeNode {
    fn get_system(&self, commands: &mut Commands) -> Box<dyn System<In = (), Out = ()>> {
        let system = voxel_node_system.system();
        commands.insert_local_resource(
            system.id(),
            VoxelVolumeNodeState {
                command_queue: self.command_queue.clone(),
                ..Default::default()
            },
        );
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
    voxel_volume_event_reader: EventReader<AssetEvent<VoxelVolume>>,
    voxel_entities: HashMap<Handle<VoxelVolume>, VoxelEntities>,
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

pub fn voxel_node_system(
    mut state: Local<VoxelVolumeNodeState>,
    render_resource_context: Res<Box<dyn RenderResourceContext>>,
    // TODO: this write on RenderResourceBindings will prevent this system from running in parallel with other systems that do the same
    voxel_volumes: Res<Assets<VoxelVolume>>,
    voxel_events: Res<Events<AssetEvent<VoxelVolume>>>,
    mut queries: QuerySet<(
        Query<&mut RenderPipelines, With<Handle<VoxelVolume>>>,
        Query<(Entity, &Handle<VoxelVolume>), Changed<Handle<VoxelVolume>>>,
    )>,
) {
    let state = &mut state;
    let mut changed_voxel_volumes = HashSet::default();
    let render_resource_context = &**render_resource_context;

    for event in state.voxel_volume_event_reader.iter(&voxel_events) {
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

    // update changed voxel data
    for changed_voxel_volume_handle in changed_voxel_volumes.iter() {
        if let Some(voxel_volume) = voxel_volumes.get(changed_voxel_volume_handle) {
            // TODO: check for individual buffer changes in non-interleaved mode
            let data = voxel_volume.data.as_bytes();
            let size = data.len();

            if let Some(RenderResourceId::Buffer(staging_buffer)) =
                render_resource_context.get_asset_resource(changed_voxel_volume_handle, VOXEL_VOLUME_STAGING_BUFFER_ID)
            {
                render_resource_context.map_buffer(staging_buffer)
            } else {
                let voxel_buffer = render_resource_context.create_buffer(
                    BufferInfo {
                        size,
                        buffer_usage: BufferUsage::COPY_DST | BufferUsage::STORAGE,
                        ..Default::default()
                    }
                );
    
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

                render_resource_context.set_asset_resource(
                    changed_voxel_volume_handle,
                    RenderResourceId::Buffer(staging_buffer),
                    VOXEL_VOLUME_STAGING_BUFFER_ID,
                );
            }
            
            // update_entity_voxel_volume(
            //     &mut state.command_queue,
            //     render_resource_context,
            //     voxel_volume,
            //     changed_voxel_volume_handle,
            // );
        }
    }

    // handover buffers to pipeline
    for (entity, handle) in queries.q1_mut().iter_mut() {
        let voxel_entities = state
            .voxel_entities
            .entry(handle.clone_weak())
            .or_insert_with(VoxelEntities::default);
        
        voxel_entities.entities.insert(entity);
        
        if let Some(voxel_volume) = voxel_volumes.get(handle) {
            update_entity_voxel_volume(&mut state.command_queue, render_resource_context, voxel_volume, handle);
        }
    }
}

fn update_entity_voxel_volume(
    command_queue: &mut CommandQueue,
    render_resource_context: &dyn RenderResourceContext,
    voxel_volume: &VoxelVolume,
    handle: &Handle<VoxelVolume>,
) {
    if let Some(RenderResourceId::Buffer(staging_buffer)) =
        render_resource_context.get_asset_resource(handle, VOXEL_VOLUME_STAGING_BUFFER_ID)
    {
        let voxels = &voxel_volume.data;
        let voxels_bytes = voxels.as_bytes();
        let voxels_size = voxels_bytes.len();
        render_resource_context.write_mapped_buffer(
            staging_buffer,
            0..voxels_size as u64,
            &mut |data, _renderer| {
                data[0..voxels_size].copy_from_slice(voxels_bytes);
            },
        );
        render_resource_context.unmap_buffer(staging_buffer);

        if let Some(RenderResourceId::Buffer(voxel_buffer)) =
        render_resource_context.get_asset_resource(handle, VOXEL_VOLUME_BUFFER_ID)
        {
            command_queue.copy_buffer_to_buffer(
                staging_buffer,
                0,
                voxel_buffer,
                0,
                voxels_size as u64,
            );
        }
    }
}