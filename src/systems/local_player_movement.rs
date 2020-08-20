use bevy::{
    prelude::*,
};
use crate::systems::CommandAccumulatorState;

#[derive(Default)]
pub struct LocalPlayerMovementState {
}

/// This system prints out all mouse events as they come in
pub fn local_player_movement_system(
    mut state: ResMut<LocalPlayerMovementState>,
    time: Res<Time>,
    command_accumulator: Res<CommandAccumulatorState>,
    input: Res<Input<KeyCode>>
) {
    println!("Command: {:?}", *command_accumulator.command_buffer.commands.back().unwrap());
}
