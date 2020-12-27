use bevy::prelude::*;
use bevy::ecs::{Entity, Resources, World};
use crate::components::Synchronizable;

pub struct Player;
pub struct PlayerHead;
pub struct PlayerBody;

impl Synchronizable for Player {
    fn type_id() -> u8 { 2 }

    fn spawn(world: &mut World, resources: &mut Resources, entity:Entity) {
        let mut meshes = resources.get_mut::<Assets<Mesh>>().unwrap();
        let mut materials = resources.get_mut::<Assets<StandardMaterial>>().unwrap();

        world.insert(entity, PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.5, 0.4, 0.3).into()),
            ..Default::default()
        });
    }

    fn author_serialized_state(&self, resources: &mut bevy::ecs::Resources) -> Vec<u8> {
        todo!()
    }

    fn consume_serialized_state(&mut self, state: &Vec<u8>, resources: &mut bevy::ecs::Resources) {
        todo!()
    }
}