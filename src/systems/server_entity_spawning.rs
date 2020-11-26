use std::net::SocketAddr;
use bevy::{
    prelude::*,
};
use bevy_prototype_networking_laminar::{NetworkResource, NetworkDelivery};
use serde::{Serialize};
use crate::components::*;
use crate::models::*;
use crate::resources::*;

/// This system sends information about spawned entities to clients
pub fn server_entity_spawning(
    mut commands: Commands,
    clients: Res<Clients>,
    sim_time: Res<SimulationTime>,
    net: Res<NetworkResource>,
    awaiting_spawn_query: Query<(&Entity, &AwaitingSpawn)>,
) {
    for (entity, _) in awaiting_spawn_query.iter() {
        println!("Server spawning entity {}", entity.id());

        // TODO: Figure out how to send current state of all synchronizable components as well
        
        for client in clients.iter() {
            net.send(
                client.connection().addr,
                &bincode::serialize(&NetMessage::EntitySpawn(EntitySpawn {
                    entity_id: entity.id()
                })).unwrap(),
                NetworkDelivery::ReliableOrdered(Some(2))
            );
        }

        commands.remove_one::<AwaitingSpawn>(*entity);
    }
}
