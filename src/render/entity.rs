use bevy::{ecs::{Bundle, ResMut}, math::Vec2, prelude::{Assets, Draw, GlobalTransform, Handle, Mesh, RenderPipelines, StandardMaterial, Transform, shape::{self, Quad}}, render::{prelude::Visible, mesh::VertexAttributeValues, pipeline::{PrimitiveTopology, RenderPipeline}, render_graph::base::MainPass}};

use crate::render::material::VoxelMaterial;
use crate::render::VoxelVolume;

#[derive(Bundle)]
pub struct VoxelBundle {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
    pub voxel_volume: Handle<VoxelVolume>,
    pub main_pass: MainPass,
    pub draw: Draw,
    pub visible: Visible,
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
            visible: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
            voxel_volume: Default::default()
        }
    }
}
