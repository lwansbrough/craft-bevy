use std::net::SocketAddr;
use std::time::Duration;

use bevy::{PipelinedDefaultPlugins, prelude::*};
use bevy::{prelude::*, render::{camera::{Camera, CameraProjection}, mesh::Indices, pipeline::PrimitiveTopology, renderer::RenderResourceBinding}, window::WindowId};
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
        .insert_resource(WorldGenerator::new(32))
        .insert_resource(WorldData::new())
        // .add_system_to_stage(
        //     stage::POST_UPDATE, // We want this system to run after ray casting has been computed
        //     player_focus.system(), // Update the debug cursor location
        // )
        .add_startup_system(setup.system())
        .add_system(window_resolution_system.system())
        .add_startup_system(chunk_loading_system.system())
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials_standard: ResMut<Assets<StandardMaterial>>,
    mut voxel_volumes: ResMut<Assets<VoxelVolume>>,
) {    
    commands
        .spawn()
        .insert_bundle(PbrBundle {
            material: materials_standard.add(bevy::render::color::Color::GREEN.into()),
            transform: Transform::from_translation(Vec3::new(-3.0, 0.0, 0.0)),
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            ..Default::default()
        })
        .insert(LocalPlayerBody {})
        .insert_bundle(PerspectiveCameraBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 12.0)),
            ..Default::default()
        });
        // .insert(FlyCamera::default());
}
