use bevy::{
    prelude::*,
    render::{
        render_graph::{SystemNode, CommandQueue, Node, ResourceSlots},
        renderer::{RenderResourceBindings, BufferUsage, RenderResourceContext, BufferInfo, RenderResourceBinding, BufferId, RenderContext}
    },
    ecs::{ResMut, Res, Local, Commands, System, World, Resources}
};

#[derive(Debug)]
pub struct TimeNode {
    command_queue: CommandQueue
}

impl TimeNode {
    pub fn new() -> Self {
        TimeNode {
            command_queue: Default::default()
        }
    }
}

impl Node for TimeNode {
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

impl SystemNode for TimeNode {
    fn get_system(&self, commands: &mut Commands) -> Box<dyn System<In = (), Out = ()>> {
        let system = time_node_system.system();
        commands.insert_local_resource(
            system.id(),
            TimeNodeState {
                command_queue: self.command_queue.clone(),
                time_buffer: None,
                staging_buffer: None,
            },
        );
        Box::new(system)
    }
}

#[derive(Debug, Default)]
pub struct TimeNodeState {
    command_queue: CommandQueue,
    time_buffer: Option<BufferId>,
    staging_buffer: Option<BufferId>,
}

pub fn time_node_system(
    mut state: Local<TimeNodeState>,
    render_resource_context: Res<Box<dyn RenderResourceContext>>,
    // TODO: this write on RenderResourceBindings will prevent this system from running in parallel with other systems that do the same
    mut render_resource_bindings: ResMut<RenderResourceBindings>,
    time: Res<Time>,
) {
    let state = &mut state;
    let render_resource_context = &**render_resource_context;

    let staging_buffer = if let Some(staging_buffer) = state.staging_buffer {
        render_resource_context.map_buffer(staging_buffer);
        staging_buffer
    } else {
        let size = std::mem::size_of::<f64>();
        let buffer = render_resource_context.create_buffer(BufferInfo {
            size,
            buffer_usage: BufferUsage::COPY_DST | BufferUsage::UNIFORM,
            ..Default::default()
        });
        render_resource_bindings.set(
            super::super::uniform::TIME,
            RenderResourceBinding::Buffer {
                buffer,
                range: 0..size as u64,
                dynamic_index: None,
            },
        );
        state.time_buffer = Some(buffer);

        let staging_buffer = render_resource_context.create_buffer(BufferInfo {
            size,
            buffer_usage: BufferUsage::COPY_SRC | BufferUsage::MAP_WRITE,
            mapped_at_creation: true,
        });

        state.staging_buffer = Some(staging_buffer);
        staging_buffer
    };

    let time_size = std::mem::size_of::<f64>();

    render_resource_context.write_mapped_buffer(
        staging_buffer,
        0..time_size as u64,
        &mut |data, _renderer| {
            data[0..time_size].copy_from_slice(&time.seconds_since_startup().to_ne_bytes());
        },
    );
    render_resource_context.unmap_buffer(staging_buffer);

    let time_buffer = state.time_buffer.unwrap();
    state.command_queue.copy_buffer_to_buffer(
        staging_buffer,
        0,
        time_buffer,
        0,
        time_size as u64,
    );
}