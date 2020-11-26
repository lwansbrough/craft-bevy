use bevy::{
    prelude::*,
};
use bevy_rapier3d::rapier::dynamics::{RigidBody, RigidBodyMut, RigidBodySet};
use bevy_rapier3d::rapier::math::{Vector};
use bevy_rapier3d::physics::{RigidBodyHandleComponent};
use crate::components::*;
use crate::models::*;
use crate::resources::*;
use crate::systems::CommandAccumulatorState;

#[derive(Default)]
pub struct PlayerMovementState {
}

fn move_player(local_player_movement: &ResMut<PlayerMovementState>, sim_time: &SimulationTime, entity: Entity, mut rigid_body: RigidBodyMut, synchronizable_rigid_body: &mut Synchronizable<RigidBodyHandleComponent>) {
    let mut command_frames = synchronizable_rigid_body.command_frames();
    let command_frame = command_frames.history_iter(3).next();
    
    if let Some(command_frame) = command_frame {
        if let SynchronizedInput::InputCommand(input_command) = command_frame.input {
            println!("Command: {:?}", &input_command);

            if input_command.jump {
                rigid_body.apply_impulse(Vector::new(0.0, 10.0, 0.0));
            }
            
            if input_command.left {
                rigid_body.apply_impulse(Vector::new(-2.0, 0.0, 0.0));
            }

            if input_command.right {
                rigid_body.apply_impulse(Vector::new(2.0, 0.0, 0.0));
            }

            if input_command.forward {
                rigid_body.apply_impulse(Vector::new(0.0, 0.0, -2.0));
            }

            if input_command.backward {
                rigid_body.apply_impulse(Vector::new(0.0, 0.0, 2.0));
            }

            synchronizable_rigid_body.state_frames().push(
                entity.id(),
                sim_time.frame(),
                SynchronizedState::RigidBody(rigid_body.clone())
            );
        }
    }
}

pub fn player_movement_system(
    mut state: ResMut<PlayerMovementState>,
    sim_time: Res<SimulationTime>,
    mut rigid_body_set: ResMut<RigidBodySet>,
    mut player_body_query: Query<(Entity, &LocalPlayerBody, &RigidBodyHandleComponent, &mut Synchronizable<RigidBodyHandleComponent>)>
) {
    for (entity, _player_body, rigid_body_handle, mut synchronizable_rigid_body) in player_body_query.iter_mut() {
        let mut rigid_body = rigid_body_set.get_mut(rigid_body_handle.handle()).unwrap();
        move_player(&state, &sim_time, entity, rigid_body, &mut synchronizable_rigid_body);
    }
}
