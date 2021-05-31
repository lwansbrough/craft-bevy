use bevy::{
    prelude::*,
};
use crate::{LocalPlayerBody, VOXELS_PER_METER, VoxelBundle, VoxelVolume, resources::*};

/// This system prints out all mouse events as they come in
pub fn chunk_loading_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials_standard: ResMut<Assets<StandardMaterial>>,
    mut voxel_volumes: ResMut<Assets<VoxelVolume>>,
    world_generator: Res<WorldGenerator>,
    mut world_data: ResMut<WorldData>,
    mut player_body_query: Query<(&LocalPlayerBody, &mut Transform)>,
) {
    // for (_, transform) in player_body_query.iter_mut() {
    //     let new_coords = world_data.move_to([
    //         transform.translation.x.floor() as i32,
    //         transform.translation.y.floor() as i32,
    //         transform.translation.z.floor() as i32
    //     ]);

    //     for [x, y, z] in new_coords {
    //         println!("{:?}, {:?}, {:?}", x, y, z);

    //         let voxel_volume = world_generator.generate(x, y, z);
    //         let mut voxel_bundle = VoxelBundle::new(&mut meshes, &mut voxel_volumes, voxel_volume);
    //         voxel_bundle.transform.translation = Vec3::new(
    //             world_generator.chunk_size() as f32 / VOXELS_PER_METER * x as f32,
    //             world_generator.chunk_size() as f32 / VOXELS_PER_METER * y as f32,
    //             world_generator.chunk_size() as f32 / VOXELS_PER_METER * z as f32
    //         );
    //         commands.spawn(voxel_bundle);
    //     }
    // }

    for x in -2..=2 {
        for z in -2..=2 {
            let voxel_volume = world_generator.generate(x, 1, z);
            let mut voxel_bundle = VoxelBundle::new(&mut meshes, &mut voxel_volumes, voxel_volume);
            // let mut voxel_bundle = PbrBundle {
            //     material: materials_standard.add(bevy::render::color::Color::GREEN.into()),
            //     mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            //     ..Default::default()
            // };
            
            voxel_bundle.transform.translation = Vec3::new(
                x as f32 * world_generator.chunk_size() as f32 / VOXELS_PER_METER,
                0 as f32 * world_generator.chunk_size() as f32 / VOXELS_PER_METER,
                z as f32 * world_generator.chunk_size() as f32 / VOXELS_PER_METER
            );
            
            commands.spawn().insert_bundle(voxel_bundle);
        }
    }
}
