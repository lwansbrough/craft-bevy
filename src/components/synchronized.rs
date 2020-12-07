use bevy::ecs::{Entity, World};
use serde::{Serialize, Deserialize};
use crate::models::{CommandFrameBuffer, StateFrameBuffer, StateFrame};

pub struct Synchronized {
    author_state_inner: fn(&World, Entity) -> StateFrame,
    consume_state_inner: fn(&World, Entity, StateFrame),
    command_frame_buffer: CommandFrameBuffer,
    state_frame_buffer: StateFrameBuffer,
}

impl Synchronized {
    pub fn new<T: Serialize>(component_type_id: i8) -> Self {
        trait Synchronizable: Serialize {
            fn author_state(world: &World, entity: Entity) -> StateFrame {
                Self::serialize(world.get(entity).unwrap());

                StateFrame {
                    entity.id(),
                    component_type_id,
                    frame,
                    state
                }
            }

            fn consume_state(world: &World, entity: Entity, state_frame: StateFrame) {
                
            }
        }
        impl<T: Serialize + ?Sized> Synchronizable for T {}
        Self {
            author_state_inner: <T as Synchronizable>::author_state,
            consume_state_inner: <T as Synchronizable>::consume_state,
            command_frame_buffer: CommandFrameBuffer::default(),
            state_frame_buffer: StateFrameBuffer::default()
        }
    }

    pub fn author_state(&mut self, world: &World, entity: Entity) -> Option<StateFrame> {
        let oldest_state_frame = self.state_frame_buffer.history_iter(1).next();
        let new_state_frame = self.author_state_inner(world, entity);

        if let Some(state_frame) = oldest_state_frame {
            if state_frame == new_state_frame {
                return None
            }
        }

        self.state_frame_buffer.push(new_state_frame);

        Some(new_state_frame) // maybe should be oldest_state_frame?
    }

    pub fn consume_state(&self, world: &World, entity: Entity, state_frame: StateFrame) {
        self.consume_state_inner(world, entity)
    }

    pub fn command_frames(&mut self) -> &mut CommandFrameBuffer {
        &mut self.command_frame_buffer
    }

    pub fn state_frames(&mut self) -> &mut StateFrameBuffer {
        &mut self.state_frame_buffer
    }
}
