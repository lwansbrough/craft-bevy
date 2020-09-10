use bevy::prelude::*;
use bevy_rapier3d::physics::{RapierPhysicsPlugin, RigidBodyHandleComponent};
use bevy_rapier3d::rapier::dynamics::{BodyStatus, RigidBody, RigidBodyBuilder};
use bevy_rapier3d::rapier::geometry::ColliderBuilder;
use bevy_prototype_networking_laminar::{NetworkResource, NetworkingPlugin};

use craft::components::*;
use craft::models::*;
use craft::systems::*;

fn main() {
    App::build()
        .add_default_plugins()
        .add_plugin(RapierPhysicsPlugin)
        .add_plugin(NetworkingPlugin)
        .add_event::<CommandFrameEvent>()
        .add_resource(SimulationTime::new(60))
        .init_resource::<NetworkEventState>()
        .init_resource::<Clients>()
        .add_startup_system(setup.system())
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut net: ResMut<NetworkResource>
) {
    net.bind("127.0.0.1:12350").unwrap();
}
