use serde::{Serialize, Deserialize};
use std::marker::{PhantomData};

use crate::models::*;

pub struct Synchronizable<TComponent> {
    command_frame_buffer: CommandFrameBuffer,
    state_frame_buffer: StateFrameBuffer,
    _m: PhantomData<TComponent>
}

impl<TComponent> Default for Synchronizable<TComponent> where TComponent: 'static + Send + Sync {
    fn default() -> Self {
        Self {
            command_frame_buffer: CommandFrameBuffer::default(),
            state_frame_buffer: StateFrameBuffer::default(),
            _m: PhantomData
        }
    }
}

impl<TComponent> Synchronizable<TComponent> where TComponent: 'static + Send + Sync {
    pub fn command_frames(&mut self) -> &mut CommandFrameBuffer {
        &mut self.command_frame_buffer
    }

    pub fn state_frames(&mut self) -> &mut StateFrameBuffer {
        &mut self.state_frame_buffer
    }
}
