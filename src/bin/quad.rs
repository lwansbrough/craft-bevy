use std::net::SocketAddr;
use std::time::Duration;

use bevy::{core::AsBytes, prelude::*, render::renderer::RenderResourceBinding};
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
    let mut palette = vec![Vec4::zero(); 255];
    palette[1] = Vec4::new(0.086, 0.651, 0.780, 1.0);  // Blue
    palette[2] = Vec4::new(0.900, 0.894, 0.737, 1.0); // Yellow
    palette[3] = Vec4::new(0.196, 0.659, 0.321, 1.0); // Green
    palette[4] = Vec4::new(0.545, 0.271, 0.075, 1.0); // Brown
    palette[5] = Vec4::new(0.502, 0.502, 0.502, 1.0); // Grey
    palette[6] = Vec4::new(1.0, 0.98, 0.98, 1.0);     // White
    
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

    let mut voxels2 = Vec::with_capacity(VOLUME_SIZE[0] as usize * VOLUME_SIZE[1] as usize * VOLUME_SIZE[2] as usize);
    let mut palette2 = vec![Vec4::zero(); 255];
    palette2[1] = Vec4::new(0.086, 0.651, 0.780, 1.0);  // Blue
    palette2[2] = Vec4::new(0.900, 0.894, 0.737, 1.0); // Yellow
    palette2[3] = Vec4::new(0.196, 0.659, 0.321, 1.0); // Green
    palette2[4] = Vec4::new(0.545, 0.271, 0.075, 1.0); // Brown
    palette2[5] = Vec4::new(0.502, 0.502, 0.502, 1.0); // Grey
    palette2[6] = Vec4::new(1.0, 0.98, 0.98, 1.0);     // White
    
    let OFFSET: f64 = 1.0;
    for z in 0..VOLUME_SIZE[2] as u32 {
        for y in 0..VOLUME_SIZE[1] as u32 {
            for x in 0..VOLUME_SIZE[0] as u32 {
                let y_noise = generator.get([x as f64 / 20.0 + OFFSET, y as f64 / 20.0, z as f64 / 20.0]);

                if y_noise == 0.0 {
                    voxels2.push(VoxelData { material: 0 });
                    continue;
                }

                voxels2.push(VoxelData {
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

    let map2 = voxel_volumes.add(VoxelVolume {
        palette: palette2,
        size: Vec3::new(VOLUME_SIZE[0] as f32, VOLUME_SIZE[1] as f32, VOLUME_SIZE[2] as f32),
        data: voxels2
    });

    // let test = VoxelVolume {
    //     palette: palette,
    //     size: Vec3::new(VOLUME_SIZE[0] as f32, VOLUME_SIZE[1] as f32, VOLUME_SIZE[2] as f32),
    //     data: voxels
    // };

    // println!("{:?}", test.to_bytes());

    // println!("{:?}", Vec3::new(VOLUME_SIZE[0] as f32, VOLUME_SIZE[1] as f32, VOLUME_SIZE[2] as f32).as_bytes());

    let mut palette3 = vec![Vec4::zero(); 255];
    palette3[1] = Vec4::new(1.0, 0.0, 0.0, 1.0);
    palette3[2] = Vec4::new(0.0, 1.0, 0.0, 1.0);
    palette3[3] = Vec4::new(0.0, 0.0, 1.0, 1.0);

    let mut voxels3 = vec![VoxelData { material: 0}; 64];
        // X
    voxels3[0] = VoxelData { material: 0 };
    voxels3[1] = VoxelData { material: 1 };
    voxels3[2] = VoxelData { material: 1 };
    voxels3[3] = VoxelData { material: 1 };

        // Y
    voxels3[4] = VoxelData { material: 2 };
    voxels3[5] = VoxelData { material: 0 };
    voxels3[6] = VoxelData { material: 0 };
    voxels3[7] = VoxelData { material: 0 };
    voxels3[8] = VoxelData { material: 2 };
    voxels3[9] = VoxelData { material: 0 };
    voxels3[10] = VoxelData { material: 0 };
    voxels3[11] = VoxelData { material: 0 };
    voxels3[12] = VoxelData { material: 2 };
    voxels3[13] = VoxelData { material: 0 };
    voxels3[14] = VoxelData { material: 0 };
    voxels3[15] = VoxelData { material: 0 };

        // Z
    voxels3[16] = VoxelData { material: 3 };
    voxels3[17] = VoxelData { material: 0 };
    voxels3[18] = VoxelData { material: 0 };
    voxels3[19] = VoxelData { material: 0 };
    voxels3[20] = VoxelData { material: 0 };
    voxels3[21] = VoxelData { material: 0 };
    voxels3[22] = VoxelData { material: 0 };
    voxels3[23] = VoxelData { material: 0 };
    voxels3[24] = VoxelData { material: 0 };
    voxels3[25] = VoxelData { material: 0 };
    voxels3[26] = VoxelData { material: 0 };
    voxels3[27] = VoxelData { material: 0 };
    voxels3[28] = VoxelData { material: 0 };
    voxels3[29] = VoxelData { material: 0 };
    voxels3[30] = VoxelData { material: 0 };
    voxels3[31] = VoxelData { material: 0 };
    voxels3[32] = VoxelData { material: 3 };
    voxels3[33] = VoxelData { material: 0 };
    voxels3[34] = VoxelData { material: 0 };
    voxels3[35] = VoxelData { material: 0 };
    voxels3[36] = VoxelData { material: 0 };
    voxels3[37] = VoxelData { material: 0 };
    voxels3[38] = VoxelData { material: 0 };
    voxels3[39] = VoxelData { material: 0 };
    voxels3[40] = VoxelData { material: 0 };
    voxels3[41] = VoxelData { material: 0 };
    voxels3[42] = VoxelData { material: 0 };
    voxels3[43] = VoxelData { material: 0 };
    voxels3[44] = VoxelData { material: 0 };
    voxels3[45] = VoxelData { material: 0 };
    voxels3[46] = VoxelData { material: 0 };
    voxels3[47] = VoxelData { material: 0 };
    voxels3[48] = VoxelData { material: 3 };
    voxels3[49] = VoxelData { material: 0 };
    voxels3[50] = VoxelData { material: 0 };
    voxels3[51] = VoxelData { material: 0 };
    voxels3[52] = VoxelData { material: 0 };
    voxels3[53] = VoxelData { material: 0 };
    voxels3[54] = VoxelData { material: 0 };
    voxels3[55] = VoxelData { material: 0 };
    voxels3[56] = VoxelData { material: 0 };
    voxels3[57] = VoxelData { material: 0 };
    voxels3[58] = VoxelData { material: 0 };
    voxels3[59] = VoxelData { material: 0 };
    voxels3[60] = VoxelData { material: 0 };
    voxels3[61] = VoxelData { material: 0 };
    voxels3[62] = VoxelData { material: 0 };
    voxels3[63] = VoxelData { material: 0 };

    let map3 = voxel_volumes.add(VoxelVolume {
        palette: palette3,
        size: Vec3::new(4.0, 4.0, 4.0),
        data: voxels3
    });

    let quad = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(2.0, 2.0))));

    commands
        // Fullscreen quad
        .spawn(VoxelBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 6.0 })),
            voxel_volume: map3,
            // transform: Transform::identity().looking_at(Vec3::new(2.0, 0.0, 2.0), Vec3::unit_z()),
            ..Default::default()
        })
        // .spawn(VoxelBundle {
        //     mesh: meshes.add(Mesh::from(shape::Cube { size: 16.0 })),
        //     voxel_volume: map,
        //     ..Default::default()
        // })
        // .spawn(VoxelBundle {
        //     mesh: quad.clone(),
        //     voxel_volume: map,
        //     ..Default::default()
        // })
        // .spawn(VoxelBundle {
        //     mesh: quad.clone(),
        //     voxel_volume: map2,
        //     transform: Transform::from_translation(Vec3::new(0.0, 0.0, 20.0)),
        //     ..Default::default()
        // })
        // .spawn(PbrBundle {
        //     material: materials_standard.add(bevy::render::color::Color::ALICE_BLUE.into()),
        //     transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        //     mesh: meshes.add(Mesh::from(shape::Plane { size: 1000.0 })),
        //     ..Default::default()
        // })
        .spawn(Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 20.0)),
            ..Default::default()
        })
        .with(FlyCamera::default());
}
