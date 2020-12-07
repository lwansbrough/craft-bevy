use serde::{Serialize, Deserialize};
use crate::models::*;

/// A discrete representation of a component's state at a moment in time
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct StateFrame {
    pub frame: u32,
    pub entity_id: u32,
    pub component_type_id: i8,
    pub state: 
}
