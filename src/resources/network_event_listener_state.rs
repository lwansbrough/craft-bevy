use bevy_prototype_networking_laminar::{NetworkEvent};
use bevy::prelude::*;
use crate::events::*;
use crate::models::*;

#[derive(Default)]
pub struct NetworkEventListenerState {
    pub network_events: EventReader<NetworkEvent>,
    pub command_frame_events: EventReader<CommandFrameEvent>,
    pub state_frame_events: EventReader<StateFrameEvent>,
    pub entity_spawn_events: EventReader<EntitySpawnEvent>
}
