use bevy::{core::{AsBytes, Byteable}, ecs::{ResMut, Res, Local, Commands, System, World, Resources}, prelude::*, render::{render_graph::{SystemNode, CommandQueue, Node, ResourceSlots}, renderer::{BufferId, BufferInfo, BufferUsage, RenderContext, RenderResourceBinding, RenderResourceBindings, RenderResourceContext, RenderResourceId}, texture::{Extent3d, SamplerDescriptor, TextureDescriptor, TextureFormat, TextureUsage}}, utils::{HashMap, HashSet}};

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

#[derive(Default)]
pub struct VoxelResourceProviderState {
    voxel_volume_event_reader: EventReader<AssetEvent<VoxelVolume>>,
    voxel_entities: HashMap<Handle<VoxelVolume>, VoxelEntities>,
}

pub const VOXEL_VOLUME_SIZE_STAGING_BUFFER_ID: u64 = 0;
pub const VOXEL_VOLUME_SIZE_BUFFER_ID: u64 = 1;
pub const VOXEL_VOLUME_PALETTE_STAGING_BUFFER_ID: u64 = 2;
pub const VOXEL_VOLUME_DATA_STAGING_BUFFER_ID: u64 = 3;
pub const VOXEL_VOLUME_PALETTE_TEXTURE_ID: u64 = 4;
pub const VOXEL_VOLUME_DATA_TEXTURE_ID: u64 = 5;
pub const VOXEL_VOLUME_PALETTE_TEXTURE_SAMPLER_ID: u64 = 6;
pub const VOXEL_VOLUME_DATA_TEXTURE_SAMPLER_ID: u64 = 7;

fn remove_resource_save(
    render_resource_context: &dyn RenderResourceContext,
    handle: &Handle<VoxelVolume>,
    index: u64,
) {
    
}
fn remove_current_voxel_resources(
    render_resource_context: &dyn RenderResourceContext,
    handle: &Handle<VoxelVolume>,
) {
    if let Some(RenderResourceId::Buffer(buffer)) =
        render_resource_context.get_asset_resource(&handle, VOXEL_VOLUME_SIZE_STAGING_BUFFER_ID)
    {
        render_resource_context.remove_buffer(buffer);
        render_resource_context.remove_asset_resource(handle, VOXEL_VOLUME_SIZE_STAGING_BUFFER_ID);
    }

    if let Some(RenderResourceId::Buffer(buffer)) =
        render_resource_context.get_asset_resource(&handle, VOXEL_VOLUME_SIZE_BUFFER_ID)
    {
        render_resource_context.remove_buffer(buffer);
        render_resource_context.remove_asset_resource(handle, VOXEL_VOLUME_SIZE_BUFFER_ID);
    }

    if let Some(RenderResourceId::Buffer(buffer)) =
        render_resource_context.get_asset_resource(&handle, VOXEL_VOLUME_PALETTE_STAGING_BUFFER_ID)
    {
        render_resource_context.remove_buffer(buffer);
        render_resource_context.remove_asset_resource(handle, VOXEL_VOLUME_PALETTE_STAGING_BUFFER_ID);
    }

    if let Some(RenderResourceId::Buffer(buffer)) =
        render_resource_context.get_asset_resource(&handle, VOXEL_VOLUME_DATA_STAGING_BUFFER_ID)
    {
        render_resource_context.remove_buffer(buffer);
        render_resource_context.remove_asset_resource(handle, VOXEL_VOLUME_DATA_STAGING_BUFFER_ID);
    }

    if let Some(RenderResourceId::Texture(texture)) =
        render_resource_context.get_asset_resource(&handle, VOXEL_VOLUME_PALETTE_TEXTURE_ID)
    {
        render_resource_context.remove_texture(texture);
        render_resource_context.remove_asset_resource(handle, VOXEL_VOLUME_PALETTE_TEXTURE_ID);
    }

    if let Some(RenderResourceId::Texture(texture)) =
        render_resource_context.get_asset_resource(&handle, VOXEL_VOLUME_DATA_TEXTURE_ID)
    {
        render_resource_context.remove_texture(texture);
        render_resource_context.remove_asset_resource(handle, VOXEL_VOLUME_DATA_TEXTURE_ID);
    }

    if let Some(RenderResourceId::Sampler(sampler)) =
        render_resource_context.get_asset_resource(&handle, VOXEL_VOLUME_PALETTE_TEXTURE_SAMPLER_ID)
    {
        render_resource_context.remove_sampler(sampler);
        render_resource_context.remove_asset_resource(handle, VOXEL_VOLUME_PALETTE_TEXTURE_SAMPLER_ID);
    }

    if let Some(RenderResourceId::Sampler(sampler)) =
        render_resource_context.get_asset_resource(&handle, VOXEL_VOLUME_DATA_TEXTURE_SAMPLER_ID)
    {
        render_resource_context.remove_sampler(sampler);
        render_resource_context.remove_asset_resource(handle, VOXEL_VOLUME_DATA_TEXTURE_SAMPLER_ID);
    }
}

pub fn voxel_node_system(
    mut state: Local<VoxelVolumeNodeState>,
    render_resource_context: Res<Box<dyn RenderResourceContext>>,
    mut render_resource_bindings: ResMut<RenderResourceBindings>,
    // TODO: this write on RenderResourceBindings will prevent this system from running in parallel with other systems that do the same
    voxel_volumes: Res<Assets<VoxelVolume>>,
    voxel_events: Res<Events<AssetEvent<VoxelVolume>>>,
    mut queries: QuerySet<(
        Query<&mut RenderPipelines, With<Handle<VoxelVolume>>>,
        Query<(Entity, &Handle<VoxelVolume>, &mut RenderPipelines), Changed<Handle<VoxelVolume>>>,
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

            if let Some(RenderResourceId::Buffer(staging_buffer)) =
                render_resource_context.get_asset_resource(changed_voxel_volume_handle, VOXEL_VOLUME_DATA_STAGING_BUFFER_ID)
            {
                render_resource_context.map_buffer(staging_buffer)
            } else {
                let size_staging_buffer = render_resource_context.create_buffer(BufferInfo {
                    size: std::mem::size_of::<[u32; 3]>(),
                    buffer_usage: BufferUsage::COPY_SRC | BufferUsage::MAP_WRITE,
                    mapped_at_creation: true,
                });

                let palette_staging_buffer = render_resource_context.create_buffer(BufferInfo {
                    size: voxel_volume.palette_len(),
                    buffer_usage: BufferUsage::COPY_SRC | BufferUsage::MAP_WRITE,
                    mapped_at_creation: true,
                });

                let data_staging_buffer = render_resource_context.create_buffer(BufferInfo {
                    size: voxel_volume.data_len(),
                    buffer_usage: BufferUsage::COPY_SRC | BufferUsage::MAP_WRITE,
                    mapped_at_creation: true,
                });
                
                let volume_size_buffer = render_resource_context.create_buffer(BufferInfo {
                    size: std::mem::size_of::<[u32; 3]>(),
                    buffer_usage: BufferUsage::COPY_DST | BufferUsage::UNIFORM,
                    ..Default::default()
                });

                let palette_texture = render_resource_context.create_texture(
                    TextureDescriptor {
                        format: TextureFormat::Rgba32Float,
                        size: Extent3d::new(voxel_volume.palette.len() as u32, 1, 1),
                        usage: TextureUsage::COPY_DST | TextureUsage::SAMPLED,
                        ..Default::default()
                    }
                );

                let palette_texture_sampler = render_resource_context.create_sampler(
                    &SamplerDescriptor::default()
                );

                let data_texture = render_resource_context.create_texture(
                    TextureDescriptor {
                        format: TextureFormat::R8Uint,
                        size: Extent3d::new(voxel_volume.size.x as u32, voxel_volume.size.y as u32, voxel_volume.size.z as u32),
                        usage: TextureUsage::COPY_DST | TextureUsage::SAMPLED,
                        ..Default::default()
                    }
                );

                let data_texture_sampler = render_resource_context.create_sampler(
                    &SamplerDescriptor::default()
                );

                render_resource_context.set_asset_resource(
                    changed_voxel_volume_handle,
                    RenderResourceId::Buffer(volume_size_buffer),
                    VOXEL_VOLUME_SIZE_BUFFER_ID,
                );

                render_resource_context.set_asset_resource(
                    changed_voxel_volume_handle,
                    RenderResourceId::Texture(palette_texture),
                    VOXEL_VOLUME_PALETTE_TEXTURE_ID,
                );

                render_resource_context.set_asset_resource(
                    changed_voxel_volume_handle,
                    RenderResourceId::Texture(palette_texture),
                    VOXEL_VOLUME_PALETTE_TEXTURE_ID,
                );

                render_resource_context.set_asset_resource(
                    changed_voxel_volume_handle,
                    RenderResourceId::Sampler(palette_texture_sampler),
                    VOXEL_VOLUME_PALETTE_TEXTURE_SAMPLER_ID,
                );

                render_resource_context.set_asset_resource(
                    changed_voxel_volume_handle,
                    RenderResourceId::Texture(data_texture),
                    VOXEL_VOLUME_DATA_TEXTURE_ID,
                );

                render_resource_context.set_asset_resource(
                    changed_voxel_volume_handle,
                    RenderResourceId::Sampler(data_texture_sampler),
                    VOXEL_VOLUME_DATA_TEXTURE_SAMPLER_ID,
                );

                render_resource_context.set_asset_resource(
                    changed_voxel_volume_handle,
                    RenderResourceId::Buffer(palette_staging_buffer),
                    VOXEL_VOLUME_PALETTE_STAGING_BUFFER_ID,
                );

                render_resource_context.set_asset_resource(
                    changed_voxel_volume_handle,
                    RenderResourceId::Buffer(data_staging_buffer),
                    VOXEL_VOLUME_DATA_STAGING_BUFFER_ID,
                );
            }
        }
    }

    // handover buffers to pipeline
    for (entity, handle, render_pipelines) in queries.q1_mut().iter_mut() {
        let voxel_entities = state
            .voxel_entities
            .entry(handle.clone_weak())
            .or_insert_with(VoxelEntities::default);
        
        voxel_entities.entities.insert(entity);
        
        if let Some(voxel_volume) = voxel_volumes.get(handle) {
            update_entity_voxel_volume(&mut state.command_queue, render_resource_context, voxel_volume, handle, render_pipelines);
        }
    }
}

fn update_entity_voxel_volume(
    command_queue: &mut CommandQueue,
    render_resource_context: &dyn RenderResourceContext,
    voxel_volume: &VoxelVolume,
    handle: &Handle<VoxelVolume>,
    mut render_pipelines: Mut<RenderPipelines>,
) {
    if let Some(RenderResourceId::Buffer(volume_size_staging_buffer)) =
            render_resource_context.get_asset_resource(handle, VOXEL_VOLUME_SIZE_STAGING_BUFFER_ID)
    {
        let size_bytes = voxel_volume.size.as_bytes();
        let size = size_bytes.len();
        render_resource_context.write_mapped_buffer(
            volume_size_staging_buffer,
            0..size as u64,
            &mut |data, _renderer| {
                data[0..size].copy_from_slice(&size_bytes[..]);
            },
        );
        render_resource_context.unmap_buffer(volume_size_staging_buffer);

        if let Some(RenderResourceId::Buffer(volume_size_buffer)) =
            render_resource_context.get_asset_resource(handle, VOXEL_VOLUME_SIZE_BUFFER_ID)
        {
            render_pipelines.bindings.set(
                "VoxelVolume_size",
                RenderResourceBinding::Buffer {
                    buffer: volume_size_buffer,
                    range: 0..size as u64,
                    dynamic_index: None,
                },
            );

            command_queue.copy_buffer_to_buffer(
                volume_size_staging_buffer,
                0,
                volume_size_buffer,
                0,
                size as u64
            );
        }
    }

    if let Some(RenderResourceId::Buffer(palette_staging_buffer)) =
        render_resource_context.get_asset_resource(handle, VOXEL_VOLUME_PALETTE_STAGING_BUFFER_ID)
    {
        let palette_bytes = voxel_volume.palette.as_bytes();
        let palette_size = palette_bytes.len();
        render_resource_context.write_mapped_buffer(
            palette_staging_buffer,
            0..palette_size as u64,
            &mut |data, _renderer| {
                data[0..palette_size].copy_from_slice(&palette_bytes[..]);
            },
        );
        render_resource_context.unmap_buffer(palette_staging_buffer);

        if let Some(RenderResourceId::Texture(palette_texture)) =
            render_resource_context.get_asset_resource(handle, VOXEL_VOLUME_PALETTE_TEXTURE_ID)
        {
            render_pipelines.bindings.set(
                "VoxelVolume_palette",
                RenderResourceBinding::Texture(palette_texture)
            );

            command_queue.copy_buffer_to_texture(
                palette_staging_buffer,
                0,
                256,
                palette_texture,
                [0, 0, 0],
                1,
                Extent3d::new(256, 1, 1)
            );
        }

        if let Some(RenderResourceId::Sampler(palette_texture_sampler)) =
            render_resource_context.get_asset_resource(handle, VOXEL_VOLUME_PALETTE_TEXTURE_SAMPLER_ID)
        {
            render_pipelines.bindings.set(
                "VoxelVolume_palette_sampler",
                RenderResourceBinding::Sampler(palette_texture_sampler)
            );
        }
    }

    if let Some(RenderResourceId::Buffer(data_staging_buffer)) =
        render_resource_context.get_asset_resource(handle, VOXEL_VOLUME_DATA_STAGING_BUFFER_ID)
    {
        let data_bytes = voxel_volume.data.as_bytes();
        let data_size = data_bytes.len();
        render_resource_context.write_mapped_buffer(
            data_staging_buffer,
            0..data_size as u64,
            &mut |data, _renderer| {
                data[0..data_size].copy_from_slice(&data_bytes[..]);
            },
        );
        render_resource_context.unmap_buffer(data_staging_buffer);

        if let Some(RenderResourceId::Texture(data_texture)) =
            render_resource_context.get_asset_resource(handle, VOXEL_VOLUME_DATA_TEXTURE_ID)
        {
            render_pipelines.bindings.set(
                "VoxelVolume_data",
                RenderResourceBinding::Texture(data_texture)
            );

            command_queue.copy_buffer_to_texture(
                data_staging_buffer,
                0,
                4096,
                data_texture,
                [0, 0, 0],
                1,
                Extent3d::new(4096, 4096, 1)
            );
        }

        if let Some(RenderResourceId::Sampler(data_texture_sampler)) =
            render_resource_context.get_asset_resource(handle, VOXEL_VOLUME_DATA_TEXTURE_SAMPLER_ID)
        {
            render_pipelines.bindings.set(
                "VoxelVolume_data_sampler",
                RenderResourceBinding::Sampler(data_texture_sampler)
            );
        }
    }
}