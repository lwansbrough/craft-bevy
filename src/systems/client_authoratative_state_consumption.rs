use bevy::prelude::*;
use crate::components::*;
use crate::events::*;
use crate::models::*;
use crate::resources::*;

pub fn client_authoratative_state_consumption_system<TComponent: Synchronizable>(
    commands: &mut Commands,
    mut state: ResMut<NetworkEventListenerState>,
    mut state_frame_events: EventReader<StateFrameEvent>,
    mut query: Query<(Entity, &ServerEntity, &Synchronize)>
) {
    for event in state_frame_events.iter() {
        let state_frame = event.state_frame.clone();

        if state_frame.component_type_id == TComponent::type_id() {
            for (entity, server_entity, _) in query.iter() {
                if server_entity.id == state_frame.entity_id {
                    commands.add(Synchronized::<TComponent>::consume_state_command(entity, state_frame));

                    break;
                }
            }
        }
    }
}
