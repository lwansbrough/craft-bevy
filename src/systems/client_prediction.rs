use std::net::SocketAddr;
use bevy::{
    prelude::*,
};
use bevy_prototype_networking_laminar::{NetworkResource, NetworkDelivery};
use serde::{Serialize};
use crate::components::*;
use crate::models::*;
use crate::resources::*;

/// This system prints out all mouse events as they come in
pub fn client_prediction_system<TComponent: 'static + Send + Sync>(
    ci: Res<ConnectionInfo>,
    sim_time: Res<SimulationTime>,
    net: Res<NetworkResource>,
    mut synchronizable_entity_query: Query<(&ServerEntity, &mut Synchronizable<TComponent>)>,
) {
    for (server_entity, mut synchronizable) in &mut synchronizable_entity_query.iter_mut() {
        let mut command_frames = synchronizable.command_frames();
        let command_frame = command_frames.history_iter(3).next();

        if let Some(command_frame) = command_frame {
            let frame: u32 = command_frame.frame;

            let command_frame_message = NetMessage::CommandFrame(*command_frame);

            let bytes: Vec<u8> = bincode::serialize(&command_frame_message).unwrap();
            println!("Entity {} sending message for sim frame {}. Data: {:?}", server_entity.id, frame, &bytes);
            

            net.send(*ci.server_addr(), &bytes, NetworkDelivery::UnreliableUnordered);
        }
    }
}
