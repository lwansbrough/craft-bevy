use bevy::{
    core::{AsBytes, Byteable},
    prelude::*,
    render::{
        render_graph::{SystemNode, CommandQueue, Node, ResourceSlots},
        renderer::{RenderResourceBindings, BufferUsage, RenderResourceContext, BufferInfo, RenderResourceBinding, BufferId, RenderContext}
    },
    ecs::{ResMut, Res, Local, Commands, System, World, Resources}
};

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
                voxel_buffer: None,
                staging_buffer: None,
            },
        );
        Box::new(system)
    }
}

#[derive(Debug, Default)]
pub struct VoxelVolumeNodeState {
    command_queue: CommandQueue,
    voxel_buffer: Option<BufferId>,
    staging_buffer: Option<BufferId>,
}

pub fn voxel_node_system(
    mut state: Local<VoxelVolumeNodeState>,
    render_resource_context: Res<Box<dyn RenderResourceContext>>,
    // TODO: this write on RenderResourceBindings will prevent this system from running in parallel with other systems that do the same
    mut render_resource_bindings: ResMut<RenderResourceBindings>,
    voxel_volumes_query: Query<(&VoxelVolume, &GlobalTransform)>,
) {
    let state = &mut state;
    let render_resource_context = &**render_resource_context;

    let voxels = voxel_state.data.unwrap();

    let staging_buffer = if let Some(staging_buffer) = state.staging_buffer {
        render_resource_context.map_buffer(staging_buffer);
        staging_buffer
    } else {
        let size = voxels.len();
        let buffer = render_resource_context.create_buffer(BufferInfo {
            size,
            buffer_usage: BufferUsage::COPY_DST | BufferUsage::STORAGE,
            ..Default::default()
        });
        render_resource_bindings.set(
            super::super::storage::VOXEL_VOLUME,
            RenderResourceBinding::Buffer {
                buffer,
                range: 0..size as u64,
                dynamic_index: None,
            },
        );
        state.voxel_buffer = Some(buffer);

        let staging_buffer = render_resource_context.create_buffer(BufferInfo {
            size,
            buffer_usage: BufferUsage::COPY_SRC | BufferUsage::MAP_WRITE,
            mapped_at_creation: true,
        });

        state.staging_buffer = Some(staging_buffer);
        staging_buffer
    };

    let voxels_size = std::mem::size_of::<[f32; 2]>();

    render_resource_context.write_mapped_buffer(
        staging_buffer,
        0..voxels_size as u64,
        &mut |data, _renderer| {
            data[0..voxels_size].copy_from_slice(&voxels.as_bytes());
        },
    );
    render_resource_context.unmap_buffer(staging_buffer);

    let voxel_buffer = state.voxel_buffer.unwrap();
    state.command_queue.copy_buffer_to_buffer(
        staging_buffer,
        0,
        voxel_buffer,
        0,
        voxels_size as u64,
    );
}