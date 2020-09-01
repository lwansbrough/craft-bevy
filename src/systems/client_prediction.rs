use bevy::{
    prelude::*,
};
use serde::{Serialize};
use crate::components::*;
use crate::models::*;

/// This system prints out all mouse events as they come in
pub fn client_prediction<TComponent: 'static + Send + Sync>(
    time: Res<Time>,
    mut synchronizable_entity_query: Query<(Entity, &mut Synchronizable<TComponent>)>,
) {
    for (entity, mut synchronizable) in &mut synchronizable_entity_query.iter() {
        let frame: u32 = 5;   
        let input_frame = synchronizable.input_frame(frame);

        let bytes: Vec<u8> = bincode::serialize(&input_frame).unwrap();
        println!("Entity {} sending message for sim frame {}. Data: {:?}", entity.id(), frame, &bytes);
    }
}
