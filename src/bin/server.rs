use std::net::SocketAddr;
use std::time::Duration;

use bevy::prelude::*;
use bevy::app::{ScheduleRunnerSettings};
use bevy_rapier3d::physics::{RapierPhysicsPlugin, RigidBodyHandleComponent};
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
        .add_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f64(
            1.0 / 60.0,
        )))
        .add_plugins(MinimalPlugins)
        .add_plugin(RapierPhysicsPlugin)
        .add_plugin(NetworkingPlugin)
        .add_event::<CommandFrameEvent>()
        .add_event::<StateFrameEvent>()
        .add_event::<EntitySpawnEvent>()
        .add_resource(server)
        .add_resource(SimulationTime::new(60))
        .init_resource::<NetworkEventListenerState>()
        .init_resource::<Clients>()
        .add_startup_system(setup.system())
        .add_system_to_stage(stage::PRE_UPDATE, network_message_listener_system.system())
        .add_system_to_stage(stage::PRE_UPDATE, server_player_movement_system.system())
        .add_stage_after(stage::POST_UPDATE, "pre_synchronize")
        .add_stage_after("pre_synchronize", "synchronize")
        .add_stage_after("synchronize", "post_synchronize")
        .add_system_to_stage("pre_synchronize", server_state_preauthoring_system::<RigidBodyHandleComponent>.system())
        .add_system_to_stage("pre_synchronize", server_entity_spawning_for_connected_clients.system())
        .add_system_to_stage("pre_synchronize", server_entity_spawning_for_new_clients.system())
        .add_system_to_stage("synchronize", server_state_authoring_system::<RigidBodyHandleComponent>.system())
        .run();
}

/// set up a simple 3D scene
fn setup(
    commands: &mut Commands,
    mut net: ResMut<NetworkResource>,
    ci: Res<ConnectionInfo>
) {
    net.bind(ci.addr()).expect("We failed to bind to the socket.");
}
