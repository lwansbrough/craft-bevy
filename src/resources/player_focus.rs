use bevy::prelude::*;

pub struct PlayerFocus {
    pub entity: Option<Entity>,
    pub voxel_coord: Option<Vec3>
}

impl PlayerFocus {
    pub fn new() -> PlayerFocus {

        PlayerFocus {
            entity: None,
            voxel_coord: None
        }
    }
}

impl Default for PlayerFocus {
    fn default() -> Self {
        PlayerFocus::new()
    }
}
