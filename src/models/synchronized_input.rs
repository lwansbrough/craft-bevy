use serde::{Serialize, Deserialize};
use crate::models::*;

/// A discrete representation of a client's input at a moment in time
#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum SynchronizedInput {
    InputCommand(InputCommand),
    None
}

impl Default for SynchronizedInput {
    fn default() -> Self { SynchronizedInput::None }
}
