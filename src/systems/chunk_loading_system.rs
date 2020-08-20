use bevy::{
    prelude::*,
};
use crate::components::*;
use crate::utilities::*;


/// This system prints out all mouse events as they come in
pub fn chunk_loading_system(
    time: Res<Time>,
    mut player_body_query: Query<(&LocalPlayerBody, &mut Translation)>,
) {
    
}
