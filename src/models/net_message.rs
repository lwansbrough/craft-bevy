use serde::{Serialize, Deserialize};
use crate::models::*;

/// A discrete representation of a component's state at a moment in time
#[derive(Serialize, Deserialize)]
pub enum NetMessage {
    None,
    Error(String, String),
    Authorize(String),
    CommandFrame(CommandFrame),
    AuthoritativeStateFrame(StateFrame)
}

impl Default for NetMessage {
    fn default() -> Self { NetMessage::None }
}
