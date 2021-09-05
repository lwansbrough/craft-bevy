use std::net::SocketAddr;
use std::time::Duration;

use bevy::{PipelinedDefaultPlugins, prelude::*};
use bevy::{render::{camera::{Camera, CameraProjection}, mesh::Indices, pipeline::PrimitiveTopology, renderer::RenderResourceBinding}, window::WindowId};
use bevy::{render::renderer::{RenderResourceContext, RenderResourceBindings, BufferUsage, BufferInfo}, app::{ScheduleRunnerSettings}};
use bevy_rapier3d::physics::{RapierPhysicsPlugin, RigidBodyHandleComponent};
use bevy_rapier3d::rapier::dynamics::{BodyStatus, RigidBody, RigidBodyBuilder};
use bevy_rapier3d::rapier::geometry::ColliderBuilder;
// use bevy_prototype_networking_laminar::{NetworkResource, NetworkingPlugin, NetworkDelivery};
// use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};
use noise::{
    *,
    utils::*
};
use craft::utilities::Gradient;

use craft::components::*;
use craft::events::*;
use craft::models::*;
use craft::resources::*;
use craft::systems::*;
use craft::render::*;
use craft::render::VoxelRenderPlugin;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            vsync: false,
            ..Default::default()
        })
        .add_plugins(PipelinedDefaultPlugins)
        .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
        // .add_plugin(bevy::diagnostic::PrintDiagnosticsPlugin::default())
        .init_resource::<WindowResizeEventListenerState>()
        // .init_resource::<PlayerFocus>()
        .add_plugin(VoxelRenderPlugin)
        // .add_plugin(FlyCameraPlugin)
        .add_asset::<VoxelVolume>()
        // .insert_resource(WorldGenerator::new(32))
        // .insert_resource(WorldData::new())
        // .add_system_to_stage(
        //     stage::POST_UPDATE, // We want this system to run after ray casting has been computed
        //     player_focus.system(), // Update the debug cursor location
        // )
        .add_startup_system(setup.system())
        .add_system(window_resolution_system.system())
        // .add_startup_system(chunk_loading_system.system())
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<bevy::render2::mesh::Mesh>>,
    mut materials_standard: ResMut<Assets<bevy::pbr2::StandardMaterial>>,
    mut voxel_volumes: ResMut<Assets<VoxelVolume>>,
) {
    commands
        .spawn()
        .insert_bundle(VoxelBundle {
            transform: Transform {
                translation: Vec3::ZERO,
                rotation: Quat::IDENTITY,
                scale: Vec3::ONE
            },
            ..Default::default()
        });

    commands
        .spawn()
        // .insert_bundle(VoxelBundle {
        //     transform: Transform {
        //         translation: Vec3::ZERO,
        //         rotation: Quat::IDENTITY,
        //         scale: Vec3::ONE
        //     },
        //     ..Default::default()
        // })
        // .insert(LocalPlayerBody {})
        .insert_bundle(bevy::pbr2::PbrBundle {
            mesh: meshes.add(bevy::render2::mesh::Mesh::from(bevy::render2::mesh::shape::Cube { size: 3.0 })),
            transform: Transform::from_translation(Vec3::ZERO),
            material: materials_standard.add(bevy::pbr2::StandardMaterial {
                base_color: bevy::render2::color::Color::PINK,
                ..Default::default()
            }),
            ..Default::default()
        });
    
    commands
        .spawn()
        .insert_bundle(bevy::render2::camera::PerspectiveCameraBundle {
            transform: {
                let transform = Transform::from_translation(Vec3::new(4.0, 3.0, 12.0));
                transform.looking_at(Vec3::ZERO, Vec3::Y);
                transform
            },
            ..Default::default()
        });
        // .insert(FlyCamera::default());
}
