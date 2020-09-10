use crate::models::InputCommand;
use std::collections::VecDeque;

pub struct InputCommandBuffer {
    pub inputs: VecDeque<InputCommand>
}

impl Default for InputCommandBuffer {
    fn default() -> Self {
        Self {
            inputs: VecDeque::with_capacity(4)
        }
    }
}
