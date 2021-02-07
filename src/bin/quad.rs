use std::net::SocketAddr;
use std::time::Duration;

use bevy::{core::AsBytes, prelude::*, render::{mesh::Indices, pipeline::PrimitiveTopology, renderer::RenderResourceBinding}};
use bevy::{render::renderer::{RenderResourceContext, RenderResourceBindings, BufferUsage, BufferInfo}, app::{ScheduleRunnerSettings}};
use bevy_rapier3d::physics::{RapierPhysicsPlugin, RigidBodyHandleComponent};
use bevy_rapier3d::rapier::dynamics::{BodyStatus, RigidBody, RigidBodyBuilder};
use bevy_rapier3d::rapier::geometry::ColliderBuilder;
use bevy_prototype_networking_laminar::{NetworkResource, NetworkingPlugin, NetworkDelivery};
use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};
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
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
        // .add_plugin(bevy::diagnostic::PrintDiagnosticsPlugin::default())
        .add_plugin(VoxelRenderPlugin)
        .add_plugin(FlyCameraPlugin)
        .add_asset::<VoxelVolume>()
        .add_resource(WorldGenerator::new(32))
        .add_resource(WorldData::new())
        .init_resource::<WindowResizeEventListenerState>()
        .add_startup_system(setup.system())
        .add_system(window_resolution_system.system())
        .add_system(chunk_loading_system.system())
        .run();
}

/// set up a simple 3D scene
fn setup(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials_standard: ResMut<Assets<StandardMaterial>>,
    mut voxel_volumes: ResMut<Assets<VoxelVolume>>,
) {
    
    commands
        // Fullscreen quad
        .spawn(PbrBundle {
            material: materials_standard.add(bevy::render::color::Color::GREEN.into()),
            transform: Transform::from_translation(Vec3::new(-3.0, 0.0, 0.0)),
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            ..Default::default()
        })
        .spawn(PbrBundle {
            material: materials_standard.add(bevy::render::color::Color::RED.into()),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
            ..Default::default()
        })
        .with(LocalPlayerBody {})
        .spawn(Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 20.0)),
            ..Default::default()
        })
        .with(FlyCamera::default());
}
