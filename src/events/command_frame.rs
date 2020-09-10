use crate::models::*;

#[derive(Debug)]
pub struct CommandFrameEvent {
    from: u128,
    command_frame: CommandFrame,
}
