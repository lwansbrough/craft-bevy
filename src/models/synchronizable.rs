use bevy::ecs::{Component, Resources};

/// A component that is able to be synchronized between the client and server
pub trait Synchronizable: Component + Sized {
    fn author_serialized_state(&self, resources: &mut Resources) -> Vec<u8>;
    fn consume_serialized_state(&mut self, state: Vec<u8>, resources: &mut Resources);
}
