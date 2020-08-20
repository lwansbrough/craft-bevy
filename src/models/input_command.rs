use serde::{Serialize, Deserialize};

/// A discrete representation of a player's inputs at a moment in time
#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct InputCommand {
    pub frame: u32,
    pub forward: bool,
    pub backward: bool,
    pub left: bool,
    pub right: bool,
    pub jump: bool
}
