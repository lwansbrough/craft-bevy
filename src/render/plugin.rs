use bevy::{
    prelude::*,
    render::{render_graph::RenderGraph, shader},
};

#[derive(Debug, Default)]
pub struct VoxelRenderPlugin;

impl Plugin for VoxelRenderPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_asset::<super::material::VoxelMaterial>()
            .add_system_to_stage(
                stage::POST_UPDATE,
                shader::asset_shader_defs_system::<super::material::VoxelMaterial>.system(),
            );
        let resources = app.resources();
        let mut render_graph = resources.get_mut::<RenderGraph>().unwrap();
        super::graph::add_voxel_graph(&mut render_graph, resources);
    }
}
