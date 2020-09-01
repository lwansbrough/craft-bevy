use serde::{Serialize, Deserialize};
use crate::models::*;

/// A discrete representation of a player's inputs at a moment in time
#[derive(Default, Serialize, Deserialize, PartialEq)]
pub struct InputFrame {
    pub frame: u32,
    pub entity_id: u32,
    pub input: SynchronizedInput
}