use serde::{Serialize, Deserialize};
use crate::models::*;

/// A representation of an entity that is spawning
#[derive(Default, Serialize, Deserialize)]
pub struct EntitySpawn {
    pub entity_id: u32
}
