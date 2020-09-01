use bevy::{
    prelude::*,
};
use crate::models::*;

#[derive(Default)]
pub struct CommandAccumulatorState {
    pub command_buffer: CommandBuffer
}

/// This system prints out all mouse events as they come in
pub fn command_accumulator_system(
    mut state: ResMut<CommandAccumulatorState>,
    input: Res<Input<KeyCode>>
) {
    let mut frame_command = {
        if state.command_buffer.commands.is_empty() {
            InputCommand::default()
        } else {
            *state.command_buffer.commands.back().unwrap()
        }
    };

    frame_command.frame = 5;

    frame_command.forward = input.pressed(KeyCode::W);
    frame_command.backward = input.pressed(KeyCode::S);
    frame_command.right = input.pressed(KeyCode::D);
    frame_command.left = input.pressed(KeyCode::A);
    frame_command.jump = input.pressed(KeyCode::Space);

    state.command_buffer.commands.push_back(frame_command);
}
