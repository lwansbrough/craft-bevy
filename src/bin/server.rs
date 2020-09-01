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

    // add entities to the world
    commands
        // plane
        .spawn(PbrComponents {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 10.0 })),
            ..Default::default()
        })
        .with(RigidBodyBuilder::new(BodyStatus::Static))
        .with(ColliderBuilder::cuboid(10.0, 0.0, 10.0))
        // cube
        .spawn(PbrComponents {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            ..Default::default()
        })
        .with(RigidBodyBuilder::new(BodyStatus::Static).translation(0.0, 1.0, 0.0))
        .with(ColliderBuilder::cuboid(1.0, 1.0, 1.0))
        // light
        .spawn(LightComponents {
            translation: Translation::new(4.0, 8.0, 4.0),
            ..Default::default()
        })
        .spawn(Camera3dComponents {
            transform: Transform::new_sync_disabled(Mat4::face_toward(
                Vec3::new(-3.0, 5.0, 20.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
            )),
            ..Default::default()
        });
}
