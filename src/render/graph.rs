use bevy::{
    asset::Assets,
    ecs::Resources,
    render::{
        pipeline::PipelineDescriptor,
        render_graph::{base, AssetRenderResourcesNode, RenderGraph, RenderResourcesNode},
        shader::Shader,
    },
    transform::prelude::GlobalTransform,
};
use nodes::TimeNode;

pub mod nodes;
pub mod pipeline;

pub mod node {
    pub const TIME: &str = "time";
    pub const TRANSFORM: &str = "transform";
    pub const VOXEL_MATERIAL: &str = "voxel_material";
}

pub mod uniform {
    pub const TIME: &str = "Time";
}

pub(crate) fn add_voxel_graph(graph: &mut RenderGraph, resources: &Resources) {
    // graph.add_system_node(
    //     node::TRANSFORM,
    //     RenderResourcesNode::<GlobalTransform>::new(true)
    // );
    graph.add_system_node(node::TIME, TimeNode::new());

    let mut shaders = resources.get_mut::<Assets<Shader>>().unwrap();
    let mut pipelines = resources.get_mut::<Assets<PipelineDescriptor>>().unwrap();
    pipelines.set_untracked(
        pipeline::PIPELINE_HANDLE,
        pipeline::build_pipeline(&mut shaders),
    );

    graph.add_node_edge(node::TIME, base::node::MAIN_PASS).unwrap();
    // graph.add_node_edge(node::TRANSFORM, base::node::MAIN_PASS).unwrap();
}
