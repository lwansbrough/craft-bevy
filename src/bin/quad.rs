use std::net::SocketAddr;
use std::time::Duration;

use bevy::{prelude::*, render::renderer::RenderResourceBinding};
use bevy::{render::renderer::{RenderResourceContext, RenderResourceBindings, BufferUsage, BufferInfo}, app::{ScheduleRunnerSettings}};
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
use craft::render::*;
use craft::render::VoxelRenderPlugin;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
        .add_plugin(bevy::diagnostic::PrintDiagnosticsPlugin::default())
        .add_plugin(VoxelRenderPlugin)
        .add_plugin(FlyCameraPlugin)
        .add_startup_system(setup.system())
        .run();
}

/// set up a simple 3D scene
fn setup(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials_voxel: ResMut<Assets<VoxelMaterial>>,
    mut materials_standard: ResMut<Assets<StandardMaterial>>
) {
    commands
        // TODO: Why do I need this in order for the quad to render?
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 10.0 })),
            material: materials_standard.add(Color::rgb(0.1, 0.2, 0.1).into()),
            ..Default::default()
        })
        // Fullscreen quad
        .spawn(VoxelBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(2.0, 2.0)))),
            material: materials_voxel.add(VoxelMaterial {}),
            ..Default::default()
        })
        .spawn(Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, -20.0)),
            ..Default::default()
        })
        .with(FlyCamera::default());
}
