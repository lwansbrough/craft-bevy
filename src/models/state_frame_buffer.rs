use std::collections::{
    VecDeque,
    vec_deque::{Iter, IterMut}
};
use crate::models::*;

pub struct StateFrameBuffer {
    latest_frame: u32,
    earliest_frame: u32,
    max_entries: u32,
    entries: VecDeque<StateFrame>
}

impl Default for StateFrameBuffer {
    fn default() -> Self {
        Self {
            latest_frame: 0,
            earliest_frame: 0,
            max_entries: 3,
            entries:  VecDeque::<StateFrame>::with_capacity(3)
        }
    }
}

impl StateFrameBuffer {
    pub fn grow(&mut self, size: u32) {
        self.max_entries += size;
    }

    pub fn shrink(&mut self, size: u32) {
        self.max_entries -= size;

        if self.latest_frame > self.max_entries {
            let shrinked = self.latest_frame - self.max_entries;

            for state_frame in (0..shrinked).rev() {
                self.clear_old(state_frame);
            }
        }
    }

    pub fn push(
        &mut self,
        state_frame: StateFrame
    ) {
        self.latest_frame = state_frame.frame;

        if self.earliest_frame == 0 {
            self.earliest_frame = self.latest_frame
        }

        if (self.latest_frame - self.earliest_frame) == self.max_entries {
            let removed_command = self.entries.pop_front();

            if let Some(removed_command) = removed_command {
                if self.entries.len() == 0 {
                    return;
                }

                self.clear_old(removed_command.frame);

                if let Some(earliest_command) = self.entries.front() {
                    self.earliest_frame = earliest_command.frame;
                }
            }
        }

        self.entries.push_back(state_frame)
    }

    pub fn iter(&self) -> Iter<StateFrame> {
        self.entries.iter()
    }

    pub fn history_iter(&mut self, history_size: u32) -> StateFrameBufferIterMut<'_> {
        let mut history_size = history_size;
        if history_size > self.earliest_frame {
            history_size = self.latest_frame;
        }

        StateFrameBufferIterMut {
            items: self.entries.iter_mut(),
            earliest_frame: self.latest_frame - history_size
        }
    }

    fn clear_old(&mut self, frame: u32) {
        while let Some(state) = self.entries.get(0) {
            if frame == state.frame {
                self.entries.pop_front().expect("Should have state frame");
            } else {
                break;
            }
        }
    }
}

pub struct StateFrameBufferIterMut<'a> {
    items: IterMut<'a, StateFrame>,
    earliest_frame: u32,
}

impl<'a> Iterator for StateFrameBufferIterMut<'a> {
    type Item = &'a mut StateFrame;

    /// Returns `Some` when there is an item in our cache matching the `expected_index`.
    /// Returns `None` if there are no times matching our `expected` index.
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        if let Some(state) = self.items.next() {
            if state.frame >= self.earliest_frame /* && !command.is_sent */ {
                return Some(state);
            }
        }

        return None;
    }
}
