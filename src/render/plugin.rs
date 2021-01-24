use bevy::{
    prelude::*,
    render::{render_graph::RenderGraph, shader, stage::RENDER_RESOURCE},
};

use super::voxel_volume;

#[derive(Debug, Default)]
pub struct VoxelRenderPlugin;

impl Plugin for VoxelRenderPlugin {
    fn build(&self, app: &mut AppBuilder) {
            // .add_system_to_stage(
            //     RENDER_RESOURCE,
            //     voxel_volume::voxel_resource_provider_system.system()
            // );
        let resources = app.resources();
        let mut render_graph = resources.get_mut::<RenderGraph>().unwrap();
        super::graph::add_voxel_graph(&mut render_graph, resources);
    }
}
