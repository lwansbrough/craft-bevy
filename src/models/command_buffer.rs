use crate::models::InputCommand;
use std::collections::VecDeque;

pub struct CommandBuffer {
    pub commands: VecDeque<InputCommand>
}

impl Default for CommandBuffer {
    fn default() -> Self {
        Self {
            commands: VecDeque::with_capacity(4)
        }
    }
}
