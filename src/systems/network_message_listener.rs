use std::net::SocketAddr;
use bevy::{
    prelude::*,
};
use bevy_prototype_networking_laminar::{Connection, NetworkResource, NetworkDelivery, NetworkEvent};
use serde::{Serialize};
use crate::components::*;
use crate::events::*;
use crate::models::*;
use crate::resources::*;

/// Listen for network messages (server + client)
pub fn network_message_listener_system/*<TComponent: 'static + Send + Sync>*/(
    ci: Res<ConnectionInfo>,
    net: Res<NetworkResource>,
    mut state: ResMut<NetworkEventState>,
    network_events: Res<Events<NetworkEvent>>,
    mut command_frame_events: ResMut<Events<CommandFrameEvent>>,
    mut state_frame_events: ResMut<Events<StateFrameEvent>>,
    mut clients: ResMut<Clients>
) {
    for event in state.network_events.iter(&network_events) {
        // println!("Received a NetworkEvent: {:?}", event);
        match event {
            NetworkEvent::Message(conn, msg) => {
                let msg = bincode::deserialize::<NetMessage>(&msg[..]).unwrap();
                match msg {
                    NetMessage::Authorize(token) => handle_authorization(
                        token,
                        *conn,
                        &net,
                        &mut clients
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
    clients: &mut ResMut<Clients>
) {
    // TODO: check auth token
    let user_device_id = 123u128;

    // let _ = net.send(
    //     conn.addr,
    //     &TestbedMessage::Pong.encode()[..],
    //     NetworkDelivery::ReliableSequenced(Some(2)),
    // );

    clients.add(conn, Client::new(user_device_id));
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

    println!("Jump?: {:?}", command_frame.input);

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

    state_frame_events.send(StateFrameEvent {
        state_frame
    });
}