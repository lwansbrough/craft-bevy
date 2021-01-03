use bevy::{
    core::{AsBytes, Byteable},
    prelude::*,
    render::{
        render_graph::{SystemNode, CommandQueue, Node, ResourceSlots},
        renderer::{RenderResourceBindings, BufferUsage, RenderResourceContext, BufferInfo, RenderResourceBinding, BufferId, RenderContext}
    },
    ecs::{ResMut, Res, Local, Commands, System, World, Resources}
};

use crate::resources::WindowResizeEventListenerState;

#[derive(Debug)]
pub struct ResolutionNode {
    command_queue: CommandQueue
}

impl ResolutionNode {
    pub fn new() -> Self {
        ResolutionNode {
            command_queue: Default::default()
        }
    }
}

impl Node for ResolutionNode {
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

impl SystemNode for ResolutionNode {
    fn get_system(&self, commands: &mut Commands) -> Box<dyn System<In = (), Out = ()>> {
        let system = resolution_node_system.system();
        commands.insert_local_resource(
            system.id(),
            ResolutionNodeState {
                command_queue: self.command_queue.clone(),
                resolution_buffer: None,
                staging_buffer: None,
            },
        );
        Box::new(system)
    }
}

#[derive(Debug, Default)]
pub struct ResolutionNodeState {
    command_queue: CommandQueue,
    resolution_buffer: Option<BufferId>,
    staging_buffer: Option<BufferId>,
}

pub fn resolution_node_system(
    mut state: Local<ResolutionNodeState>,
    render_resource_context: Res<Box<dyn RenderResourceContext>>,
    // TODO: this write on RenderResourceBindings will prevent this system from running in parallel with other systems that do the same
    mut render_resource_bindings: ResMut<RenderResourceBindings>,
    resolution_state: Res<WindowResizeEventListenerState>,
) {
    let state = &mut state;
    let render_resource_context = &**render_resource_context;

    let resolution = resolution_state.resolution.unwrap();

    let staging_buffer = if let Some(staging_buffer) = state.staging_buffer {
        render_resource_context.map_buffer(staging_buffer);
        staging_buffer
    } else {
        let size = std::mem::size_of::<[f32; 2]>();
        let buffer = render_resource_context.create_buffer(BufferInfo {
            size,
            buffer_usage: BufferUsage::COPY_DST | BufferUsage::UNIFORM,
            ..Default::default()
        });
        render_resource_bindings.set(
            super::super::uniform::RESOLUTION,
            RenderResourceBinding::Buffer {
                buffer,
                range: 0..size as u64,
                dynamic_index: None,
            },
        );
        state.resolution_buffer = Some(buffer);

        let staging_buffer = render_resource_context.create_buffer(BufferInfo {
            size,
            buffer_usage: BufferUsage::COPY_SRC | BufferUsage::MAP_WRITE,
            mapped_at_creation: true,
        });

        state.staging_buffer = Some(staging_buffer);
        staging_buffer
    };

    let resolution_size = std::mem::size_of::<[f32; 2]>();

    render_resource_context.write_mapped_buffer(
        staging_buffer,
        0..resolution_size as u64,
        &mut |data, _renderer| {
            data[0..resolution_size].copy_from_slice(&resolution.as_bytes());
        },
    );
    render_resource_context.unmap_buffer(staging_buffer);

    let resolution_buffer = state.resolution_buffer.unwrap();
    state.command_queue.copy_buffer_to_buffer(
        staging_buffer,
        0,
        resolution_buffer,
        0,
        resolution_size as u64,
    );
}