use bevy::{
    prelude::*,
};
use bevy_rapier3d::rapier::dynamics::{RigidBody, RigidBodyMut, RigidBodySet};
use bevy_rapier3d::rapier::math::{Vector};
use bevy_rapier3d::physics::{RigidBodyHandleComponent};
use crate::components::*;
use crate::models::*;
use crate::systems::CommandAccumulatorState;

#[derive(Default)]
pub struct LocalPlayerMovementState {
}

fn predict(local_player_movement: &ResMut<LocalPlayerMovementState>, entity: Entity, input_command: &InputCommand, mut rigid_body: RigidBodyMut, synchronizable_rigid_body: &mut Synchronizable<RigidBodyHandleComponent>) {
    println!("Command: {:?}", &input_command);

    if input_command.jump {
        rigid_body.apply_impulse(Vector::new(0.0, 10.0, 0.0));
    }
    
    if input_command.left {
        rigid_body.apply_force(Vector::new(-100.0, 0.0, 0.0));
    }

    if input_command.right {
        rigid_body.apply_force(Vector::new(100.0, 0.0, 0.0));
    }

    if input_command.forward {
        rigid_body.apply_force(Vector::new(0.0, 0.0, -100.0));
    }

    if input_command.backward {
        rigid_body.apply_force(Vector::new(0.0, 0.0, 100.0));
    }

    let framed_input_command = InputFrame {
        entity_id: entity.id(),
        frame: 5,
        input: SynchronizedInput::InputCommand(*input_command)
    };

    let framed_prediction = StateFrame {
        entity_id: entity.id(),
        frame: 5,
        state: SynchronizedState::RigidBody(rigid_body.clone())
    };

    synchronizable_rigid_body.inputs_mut().push_back(framed_input_command);
    synchronizable_rigid_body.predictions_mut().push_back(framed_prediction);
}

/// This system prints out all mouse events as they come in
pub fn local_player_movement_system(
    mut state: ResMut<LocalPlayerMovementState>,
    time: Res<Time>,
    command_accumulator: Res<CommandAccumulatorState>,
    input: Res<Input<KeyCode>>,
    mut rigid_body_set: ResMut<RigidBodySet>,
    mut player_body_query: Query<(Entity, &LocalPlayerBody, &RigidBodyHandleComponent, &mut Synchronizable<RigidBodyHandleComponent>)>
) {
    let latest_input_command = *command_accumulator.command_buffer.commands.back().unwrap_or(&InputCommand::default());

    for (entity, _player_body, rigid_body_handle, mut synchronizable_rigid_body) in &mut player_body_query.iter() {
        let mut rigid_body = rigid_body_set.get_mut(rigid_body_handle.handle()).unwrap();
        predict(&state, entity, &latest_input_command, rigid_body, &mut synchronizable_rigid_body);
    }
}
