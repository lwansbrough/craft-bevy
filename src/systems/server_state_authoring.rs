use std::net::SocketAddr;
use bevy::{
    prelude::*,
};
use bevy_prototype_networking_laminar::{NetworkResource, NetworkDelivery};
use serde::{Serialize};
use crate::components::*;
use crate::models::*;
use crate::resources::*;

/// This system sends the latest authoritative state from synchronizable components to clients
pub fn server_state_authoring_system(
    resources: &Resources,
    clients: Res<Clients>,
    sim_time: Res<SimulationTime>,
    net: Res<NetworkResource>,
    world: &World,
    mut synchronizable_entity_query: Query<(Entity, &mut Synchronized)>
) {
    for (entity, mut synchronized) in &mut synchronizable_entity_query.iter_mut() {
        
        let state_frame = synchronized.state_frames().history_iter(1).next();

        if let Some(state_frame) = state_frame {
            let frame: u32 = state_frame.frame;

            let state_frame_message = NetMessage::AuthoritativeStateFrame(state_frame.clone());

            let bytes: Vec<u8> = bincode::serialize(&state_frame_message).unwrap();
            println!("Entity {} sending authoratative state for sim frame {}. Data: {:?}", entity.id(), frame, &bytes);
            
            for client in clients.iter() {
                net.send(client.connection().addr, &bytes, NetworkDelivery::UnreliableUnordered).unwrap();
            }
        }
    }
}
