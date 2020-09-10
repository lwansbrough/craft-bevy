use std::net::SocketAddr;

use bevy::prelude::*;
use bevy_rapier3d::physics::{RapierPhysicsPlugin, RigidBodyHandleComponent};
use bevy_rapier3d::rapier::dynamics::{BodyStatus, RigidBody, RigidBodyBuilder};
use bevy_rapier3d::rapier::geometry::ColliderBuilder;
use bevy_prototype_networking_laminar::{NetworkResource, NetworkingPlugin};

use craft::components::*;
use craft::events::*;
use craft::models::*;
use craft::resources::*;
use craft::systems::*;

fn main() {
    let addr: SocketAddr = "127.0.0.1:12350".parse().expect("The socket address wasn't a valid format");
    let server = ConnectionInfo::Server { addr };

    App::build()
        .add_default_plugins()
        .add_plugin(RapierPhysicsPlugin)
        .add_plugin(NetworkingPlugin)
        .add_event::<CommandFrameEvent>()
        .add_event::<StateFrameEvent>()
        .add_resource(server)
        .add_resource(SimulationTime::new(60))
        .init_resource::<NetworkEventState>()
        .init_resource::<Clients>()
        .add_startup_system(setup.system())
        .add_system(network_message_listener_system.system())
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut net: ResMut<NetworkResource>,
    ci: Res<ConnectionInfo>
) {
    net.bind(ci.addr()).expect("We failed to bind to the socket.");
}
