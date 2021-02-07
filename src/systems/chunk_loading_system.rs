use bevy::{
    prelude::*,
};
use crate::{LocalPlayerBody, VOXELS_PER_METER, VoxelBundle, VoxelVolume, resources::*};


/// This system prints out all mouse events as they come in
pub fn chunk_loading_system(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials_standard: ResMut<Assets<StandardMaterial>>,
    mut voxel_volumes: ResMut<Assets<VoxelVolume>>,
    world_generator: Res<WorldGenerator>,
    mut world_data: ResMut<WorldData>,
    mut player_body_query: Query<(&LocalPlayerBody, &mut Transform)>,
) {
    for (_, transform) in player_body_query.iter_mut() {
        let new_coords = world_data.move_to([
            transform.translation.x.floor() as i32,
            transform.translation.y.floor() as i32,
            transform.translation.z.floor() as i32
        ]);

        for [x, y, z] in new_coords {
            println!("{:?}, {:?}, {:?}", x, y, z);

            let voxel_volume = world_generator.generate(x, y, z);
            let mut voxel_bundle = VoxelBundle::new(&mut meshes, &mut voxel_volumes, voxel_volume);
            voxel_bundle.transform.translation = Vec3::new(
                world_generator.chunk_size() as f32 / VOXELS_PER_METER * x as f32,
                world_generator.chunk_size() as f32 / VOXELS_PER_METER * y as f32,
                world_generator.chunk_size() as f32 / VOXELS_PER_METER * z as f32
            );
            commands.spawn(voxel_bundle);
        }
    }
}
