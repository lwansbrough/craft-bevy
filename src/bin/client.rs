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
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_default_plugins()
        .add_plugin(RapierPhysicsPlugin)
        .add_plugin(NetworkingPlugin)
        .add_event::<StateFrameEvent>()
        .add_resource(SimulationTime::new(60))
        .init_resource::<NetworkEventState>()
        .init_resource::<Clients>()
        .init_resource::<CommandAccumulatorState>()
        .init_resource::<LocalPlayerCameraState>()
        .init_resource::<LocalPlayerMovementState>()
        .init_resource::<PlayerMovementState>()
        .add_startup_system(setup.system())
        .add_system(simulation_time_system.system())
        .add_system(command_accumulator_system.system())
        .add_system(local_player_camera_system.system())
        .add_system(local_player_movement_system.system())
        .add_system(player_movement_system.system())
        .add_system(client_prediction_system::<RigidBodyHandleComponent>.system())
        .run();
}


/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut net: ResMut<NetworkResource>
) {
    // add entities to the world
    commands
        // plane
        .spawn(PbrComponents {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 10.0 })),
            material: materials.add(Color::rgb(0.1, 0.2, 0.1).into()),
            ..Default::default()
        })
        .with(RigidBodyBuilder::new(BodyStatus::Static))
        .with(ColliderBuilder::cuboid(10.0, 0.0, 10.0))
        // cube
        .spawn(PbrComponents {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.5, 0.4, 0.3).into()),
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
    
    // player
    commands
        .spawn(PbrComponents {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
            ..Default::default()
        })
        .with(LocalPlayer)
        .with(LocalPlayerBody)
        .with_children(|player| {
            player.spawn(PbrComponents {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
                material: materials.add(Color::rgb(0.0, 0.0, 1.0).into()),
                translation: Translation::new(0.0, 1.0, 0.0),
                transform: Transform::new(Mat4::from_quat(Quat::from_xyzw(0.0, 1.0, 0.0, 0.0))),
                ..Default::default()
            })
            .with(LocalPlayerHead)
            .with_children(|head| {
            // camera
                // head.spawn(Camera3dComponents {
                //     transform: Transform::new_sync_disabled(Mat4::identity()),
                //     ..Default::default()
                // });
            });
        })
        .with(RigidBodyBuilder::new(BodyStatus::Dynamic).translation(5.0, 2.0, 5.0))
        .with(ColliderBuilder::cuboid(1.0, 1.0, 1.0))
        .with(Synchronizable::<RigidBodyHandleComponent>::default());
}
