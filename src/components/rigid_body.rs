use bevy::ecs::{Entity, Resources, World};
use bevy_rapier3d::{
    rapier::dynamics::{RigidBody, RigidBodySet},
    physics::RigidBodyHandleComponent
};
use crate::components::Synchronizable;

impl Synchronizable for RigidBodyHandleComponent {
    fn type_id() -> u8 { 1 }

    fn spawn(world: &mut World, resources: &mut Resources, entity: Entity) {
        // No-op, rigid body spawning is handled by "parent" components
    }

    fn author_serialized_state(&self, resources: &mut Resources) -> Vec<u8> {
        let rigid_body_set = resources.get::<RigidBodySet>().unwrap();
        let rigid_body = rigid_body_set.get(self.handle()).unwrap();
        bincode::serialize(rigid_body).unwrap()
    }

    fn consume_serialized_state(&mut self, state: &Vec<u8>, resources: &mut Resources) {
        let deserialized_rigid_body = bincode::deserialize::<RigidBody>(&state[..]).unwrap();
        let mut rigid_body_set = resources.get_mut::<RigidBodySet>().unwrap();
        let mut rigid_body = rigid_body_set.get_mut(self.handle()).unwrap();

        rigid_body.set_position(*deserialized_rigid_body.position(), false);
        rigid_body.set_linvel(*deserialized_rigid_body.linvel(), false);
        rigid_body.set_angvel(*deserialized_rigid_body.angvel(), false);
        rigid_body.wake_up(false);
    }
}
