use bevy::prelude::*;
use crate::components::Synchronizable;

pub struct Player;
pub struct PlayerHead;
pub struct PlayerBody;

impl Synchronizable for Player {
    fn type_id() -> u8 { 2 }

    fn spawn(world: &mut World, entity:Entity) {
        let mut meshes = world.get_resource_mut::<Assets<Mesh>>().unwrap();
        let mesh = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));

        let mut materials = world.get_resource_mut::<Assets<StandardMaterial>>().unwrap();
        let material = materials.add(Color::rgb(0.5, 0.4, 0.3).into());

        world.entity_mut(entity).insert_bundle(PbrBundle {
            mesh,
            material,
            ..Default::default()
        });
    }

    fn author_serialized_state(&self, world: &mut World) -> Vec<u8> {
        todo!()
    }

    fn consume_serialized_state(&mut self, state: &Vec<u8>, world: &mut World) {
        todo!()
    }
}