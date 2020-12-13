use bevy::prelude::*;
use bevy_rapier3d::physics::{RigidBodyHandleComponent};
use crate::components::*;
use crate::events::*;
use crate::models::*;
use crate::resources::*;

pub fn server_player_movement_system(
    mut state: ResMut<NetworkEventListenerState>,
    command_frame_events: Res<Events<CommandFrameEvent>>,
    mut query: Query<(Entity, &LocalPlayer, &mut Synchronized)>
) {
    for event in state.command_frame_events.iter(&command_frame_events) {

        // If the command frame is for an input command then we're in control of it, proceed
        if let SynchronizedInput::InputCommand(_) = event.command_frame.input {
            for (entity, _, mut synchronized) in query.iter_mut() {
                if synchronized.component_type_id() != 1 {
                    continue;
                }

                if entity.id() != event.command_frame.entity_id {
                    continue;
                }

                synchronized.command_frames().push(
                    event.command_frame.entity_id,
                    event.command_frame.frame,
                    event.command_frame.input
                );

                // No more entities by the same ID (hopefully?!)
                break
            }
        }
    }
}
