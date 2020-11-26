use serde::{Serialize, Deserialize};
use crate::models::*;
use bevy_rapier3d::rapier::dynamics::{RigidBody};

/// A discrete representation of a component's state at a moment in time
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SynchronizedState {
    None,
    RigidBody(RigidBody)
}

impl Default for SynchronizedState {
    fn default() -> Self { SynchronizedState::None }
}
