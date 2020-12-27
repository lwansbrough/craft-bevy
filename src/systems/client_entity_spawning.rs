use bevy::prelude::*;
use bevy_rapier3d::physics::{RigidBodyHandleComponent};
use crate::components::*;
use crate::events::*;
use crate::models::*;
use crate::resources::*;

pub fn client_entity_spawning_system(
    commands: &mut Commands,
    mut state: ResMut<NetworkEventListenerState>,
    entity_spawn_events: Res<Events<EntitySpawnEvent>>,
    mut query: Query<(Entity, &LocalPlayer, &mut Synchronized<RigidBodyHandleComponent>)>
) {
    for event in state.entity_spawn_events.iter(&entity_spawn_events) {
        let server_entity_id = event.spawn.entity_id;
        println!("Entity spawned: {:?}", server_entity_id);
        commands.spawn((Synchronize, ServerEntity {
            id: server_entity_id
        }));
    }
}
