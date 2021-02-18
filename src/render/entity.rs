use bevy::{ecs::{Bundle, ResMut}, math::{Vec2, Vec3}, prelude::{Assets, Draw, GlobalTransform, Handle, Mesh, RenderPipelines, StandardMaterial, Transform, shape::{self, Quad}}, render::{prelude::Visible, mesh::VertexAttributeValues, pipeline::{PrimitiveTopology, RenderPipeline}, render_graph::base::MainPass}};

use crate::{GBufferPass, render::material::VoxelMaterial};
use crate::render::VoxelVolume;

#[derive(Bundle)]
pub struct VoxelBundle {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
    pub voxel_volume: Handle<VoxelVolume>,
    pub gbuffer_pass: GBufferPass,
    pub draw: Draw,
    pub visible: Visible,
    pub render_pipelines: RenderPipelines,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

pub const VOXELS_PER_METER: f32 = 16.0;

impl VoxelBundle {
    pub fn new(meshes: &mut ResMut<Assets<Mesh>>, voxel_volumes: &mut ResMut<Assets<VoxelVolume>>, voxel_volume: VoxelVolume) -> VoxelBundle {
        let size = Vec3::new(
            voxel_volume.size.x / VOXELS_PER_METER,
            voxel_volume.size.y / VOXELS_PER_METER,
            voxel_volume.size.z / VOXELS_PER_METER
        );

        let mesh_handle = meshes.add(Mesh::from(shape::Box::new(size.x, size.y, size.z)));
        let voxel_volume_handle = voxel_volumes.add(voxel_volume);

        Self {
            mesh: mesh_handle,
            voxel_volume: voxel_volume_handle,
            // transform: Transform::from_scale(Vec3::new(1.0, 1.0, 1.0)),
            ..Default::default()
        }
    }
}

impl Default for VoxelBundle {
    fn default() -> Self {
        Self {
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(super::graph::pipeline::PIPELINE_HANDLE.typed())]),
            mesh: Default::default(),
            material: Default::default(),
            gbuffer_pass: Default::default(),
            draw: Default::default(),
            visible: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
            voxel_volume: Default::default()
        }
    }
}
