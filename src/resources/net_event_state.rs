use bevy_prototype_networking_laminar::{
    NetworkEvent
};
use bevy::prelude::*;
use crate::models::*;

#[derive(Default)]
struct NetworkEventState {
    network_events: EventReader<NetworkEvent>,
}
