use bevy::{prelude::*, render2::{RenderApp, RenderStage, RenderWorld, render_graph::{RenderGraph, SlotInfo, SlotType}, render_phase::sort_phase_system}};

use crate::{VoxelPassNode, VoxelPhase};

use super::render::VoxelShaders;
use super::render::VoxelVolumeMeta;
use super::render::VoxelVolumeNode;

pub mod draw_voxels_graph {
    pub const NAME: &str = "draw_voxels";

    pub mod input {
        pub const VIEW_ENTITY: &str = "view_entity";
        pub const RENDER_TARGET: &str = "render_target";
        pub const DEPTH: &str = "depth";
    }

    pub mod node {
        pub const MAIN_PASS: &str = "main_pass";
    }
}

#[derive(Debug, Default)]
pub struct VoxelRenderPlugin;

pub struct RenderVoxelsApp;

impl Plugin for VoxelRenderPlugin {
    fn build(&self, app: &mut App) {
        let render_app = app.sub_app(RenderApp);
        render_app
            .add_system_to_stage(RenderStage::Extract, super::extract_voxel_volumes)
            .add_system_to_stage(RenderStage::Prepare, super::prepare_render_texture)
            .add_system_to_stage(RenderStage::Prepare, super::prepare_voxel_volumes)
            .add_system_to_stage(RenderStage::Queue, super::queue_voxel_volumes)
            .add_system_to_stage(RenderStage::PhaseSort, sort_phase_system::<VoxelPhase>)
            .init_resource::<VoxelShaders>()
            .init_resource::<VoxelVolumeMeta>();

        let voxel_pass_node = VoxelPassNode::new(&mut render_app.world);
        let mut graph = render_app.world.get_resource_mut::<RenderGraph>().unwrap();
        // let mut graph = graph
        //     .get_sub_graph_mut(bevy::core_pipeline::draw_3d_graph::NAME)
        //     .unwrap();

        graph.add_node("voxels", VoxelVolumeNode);
        // graph
        //     .add_node_edge("voxels", bevy::core_pipeline::draw_3d_graph::node::MAIN_PASS)
        //     .unwrap();
        graph
            .add_node_edge("voxels", bevy::core_pipeline::node::MAIN_PASS_DEPENDENCIES)
            .unwrap();

        let mut draw_voxels_graph = RenderGraph::default();
        draw_voxels_graph.add_node(draw_voxels_graph::node::MAIN_PASS, voxel_pass_node);

        let input_node_id = draw_voxels_graph.set_input(vec![
            SlotInfo::new(draw_voxels_graph::input::VIEW_ENTITY, SlotType::Entity),
            SlotInfo::new(draw_voxels_graph::input::RENDER_TARGET, SlotType::TextureView),
            // SlotInfo::new(draw_voxels_graph::input::DEPTH, SlotType::TextureView),
        ]);

        draw_voxels_graph
            .add_slot_edge(
                input_node_id,
                draw_voxels_graph::input::VIEW_ENTITY,
                draw_voxels_graph::node::MAIN_PASS,
                VoxelPassNode::IN_VIEW,
            )
            .unwrap();
        draw_voxels_graph
            .add_slot_edge(
                input_node_id,
                draw_voxels_graph::input::RENDER_TARGET,
                draw_voxels_graph::node::MAIN_PASS,
                VoxelPassNode::IN_COLOR_ATTACHMENT,
            )
            .unwrap();
        graph.add_sub_graph(draw_voxels_graph::NAME, draw_voxels_graph);
    }
}
