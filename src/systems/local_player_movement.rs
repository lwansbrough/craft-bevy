use bevy::{
    prelude::*,
};
use bevy_rapier3d::rapier::dynamics::{RigidBody, RigidBodySet};
use bevy_rapier3d::rapier::math::{Vector};
use bevy_rapier3d::physics::{RigidBodyHandleComponent};
use crate::components::*;
use crate::models::*;
use crate::resources::*;
use crate::systems::CommandAccumulatorState;

#[derive(Default)]
pub struct LocalPlayerMovementState {
}

fn predict(local_player_movement: &ResMut<LocalPlayerMovementState>, sim_time: &SimulationTime, entity: Entity, input_command: &InputCommand, rigid_body: &mut RigidBody, synchronizable_rigid_body: &mut Synchronized<RigidBodyHandleComponent>) {
    synchronizable_rigid_body.command_frames().push(
        entity.id(),
        sim_time.frame(),
        SynchronizedInput::InputCommand(*input_command)
    );
}

pub fn local_player_movement_system(
    mut state: ResMut<LocalPlayerMovementState>,
    sim_time: Res<SimulationTime>,
    command_accumulator: Res<CommandAccumulatorState>,
    input: Res<Input<KeyCode>>,
    mut rigid_body_set: ResMut<RigidBodySet>,
    mut player_body_query: Query<(Entity, &LocalPlayerBody, &RigidBodyHandleComponent, &mut Synchronized<RigidBodyHandleComponent>)>
) {
    let latest_input_command = *command_accumulator.input_buffer.inputs.back().unwrap_or(&InputCommand::default());

    for (entity, _player_body, rigid_body_handle, mut synchronizable_rigid_body) in &mut player_body_query.iter_mut() {
        let mut rigid_body = rigid_body_set.get_mut(rigid_body_handle.handle()).unwrap();
        predict(&state, &sim_time, entity, &latest_input_command, rigid_body, &mut synchronizable_rigid_body);
    }
}
