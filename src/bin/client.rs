use std::net::SocketAddr;
use std::time::Duration;

use bevy::prelude::*;
use bevy::app::{ScheduleRunnerSettings};
use bevy_rapier3d::physics::{RapierPhysicsPlugin, RigidBodyHandleComponent};
use bevy_rapier3d::rapier::dynamics::{BodyStatus, RigidBody, RigidBodyBuilder};
use bevy_rapier3d::rapier::geometry::ColliderBuilder;
use bevy_prototype_networking_laminar::{NetworkResource, NetworkingPlugin, NetworkDelivery};
use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};

use craft::components::*;
use craft::events::*;
use craft::models::*;
use craft::resources::*;
use craft::systems::*;

fn main() {
    let client_addr: SocketAddr = "127.0.0.1:12351".parse().expect("The socket address wasn't a valid format");
    let server_addr: SocketAddr = "127.0.0.1:12350".parse().expect("The socket address wasn't a valid format");

    let client = ConnectionInfo::Client {
        id: 123u128,
        addr: client_addr,
        server: server_addr
    };

    App::build()
        .add_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f64(
            1.0 / 60.0,
        )))
        .add_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin)
        .add_plugin(NetworkingPlugin)
        .add_event::<CommandFrameEvent>()
        .add_event::<StateFrameEvent>()
        .add_event::<EntitySpawnEvent>()
        .add_resource(client)
        .add_resource(SimulationTime::new(60))
        .add_resource(WorldGenerator::new(16))
        .init_resource::<NetworkEventListenerState>()
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
        .add_system(network_message_listener_system.system())
        .add_system(client_prediction_system::<RigidBodyHandleComponent>.system())
        .add_startup_system(chunk_loading_system.system())
        .add_plugin(FlyCameraPlugin)
        .run();
}


/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut net: ResMut<NetworkResource>,
    ci: Res<ConnectionInfo>
) {
    net.bind(ci.addr()).expect("We failed to bind to the socket.");
    net.send(
        *ci.server_addr(),
        &bincode::serialize(&NetMessage::None).unwrap(),
        NetworkDelivery::UnreliableUnordered
    );

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
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
        })
        .spawn(Camera3dComponents::default())
        .with(FlyCamera::default());
    
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
        .with(Synchronizable::<RigidBodyHandleComponent>::default())
        .with(Synchronized::new::<RigidBodyHandleComponent>(1));
}
