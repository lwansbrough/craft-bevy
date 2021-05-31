use bevy::{ecs::bundle::Bundle, math::{Vec2, Vec3}, pbr::PbrBundle, prelude::{Assets, Draw, GlobalTransform, Handle, Mesh, RenderPipelines, ResMut, StandardMaterial, Transform, shape::{self, Quad}}, render::{prelude::Visible, mesh::VertexAttributeValues, pipeline::{PrimitiveTopology, RenderPipeline}, render_graph::base::MainPass}};

use crate::{GBufferPass, VOXELS_PER_METER, render::material::VoxelMaterial};
use crate::render::VoxelVolume;

#[derive(Bundle)]
pub struct QuadBundle {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
    pub main_pass: MainPass,
    pub draw: Draw,
    pub visible: Visible,
    pub render_pipelines: RenderPipelines,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl Default for QuadBundle {
    fn default() -> Self {
        Self {
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                super::graph::pipeline::QUAD_PIPELINE_HANDLE.typed()
            )]),
            mesh: Default::default(),
            material: Default::default(),
            main_pass: Default::default(),
            draw: Default::default(),
            visible: Default::default(),
            transform: Default::default(),
            global_transform: Default::default()
        }
    }
}

impl QuadBundle {
    pub fn new(meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<StandardMaterial>>) -> QuadBundle {
        let texture_handle = crate::RENDER_TEXTURE_HANDLE.typed();

        QuadBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(2.0, 2.0)))),
            material: materials.add(StandardMaterial {
                base_color_texture: Some(texture_handle.clone()),
                ..Default::default()
            }),
            // visible: Visible {
            //     is_transparent: true,
            //     ..Default::default()
            // },
            ..Default::default()
        }
    }
}

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
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(super::graph::pipeline::VOXEL_PIPELINE_HANDLE.typed())]),
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
