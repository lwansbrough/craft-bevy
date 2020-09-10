use std:collections::{HashMap, Vec};
use bevy::prelude::*;
use bevy_prototype_networking_laminar::{Connection};
use crate::models::*;

pub struct Clients {
    connections: HashMap<Connection, u128>,
    clients: HashMap<u128, Client>
}

impl Default for Clients {
    pub fn default() -> Clients {
        Clients {
            connections: HashMap::new(),
            clients: HashMap::new()
        }
    }

    pub fn get_client_id(&self, connection: Connection) -> u128 {
        self.connections.get(&connection)
    }

    pub fn add(&mut self, connection: Connection, client: Client) {
        self.connections.add(&connection, client.id);
        self.clients.add(client.id, &client);
    }
}
