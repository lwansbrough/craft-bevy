use bevy::{
    prelude::*,
    render::{render_graph::RenderGraph, shader},
};

use super::voxel_volume;

#[derive(Debug, Default)]
pub struct VoxelRenderPlugin;

impl Plugin for VoxelRenderPlugin {
    fn build(&self, app: &mut AppBuilder) {
        super::graph::add_voxel_graph(app.world_mut());
    }
}
