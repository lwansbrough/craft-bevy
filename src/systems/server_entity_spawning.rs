use std::net::SocketAddr;
use bevy::{
    prelude::*,
};
use bevy_prototype_networking_laminar::{NetworkResource, NetworkDelivery};
use serde::{Serialize};
use crate::components::*;
use crate::models::*;
use crate::resources::*;

/// This system sends information about newly spawned synchronizable entities to already connected clients
pub fn server_entity_spawning_for_connected_clients(
    mut commands: Commands,
    clients: Res<Clients>,
    net: Res<NetworkResource>,
    added_synchronizable_entities: Query<(&Entity, Added<Synchronize>)>,
) {
    for (entity, _) in added_synchronizable_entities.iter() {
        println!("Server spawning entity {}", entity.id());

        for client in clients.iter() {
            net.send(
                client.connection().addr,
                &bincode::serialize(&NetMessage::EntitySpawn(EntitySpawn {
                    entity_id: entity.id()
                })).unwrap(),
                NetworkDelivery::ReliableOrdered(Some(2))
            );
        }
    }
}

/// This system sends information about existing synchronizable entities to newly connected clients
pub fn server_entity_spawning_for_new_clients(
    mut commands: Commands,
    changed_clients: ChangedRes<Clients>,
    net: Res<NetworkResource>,
    synchronizable_entities: Query<(&Entity, &Synchronize)>,
) {
    for (entity, _) in synchronizable_entities.iter() {
        println!("Server synchronizing entity {}", entity.id());

        for client in changed_clients.iter() {
            net.send(
                client.connection().addr,
                &bincode::serialize(&NetMessage::EntitySpawn(EntitySpawn {
                    entity_id: entity.id()
                })).unwrap(),
                NetworkDelivery::ReliableOrdered(Some(2))
            );
        }
    }
}
