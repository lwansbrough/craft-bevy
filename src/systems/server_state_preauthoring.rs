// use std::net::SocketAddr;
// use bevy::{
//     prelude::*,
// };
// use bevy_prototype_networking_laminar::{NetworkResource, NetworkDelivery};
// use serde::{Serialize};
// use crate::components::*;
// use crate::models::*;
// use crate::resources::*;

// /// This system sends the latest authoritative state from synchronizable components to clients
// pub fn server_state_preauthoring_system<TComponent: Synchronizable>(
//     commands: &mut Commands,
//     clients: Res<Clients>,
//     sim_time: Res<SimulationTime>,
//     net: Res<NetworkResource>,
//     mut synchronizable_entity_query: Query<(Entity, &mut Synchronized<TComponent>), Changed<TComponent>>,
// ) {
//     for (entity, mut synchronizable) in &mut synchronizable_entity_query.iter_mut() {
//         let state_frames = synchronizable.state_frames();
//         let state_frame = state_frames.history_iter(1).next();

//         commands.add(Synchronized::<TComponent>::author_state_command(entity, sim_time.frame()));
//     }
// }
