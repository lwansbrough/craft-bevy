use bevy::{
    prelude::*,
};
use crate::models::*;
use crate::resources::*;

#[derive(Default)]
pub struct CommandAccumulatorState {
    pub input_buffer: InputCommandBuffer
}

/// This system accumulates input events as they come in
pub fn command_accumulator_system(
    mut state: ResMut<CommandAccumulatorState>,
    sim_time: Res<SimulationTime>,
    input: Res<Input<KeyCode>>
) {
    let mut frame_command = {
        if state.input_buffer.inputs.is_empty() {
            InputCommand::default()
        } else {
            *state.input_buffer.inputs.back().unwrap()
        }
    };

    frame_command.frame = sim_time.frame();

    frame_command.forward = input.pressed(KeyCode::W);
    frame_command.backward = input.pressed(KeyCode::S);
    frame_command.right = input.pressed(KeyCode::D);
    frame_command.left = input.pressed(KeyCode::A);
    frame_command.jump = input.pressed(KeyCode::Space);

    state.input_buffer.inputs.push_back(frame_command);
}
