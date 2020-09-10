use std::collections::{
    VecDeque,
    vec_deque::{Iter, IterMut}
};
use crate::models::*;

pub struct CommandFrameBuffer {
    latest_frame: u32,
    earliest_frame: u32,
    max_commands: u32,
    commands: VecDeque<CommandFrame>
}

impl Default for CommandFrameBuffer {
    fn default() -> Self {
        Self {
            latest_frame: 0,
            earliest_frame: 0,
            max_commands: 3,
            commands:  VecDeque::<CommandFrame>::with_capacity(3)
        }
    }
}

impl CommandFrameBuffer {
    pub fn grow(&mut self, size: u32) {
        self.max_commands += size;
    }

    pub fn shrink(&mut self, size: u32) {
        self.max_commands -= size;

        if self.latest_frame > self.max_commands {
            let shrinked = self.latest_frame - self.max_commands;

            for command_frame in (0..shrinked).rev() {
                self.clear_old(command_frame);
            }
        }
    }

    pub fn push(
        &mut self,
        entity_id: u32,
        frame: u32,
        input: SynchronizedInput
    ) {
        let command_frame = CommandFrame {
            entity_id,
            frame,
            input
        };

        self.latest_frame = frame;

        if self.earliest_frame == 0 {
            self.earliest_frame = self.latest_frame
        }

        if (self.latest_frame - self.earliest_frame) == self.max_commands {
            let removed_command = self.commands.pop_front();

            if let Some(removed_command) = removed_command {
                if self.commands.len() == 0 {
                    return;
                }

                self.clear_old(removed_command.frame);

                if let Some(earliest_command) = self.commands.front() {
                    self.earliest_frame = earliest_command.frame;
                }
            }
        }

        self.commands.push_back(command_frame)
    }

    pub fn iter(&self) -> Iter<CommandFrame> {
        self.commands.iter()
    }

    pub fn history_iter(&mut self, history_size: u32) -> CommandFrameBufferIterMut<'_> {
        let mut history_size = history_size;
        if history_size > self.earliest_frame {
            history_size = self.latest_frame;
        }

        CommandFrameBufferIterMut {
            items: self.commands.iter_mut(),
            earliest_frame: self.latest_frame - history_size
        }
    }

    fn clear_old(&mut self, frame: u32) {
        while let Some(command) = self.commands.get(0) {
            if frame == command.frame {
                self.commands.pop_front().expect("Should have command frame");
            } else {
                break;
            }
        }
    }
}

pub struct CommandFrameBufferIterMut<'a> {
    items: IterMut<'a, CommandFrame>,
    earliest_frame: u32,
}

impl<'a> Iterator for CommandFrameBufferIterMut<'a> {
    type Item = &'a mut CommandFrame;

    /// Returns `Some` when there is an item in our cache matching the `expected_index`.
    /// Returns `None` if there are no times matching our `expected` index.
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        if let Some(command) = self.items.next() {
            if command.frame >= self.earliest_frame /* && !command.is_sent */ {
                return Some(command);
            }
        }

        return None;
    }
}
