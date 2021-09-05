use bevy::{ecs::bundle::Bundle, math::{Vec2, Vec3}, pbr::PbrBundle, prelude::{Assets, Draw, GlobalTransform, Handle, Mesh, RenderPipelines, ResMut, StandardMaterial, Transform, shape::{self, Quad}}, render::{prelude::Visible, mesh::VertexAttributeValues, pipeline::{PrimitiveTopology, RenderPipeline}, render_graph::base::MainPass}, render2::render_phase::RenderPhase};

use crate::{VOXELS_PER_METER, VoxelPhase};
use crate::render::VoxelVolume;

#[derive(Bundle)]
pub struct VoxelBundle {
    pub voxel_volume: Handle<VoxelVolume>,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub phase: RenderPhase::<VoxelPhase>
}

impl VoxelBundle {
    pub fn new(meshes: &mut ResMut<Assets<Mesh>>, voxel_volumes: &mut ResMut<Assets<VoxelVolume>>, voxel_volume: VoxelVolume) -> VoxelBundle {
        let size = Vec3::new(
            voxel_volume.size.x / VOXELS_PER_METER,
            voxel_volume.size.y / VOXELS_PER_METER,
            voxel_volume.size.z / VOXELS_PER_METER
        );

        let voxel_volume_handle = voxel_volumes.add(voxel_volume);

        Self {
            voxel_volume: voxel_volume_handle,
            transform: Transform::from_scale(size),
            ..Default::default()
        }
    }
}

impl Default for VoxelBundle {
    fn default() -> Self {
        Self {
            transform: Default::default(),
            global_transform: Default::default(),
            voxel_volume: Default::default(),
            phase: RenderPhase::<VoxelPhase>::default()
        }
    }
}
