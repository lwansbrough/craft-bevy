use serde::{Serialize, Deserialize};

/// A representation of a connected player or device
#[derive(Default)]
pub struct Client {
    id: u128
}

impl Client {
    pub fn id(&self) -> u128 {
        self.id
    }

    pub fn is_authenticated(&self) {
        true
    }
}
