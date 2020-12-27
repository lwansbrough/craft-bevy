use bevy::{
    prelude::*,
};
use crate::resources::*;


/// This system prints out all mouse events as they come in
pub fn chunk_loading_system(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    world_generator: Res<WorldGenerator>,
    // mut player_body_query: Query<(&LocalPlayerBody, &mut Translation)>,
) {
    // commands.spawn(PbrComponents {
    //     mesh: meshes.add(world_generator.generate(0, 0, 0)),
    //     material: materials.add(Color::rgb(0.2, 0.1, 0.1).into()),
    //     translation: Translation::new(0.0, 0.0, 0.0),
    //     ..Default::default()
    // });

    // layer below

    let world_chunk_size = 5;

    for x in 0..world_chunk_size {
        for y in 0..world_chunk_size {
            for z in 0..world_chunk_size {
                commands.spawn(PbrBundle {
                    mesh: meshes.add(world_generator.generate(x, y, z)),
                    material: materials.add(Color::rgb(1.0, 0.1, 0.1).into()),
                    transform: Transform::from_translation(Vec3::new((16 * x) as f32, (16 * y) as f32, (16 * z) as f32)),
                    ..Default::default()
                });
            }
        }
    }
}
