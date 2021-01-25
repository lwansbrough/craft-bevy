use std::net::SocketAddr;
use std::time::Duration;

use bevy::{prelude::*, render::renderer::RenderResourceBinding};
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
        .init_resource::<WindowResizeEventListenerState>()
        .add_startup_system(setup.system())
        .add_system(window_resolution_system.system())
        .run();
}

/// set up a simple 3D scene
fn setup(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials_standard: ResMut<Assets<StandardMaterial>>,
    mut voxel_volumes: ResMut<Assets<VoxelVolume>>,
) {
    let VOLUME_SIZE: [u16; 3] = [16, 16, 16];

    // let test = Octree::new(VOLUME_SIZE, 12);

    let ground_gradient = Gradient::new()
        .set_x_start(0.0)
        .set_y_stop(1.0);

    let lowland_shape_fractal = Billow::new()
        .set_octaves(2)
        .set_frequency(0.25);

    let lowland_autocorrect = Clamp::<[f64; 3]>::new(&lowland_shape_fractal) // TODO: Should use AutoCorrect not Clamp
        .set_bounds(-1.0, 1.0);

    let lowland_scale = ScaleBias::new(&lowland_autocorrect)
        .set_bias(-0.45)
        .set_scale(0.125);

    let lowland_y_scale = ScalePoint::new(&lowland_scale)
        .set_y_scale(0.0);
    
    let lowland_terrain = Displace::new(
        &ground_gradient,
        Constant::new(0.0),
        &lowland_y_scale,
        Constant::new(0.0),
        Constant::new(0.0)
    );

    let highland_shape_fractal = Fbm::new()
        .set_octaves(4)
        .set_frequency(2.0);

    let highland_autocorrect = Clamp::<[f64; 3]>::new(&highland_shape_fractal) // TODO: Should use AutoCorrect not Clamp
        .set_bounds(-1.0, 1.0);

    let highland_scale = ScaleBias::new(&highland_autocorrect)
        .set_bias(0.0)
        .set_scale(0.25);

    let highland_y_scale = ScalePoint::new(&highland_scale)
        .set_y_scale(0.0);

    let highland_terrain = Displace::new(
        &ground_gradient,
        Constant::new(0.0),
        &highland_y_scale,
        Constant::new(0.0),
        Constant::new(0.0)
    );

    let mountain_shape_fractal = RidgedMulti::new()
        .set_octaves(4)
        .set_frequency(2.0);
    
    let mountain_autocorrect = Clamp::new(&mountain_shape_fractal) // TODO: Should use AutoCorrect not Clamp
        .set_bounds(-1.0, 1.0);

    let mountain_scale = ScaleBias::new(&mountain_autocorrect)
        .set_bias(0.15)
        .set_scale(0.45);

    let mountain_y_scale = ScalePoint::new(&mountain_scale)
        .set_y_scale(0.25);

    let mountain_terrain = Displace::new(
        &ground_gradient,
        Constant::new(0.0),
        &mountain_y_scale,
        Constant::new(0.0),
        Constant::new(0.0)
    );

    let terrain_type_fractal = Fbm::new()
        .set_octaves(3)
        .set_frequency(0.125);

    let terrain_type_autocorrect = Clamp::new(&terrain_type_fractal) // TODO: Should use AutoCorrect not Clamp
        .set_bounds(-1.0, 1.0);

    let terrain_type_y_scale = ScalePoint::new(&terrain_type_autocorrect)
        .set_y_scale(0.0);

    let terrain_type_cache = Cache::new(&terrain_type_y_scale);

    let highland_mountain_select = Select::new(&highland_terrain, &mountain_terrain, &terrain_type_cache)
        .set_falloff(0.2);

    let highland_lowland_select = Select::new(&lowland_terrain, &highland_mountain_select, &terrain_type_cache)
        .set_falloff(0.15);

    let highland_lowland_select_cache = Cache::new(&highland_lowland_select);

    let source1 = Constant::new(0.0);
    let source2 = Constant::new(1.0);
    let generator = Select::new(&source1, &source2, &highland_lowland_select_cache);

    let mut voxels = Vec::with_capacity(VOLUME_SIZE[0] as usize * VOLUME_SIZE[1] as usize * VOLUME_SIZE[2] as usize);
    let palette = vec![
        Vec4::zero(),
        Vec4::new(0.086, 0.651, 0.780, 1.0),  // Blue
        Vec4::new(0.900, 0.894, 0.737, 1.0), // Yellow
        Vec4::new(0.196, 0.659, 0.321, 1.0), // Green
        Vec4::new(0.545, 0.271, 0.075, 1.0), // Brown
        Vec4::new(0.502, 0.502, 0.502, 1.0), // Grey
        Vec4::new(1.0, 0.98, 0.98, 1.0),     // White
    ];
    
    let OFFSET: f64 = 1.0;
    for z in 0..VOLUME_SIZE[2] as u32 {
        for y in 0..VOLUME_SIZE[1] as u32 {
            for x in 0..VOLUME_SIZE[0] as u32 {
                let y_noise = generator.get([x as f64 / 20.0 + OFFSET, y as f64 / 20.0, z as f64 / 20.0]);

                if y_noise == 0.0 {
                    voxels.push(VoxelData { material: 0 });
                    continue;
                }

                voxels.push(VoxelData {
                    material: match y {
                        y if y < 25 => 1, // Blue
                        y if y < 27 => 2, // Yellow
                        y if y < 35 => 3, // Green
                        y if y < 50 => 4, // Brown
                        y if y < 70 => 5, // Grey
                        _ => 6,           // White
                    },
                });
            }
            
        }
    }

    let map = voxel_volumes.add(VoxelVolume {
        palette: palette,
        size: Vec3::new(VOLUME_SIZE[0] as f32, VOLUME_SIZE[1] as f32, VOLUME_SIZE[2] as f32),
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
            material: materials_standard.add(bevy::render::color::Color::ALICE_BLUE.into()),
            voxel_volume: map,
            ..Default::default()
        })
        // .spawn(PbrBundle {
        //     material: materials_standard.add(bevy::render::color::Color::ALICE_BLUE.into()),
        //     transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        //     mesh: meshes.add(Mesh::from(shape::Plane { size: 10.0 })),
        //     ..Default::default()
        // })
        .spawn(Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 20.0)),
            ..Default::default()
        })
        .with(FlyCamera::default());
}
