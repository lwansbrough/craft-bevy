use std::net::SocketAddr;
use bevy::{
    prelude::*,
};
use bevy_prototype_networking_laminar::{Connection, NetworkResource, NetworkDelivery, NetworkEvent};
use bevy_rapier3d::{rapier::dynamics::RigidBody, physics::RigidBodyHandleComponent};
use serde::{Serialize};
use crate::components::*;
use crate::events::*;
use crate::models::*;
use crate::resources::*;

/// Listen for network messages (server + client)
pub fn network_message_listener_system/*<TComponent: 'static + Send + Sync>*/(
    mut commands: Commands,
    ci: Res<ConnectionInfo>,
    net: Res<NetworkResource>,
    mut state: ResMut<NetworkEventListenerState>,
    network_events: Res<Events<NetworkEvent>>,
    mut command_frame_events: ResMut<Events<CommandFrameEvent>>,
    mut state_frame_events: ResMut<Events<StateFrameEvent>>,
    mut entity_spawn_events: ResMut<Events<EntitySpawnEvent>>,
    mut clients: ResMut<Clients>
) {
    for event in state.network_events.iter(&network_events) {
        println!("Received a NetworkEvent: {:?}", event);
        match event {
            NetworkEvent::Connected(conn) => {
                if ci.is_client() {
                    net.send(
                        *ci.server_addr(),
                        &bincode::serialize(&NetMessage::Authorize(String::from("test"))).unwrap(),
                        NetworkDelivery::ReliableOrdered(Some(2))
                    );
                }

                if ci.is_server() {
                    net.send(
                        conn.addr,
                        &bincode::serialize(&NetMessage::None).unwrap(),
                        NetworkDelivery::UnreliableUnordered
                    );
                }
            },
            NetworkEvent::Message(conn, msg) => {
                let msg = bincode::deserialize::<NetMessage>(&msg[..]).unwrap();
                match msg {
                    NetMessage::Authorize(token) => handle_authorization(
                        token,
                        *conn,
                        &net,
                        &mut clients,
                        &mut commands
                    ),
                    NetMessage::CommandFrame(command_frame) => handle_command_frame_event(
                        command_frame,
                        *conn,
                        &ci,
                        &mut command_frame_events,
                        &mut clients
                    ),
                    NetMessage::AuthoritativeStateFrame(state_frame) => handle_state_frame_event(
                        state_frame,
                        *conn,
                        &ci,
                        &mut state_frame_events
                    ),
                    NetMessage::EntitySpawn(entity_spawn) => handle_entity_spawn_event(
                        entity_spawn,
                        *conn,
                        &ci,
                        &mut entity_spawn_events
                    ),
                    _ => {}
                }
            },
            _ => {}
        }
    }
}

fn handle_authorization(
    token: String,
    conn: Connection,
    net: &Res<NetworkResource>,
    clients: &mut ResMut<Clients>,
    mut commands: &mut Commands
) {
    // TODO: check auth token
    let user_device_id = 123u128;

    println!("Client authorized with token {}", token);

    clients.add(conn, Client::new(user_device_id, conn));

    commands.spawn((Synchronized::new::<RigidBodyHandleComponent>(1), LocalPlayer, LocalPlayerBody));
}

fn handle_command_frame_event(
    command_frame: CommandFrame,
    conn: Connection,
    ci: &Res<ConnectionInfo>,
    command_frame_events: &mut ResMut<Events<CommandFrameEvent>>,
    clients: &mut ResMut<Clients>
) {
    // Only handle command frames on the server
    if !ci.is_server() {
        return;
    }

    println!("{:?}", command_frame.input);

    if let Some(client_id) = clients.get_client_id(conn) {
        command_frame_events.send(CommandFrameEvent {
            from: *client_id,
            command_frame
        });
    }
}

fn handle_state_frame_event(
    state_frame: StateFrame,
    conn: Connection,
    ci: &Res<ConnectionInfo>,
    state_frame_events: &mut ResMut<Events<StateFrameEvent>>
) {
    // Only handle authoritative state frames on the client
    if !ci.is_client() {
        return;
    }

    println!("{:?}", state_frame.state);

    state_frame_events.send(StateFrameEvent {
        state_frame
    });
}

fn handle_entity_spawn_event(
    spawn: EntitySpawn,
    conn: Connection,
    ci: &Res<ConnectionInfo>,
    entity_spawn_events: &mut ResMut<Events<EntitySpawnEvent>>
) {
    // Only handle entity spawn events on the client
    if !ci.is_client() {
        return;
    }

    entity_spawn_events.send(EntitySpawnEvent {
        spawn
    });
}