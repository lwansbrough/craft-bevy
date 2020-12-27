use bevy::{ecs::{Bundle, ResMut}, prelude::{Assets, Draw, GlobalTransform, Handle, Mesh, RenderPipelines, Transform, shape::{self, Quad}}, render::{mesh::VertexAttributeValues, pipeline::{PrimitiveTopology, RenderPipeline}, render_graph::base::MainPass}, math::Vec2};

use crate::render::material::VoxelMaterial;

#[derive(Bundle)]
pub struct VoxelBundle {
    pub mesh: Handle<Mesh>,
    pub material: Handle<VoxelMaterial>,
    pub main_pass: MainPass,
    pub draw: Draw,
    pub render_pipelines: RenderPipelines,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl Default for VoxelBundle {
    fn default() -> Self {
        Self {
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(super::graph::pipeline::PIPELINE_HANDLE.typed())]),
            mesh: Default::default(),
            material: Default::default(),
            main_pass: Default::default(),
            draw: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
        }
    }
}
