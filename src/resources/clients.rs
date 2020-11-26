use std::collections::hash_map::Iter;
use std::collections::hash_map::IterMut;
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

    pub fn iter(&self) -> ClientsIter<'_> {
        ClientsIter {
            items: self.clients.iter()
        }
    }
}

pub struct ClientsIter<'a> {
    items: Iter<'a, u128, Client>
}

impl<'a> Iterator for ClientsIter<'a> {
    type Item = &'a Client;

    /// Returns `Some` when there is an item in our cache matching the `expected_index`.
    /// Returns `None` if there are no times matching our `expected` index.
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        if let Some(client) = self.items.next() {
            return Some(client.1)
        }

        return None;
    }
}
