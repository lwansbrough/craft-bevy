use bevy::{prelude::*, render::{render_graph::RenderGraph, shader}, render2::RenderStage};

use super::render::VoxelShaders;
use super::render::VoxelVolumeMeta;
use super::render::VoxelVolumeNode;

#[derive(Debug, Default)]
pub struct VoxelRenderPlugin;

impl Plugin for VoxelRenderPlugin {
    fn build(&self, app: &mut App) {
        let render_app = app.sub_app_mut(0);
        render_app
            .add_system_to_stage(RenderStage::Extract, super::extract_voxel_volumes)
            .add_system_to_stage(RenderStage::Prepare, super::prepare_voxel_volumes)
            .add_system_to_stage(RenderStage::Queue, super::queue_voxel_volumes)
            .init_resource::<VoxelShaders>()
            .init_resource::<VoxelVolumeMeta>();

        let render_world = app.sub_app_mut(0).world.cell();
        let mut graph = render_world.get_resource_mut::<RenderGraph>().unwrap();
        graph.add_node("voxels", VoxelVolumeNode);
        graph.add_node_edge("voxels", bevy::core_pipeline::node::MAIN_PASS_DEPENDENCIES).unwrap();
    }
}
