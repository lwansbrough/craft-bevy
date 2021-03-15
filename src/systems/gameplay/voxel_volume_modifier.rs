use bevy::{
    prelude::*,
};
use crate::{VoxelVolume, resources::*};

/// This system prints out all mouse events as they come in
pub fn voxel_volume_modifier(
    commands: &mut Commands,
    mut voxel_volumes: ResMut<Assets<VoxelVolume>>,
    // Equipped item
    // Player Focus (3D point being looked at, voxel volume handle, voxel coord within volume, material, type (terrain, building, object))
) {
    
}
