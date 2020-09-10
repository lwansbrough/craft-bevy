use bevy_prototype_networking_laminar::{NetworkEvent};
use bevy::prelude::*;
use crate::models::*;

#[derive(Default)]
pub struct NetworkEventState {
    pub network_events: EventReader<NetworkEvent>,
}
