mod local_player;
// mod synchronizable;
mod server_entity;
mod synchronized;
mod rigid_body;

pub use self::{
    local_player::*,
    // synchronizable::*,
    server_entity::*,
    synchronized::*,
    rigid_body::*
};
