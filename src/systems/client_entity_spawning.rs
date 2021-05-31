// use bevy::prelude::*;
// use bevy_rapier3d::physics::{RigidBodyHandleComponent};
// use crate::components::*;
// use crate::events::*;
// use crate::models::*;
// use crate::resources::*;

// pub fn client_entity_spawning_system(
//     commands: &mut Commands,
//     mut state: ResMut<NetworkEventListenerState>,
//     mut entity_spawn_events: EventReader<EntitySpawnEvent>,
//     mut query: Query<(Entity, &LocalPlayer, &mut Synchronized<RigidBodyHandleComponent>)>
// ) {
//     for event in entity_spawn_events.iter() {
//         let server_entity_id = event.spawn.entity_id;
//         println!("Entity spawned: {:?}", server_entity_id);
//         commands.spawn().insert_bundle((Synchronize, ServerEntity {
//             id: server_entity_id
//         }));
//     }
// }
