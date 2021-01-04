use std::net::SocketAddr;
use std::time::Duration;

use bevy::{prelude::*, render::renderer::RenderResourceBinding};
use bevy::{render::renderer::{RenderResourceContext, RenderResourceBindings, BufferUsage, BufferInfo}, app::{ScheduleRunnerSettings}};
use bevy_rapier3d::physics::{RapierPhysicsPlugin, RigidBodyHandleComponent};
use bevy_rapier3d::rapier::dynamics::{BodyStatus, RigidBody, RigidBodyBuilder};
use bevy_rapier3d::rapier::geometry::ColliderBuilder;
use bevy_prototype_networking_laminar::{NetworkResource, NetworkingPlugin, NetworkDelivery};
use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};
use noise::{MultiFractal, NoiseFn, RidgedMulti, Seedable};

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
        .add_asset::<VoxelVolume>()
        .init_resource::<WindowResizeEventListenerState>()
        .add_startup_system(setup.system())
        .add_system(window_resolution_system.system())
        .run();
}

const NUM_CUBES_PER_ROW: usize = 100;
const NUM_CUBES: usize = NUM_CUBES_PER_ROW * NUM_CUBES_PER_ROW;

/// set up a simple 3D scene
fn setup(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials_voxel: ResMut<Assets<VoxelMaterial>>,
    mut materials_standard: ResMut<Assets<StandardMaterial>>,
    mut voxel_volumes: ResMut<Assets<VoxelVolume>>,
) {
    let voxel_displacement = Vec4::new(
        NUM_CUBES_PER_ROW as f32 * 0.5,
        0.0,
        NUM_CUBES_PER_ROW as f32 * 0.5,
        0.0,
    );
    let noise = RidgedMulti::new()
        .set_seed(1234)
        .set_frequency(0.008)
        .set_octaves(5);
    let mut voxels = Vec::with_capacity(NUM_CUBES as usize);
    let palette = vec![
        Vec4::zero(),
        Vec4::new(0.275, 0.51, 0.706, 1.0),  // Blue
        Vec4::new(1.0, 0.98, 0.804, 1.0),    // Yellow
        Vec4::new(0.604, 0.804, 0.196, 1.0), // Green
        Vec4::new(0.545, 0.271, 0.075, 1.0), // Brown
        Vec4::new(0.502, 0.502, 0.502, 1.0), // Grey
        Vec4::new(1.0, 0.98, 0.98, 1.0),     // White
    ];
    for z in 0..NUM_CUBES_PER_ROW {
        for y in 0..NUM_CUBES_PER_ROW {
            for x in 0..NUM_CUBES_PER_ROW {
                let y_noise = noise.get([x as f64, z as f64]);
                let y_val = ((y_noise + 1.0) / 2.0 * NUM_CUBES_PER_ROW as f64).floor() as usize;

                if (y > y_val) {
                    voxels.push(VoxelData {
                        material: 0,
                    });
                    continue;
                }

                voxels.push(VoxelData {
                    material: match y_noise {
                        y_noise if y_noise < -0.5 => 1, // Blue
                        y_noise if y_noise < -0.4 => 2, // Yellow
                        y_noise if y_noise < -0.2 => 3, // Green
                        y_noise if y_noise < -0.1 => 4, // Brown
                        y_noise if y_noise < 0.6 => 5,  // Grey
                        _ => 6,             // White
                    },
                });
            }
            
        }
    }

    let map = voxel_volumes.add(VoxelVolume {
        palette: palette,
        size: Vec3::new(NUM_CUBES_PER_ROW as f32, NUM_CUBES_PER_ROW as f32, NUM_CUBES_PER_ROW as f32),
        data: voxels
    });

    // let palette = vec![
    //     Vec4::zero(),
    //     Vec4::new(1.0, 0.0, 0.0, 1.0),
    //     Vec4::new(0.0, 1.0, 0.0, 1.0),
    //     Vec4::new(0.0, 0.0, 1.0, 1.0),
    // ];

    // let voxels = vec![
    //     // X
    //     VoxelData { material: 0 },
    //     VoxelData { material: 1 },
    //     VoxelData { material: 1 },
    //     VoxelData { material: 1 },

    //     // Y
    //     VoxelData { material: 2 },
    //     VoxelData { material: 0 },
    //     VoxelData { material: 0 },
    //     VoxelData { material: 0 },
    //     VoxelData { material: 2 },
    //     VoxelData { material: 0 },
    //     VoxelData { material: 0 },
    //     VoxelData { material: 0 },
    //     VoxelData { material: 2 },
    //     VoxelData { material: 0 },
    //     VoxelData { material: 0 },
    //     VoxelData { material: 0 },

    //     // Z
    //     VoxelData { material: 3 },
    //     VoxelData { material: 0 },
    //     VoxelData { material: 0 },
    //     VoxelData { material: 0 },
    //     VoxelData { material: 0 },
    //     VoxelData { material: 0 },
    //     VoxelData { material: 0 },
    //     VoxelData { material: 0 },
    //     VoxelData { material: 0 },
    //     VoxelData { material: 0 },
    //     VoxelData { material: 0 },
    //     VoxelData { material: 0 },
    //     VoxelData { material: 0 },
    //     VoxelData { material: 0 },
    //     VoxelData { material: 0 },
    //     VoxelData { material: 0 },
    //     VoxelData { material: 3 },
    //     VoxelData { material: 0 },
    //     VoxelData { material: 0 },
    //     VoxelData { material: 0 },
    //     VoxelData { material: 0 },
    //     VoxelData { material: 0 },
    //     VoxelData { material: 0 },
    //     VoxelData { material: 0 },
    //     VoxelData { material: 0 },
    //     VoxelData { material: 0 },
    //     VoxelData { material: 0 },
    //     VoxelData { material: 0 },
    //     VoxelData { material: 0 },
    //     VoxelData { material: 0 },
    //     VoxelData { material: 0 },
    //     VoxelData { material: 0 },
    //     VoxelData { material: 3 },
    //     VoxelData { material: 0 },
    //     VoxelData { material: 0 },
    //     VoxelData { material: 0 },
    //     VoxelData { material: 0 },
    //     VoxelData { material: 0 },
    //     VoxelData { material: 0 },
    //     VoxelData { material: 0 },
    //     VoxelData { material: 0 },
    //     VoxelData { material: 0 },
    //     VoxelData { material: 0 },
    //     VoxelData { material: 0 },
    //     VoxelData { material: 0 },
    //     VoxelData { material: 0 },
    //     VoxelData { material: 0 },
    //     VoxelData { material: 0 },
    // ];

    // let map = voxel_volumes.add(VoxelVolume {
    //     palette: palette,
    //     size: Vec3::new(4.0, 4.0, 4.0),
    //     data: voxels
    // });

    commands
        // Fullscreen quad
        .spawn(VoxelBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(2.0, 2.0)))),
            material: materials_voxel.add(VoxelMaterial::default()),
            voxel_volume: map,
            ..Default::default()
        })
        .spawn(Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 20.0)),
            ..Default::default()
        })
        .with(FlyCamera::default());
}
