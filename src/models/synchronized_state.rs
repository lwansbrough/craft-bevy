use serde::{Serialize, Deserialize};
use crate::models::*;
use bevy_rapier3d::rapier::dynamics::{RigidBody};

/// A discrete representation of a component's state at a moment in time
#[derive(Debug, Serialize, Deserialize)]
pub enum SynchronizedState {
    RigidBody(RigidBody),
    None
}

impl Default for SynchronizedState {
    fn default() -> Self { SynchronizedState::None }
}
