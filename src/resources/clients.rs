use std::collections::{HashMap};
use bevy::prelude::*;
use bevy_prototype_networking_laminar::{Connection};
use crate::models::*;

pub struct Clients {
    connections: HashMap<Connection, u128>,
    clients: HashMap<u128, Client>
}

impl Default for Clients {
    fn default() -> Clients {
        Clients {
            connections: HashMap::new(),
            clients: HashMap::new()
        }
    }
}

impl Clients {
    pub fn get_client_id(&self, connection: Connection) -> Option<&u128> {
        self.connections.get(&connection)
    }

    pub fn add(&mut self, connection: Connection, client: Client) {
        self.connections.insert(connection, client.id());
        self.clients.insert(client.id(), client);
    }
}
