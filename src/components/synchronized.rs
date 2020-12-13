use bevy::ecs::{Entity, Command, Component, Resources, World, Commands};
use serde::{Serialize, ser::Serializer};
use crate::models::{CommandFrameBuffer, StateFrameBuffer, StateFrame, Synchronizable, synchronizable};
use std::io::Write;

pub struct Synchronized {
    component_type_id: u8,
    author_state_inner: fn(u8, &World, &mut Resources, Entity, u32) -> StateFrame,
    consume_state_inner: fn(&mut World, &mut Resources, Entity, StateFrame),
    command_frame_buffer: CommandFrameBuffer,
    state_frame_buffer: StateFrameBuffer,
}

pub struct SynchronizedStateAuthoring {
    author_state_inner: fn(u8, &World, &mut Resources, Entity, u32) -> StateFrame,
    component_type_id: u8,
    entity: Entity,
    frame: u32,
    state_frame_buffer: StateFrameBuffer
}

pub struct SynchronizedStateConsumption {
    consume_state_inner: fn(&mut World, &mut Resources, Entity, StateFrame),
    component_type_id: u8,
    entity: Entity,
    state_frame: StateFrame
}

impl Command for SynchronizedStateAuthoring {
    fn write(self: Box<Self>, world: &mut World, resources: &mut Resources) {
        let oldest_state_frame = self.state_frame_buffer.history_iter(1).next();
        let new_state_frame = self.author_state_inner.call((self.component_type_id, world, resources, self.entity, self.frame));

        if let Some(state_frame) = oldest_state_frame.as_deref() {
            if *state_frame == new_state_frame {
                return;
            }
        }

        self.state_frame_buffer.push(new_state_frame);
    }
}

impl Command for SynchronizedStateConsumption {
    fn write(self: Box<Self>, world: &mut World, resources: &mut Resources) {
        self.consume_state_inner.call((world, resources, self.entity, self.state_frame));
    }
}

impl Synchronized {
    pub fn new<T: Synchronizable>(component_type_id: u8) -> Self {
        trait SynchronizableWrapper: Synchronizable {
            fn author_state(component_type_id: u8, world: &World, resources: &mut Resources, entity: Entity, frame: u32) -> StateFrame {
                let state = world.get::<Self>(entity).unwrap().author_serialized_state(resources);

                StateFrame {
                    entity_id: entity.id(),
                    component_type_id,
                    frame,
                    state
                }
            }

            fn consume_state(world: &mut World, resources: &mut Resources, entity: Entity, state_frame: StateFrame) {
                world.get_mut::<Self>(entity).unwrap().consume_serialized_state(state_frame.state, resources)
            }
        }
        impl<T: Synchronizable> SynchronizableWrapper for T {}
        Self {
            component_type_id,
            author_state_inner: <T as SynchronizableWrapper>::author_state,
            consume_state_inner: <T as SynchronizableWrapper>::consume_state,
            command_frame_buffer: CommandFrameBuffer::default(),
            state_frame_buffer: StateFrameBuffer::default()
        }
    }

    pub fn component_type_id(&self) -> u8 {
        self.component_type_id
    }

    pub fn author_state(&self, commands: &mut Commands, entity: Entity, frame: u32) {
        commands.add_command(SynchronizedStateAuthoring {
            author_state_inner: self.author_state_inner,
            component_type_id: self.component_type_id,
            entity,
            frame,
            state_frame_buffer: self.state_frame_buffer
        });
    }

    pub fn consume_state(&self, commands: &mut Commands, entity: Entity, state_frame: StateFrame) {
        commands.add_command(SynchronizedStateConsumption {
            consume_state_inner: self.consume_state_inner,
            component_type_id: self.component_type_id,
            entity,
            state_frame
        });
    }

    pub fn command_frames(&mut self) -> &mut CommandFrameBuffer {
        &mut self.command_frame_buffer
    }

    pub fn state_frames(&mut self) -> &mut StateFrameBuffer {
        &mut self.state_frame_buffer
    }
}
