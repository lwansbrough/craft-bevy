use bevy::{prelude::{Query, QueryState, Res, With, World}, render2::{color::Color, render_graph::{Node, NodeRunError, RenderGraphContext, SlotInfo, SlotType}, render_phase::RenderPhase, render_resource::{BufferUsage, BufferVec, ComputePassDescriptor, IndexFormat, LoadOp, Operations, RenderPassColorAttachment, RenderPassDepthStencilAttachment, RenderPassDescriptor}, renderer::RenderContext, view::{ExtractedView, ViewUniformOffset}}, wgpu};

use crate::{VoxelShaders, VoxelVolumeMeta};

pub struct VoxelPhase;

pub struct VoxelPassNode {
    query: QueryState<&'static RenderPhase<VoxelPhase>, With<ExtractedView>>,
}

impl VoxelPassNode {
    pub const IN_COLOR_ATTACHMENT: &'static str = "color_attachment";
    pub const IN_VIEW: &'static str = "view";

    pub fn new(world: &mut World) -> Self {
        Self {
            query: QueryState::new(world),
        }
    }
}

impl Node for VoxelPassNode {
    fn input(&self) -> Vec<SlotInfo> {
        vec![
            SlotInfo::new(Self::IN_VIEW, SlotType::Entity),
            SlotInfo::new(Self::IN_COLOR_ATTACHMENT, SlotType::TextureView)
        ]
    }

    fn update(&mut self, world: &mut World) {
        self.query.update_archetypes(world);
    }

    fn run(
        &self,
        graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World
    ) -> Result<(), NodeRunError> {
        let voxel_shaders = world.get_resource::<VoxelShaders>().unwrap();
        let voxel_volume_meta = world.get_resource::<VoxelVolumeMeta>().unwrap();
        let color_attachment_texture = graph.get_input_texture(Self::IN_COLOR_ATTACHMENT)?;
        
        let broadphase_pass_descriptor = ComputePassDescriptor {
            label: Some("voxel_pass_broadphase"),
        };
        let raytrace_pass_descriptor = ComputePassDescriptor {
            label: Some("voxel_pass_raytrace"),
        };
        let rasterize_pass_descriptor = RenderPassDescriptor {
            label: Some("voxel_pass_render"),
            color_attachments: &[RenderPassColorAttachment {
                view: color_attachment_texture,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(Color::rgb(0.4, 0.4, 0.4).into()),
                    store: true,
                },
            }],
            depth_stencil_attachment: None
        };

        // let view_entity = graph.get_input_entity(Self::IN_VIEW)?;

        let render_texture_bind_group = voxel_volume_meta.render_texture_bind_group.as_ref().unwrap();

        let command_encoder = &mut render_context.command_encoder;

        if voxel_volume_meta.voxel_transforms_bind_group_key.is_some() {
            let broadphase_pass = &mut command_encoder.begin_compute_pass(&broadphase_pass_descriptor);
            
            broadphase_pass.set_pipeline(&voxel_shaders.broadphase_pipeline);
            broadphase_pass.set_bind_group(
                0,
                voxel_volume_meta
                    .voxel_transforms_bind_group
                    .get_value(voxel_volume_meta.voxel_transforms_bind_group_key.unwrap())
                    .unwrap()
                    .value(),
                &[]
            );
            broadphase_pass.set_bind_group(
                1,
                voxel_volume_meta.raybox_intersections_bind_group
                    .get_value(voxel_volume_meta.raybox_intersections_bind_group_key.unwrap())
                    .unwrap()
                    .value(),
                &[]
            );
            broadphase_pass.dispatch(1000, 1000, 1);
        }
        
        if voxel_volume_meta.raybox_intersections_bind_group_key.is_some() {
            let raytrace_pass = &mut command_encoder.begin_compute_pass(&raytrace_pass_descriptor);

            raytrace_pass.set_pipeline(&voxel_shaders.raytrace_pipeline);
            // raytrace_pass.set_bind_group(
            //     0,
            //     view_meta.
            //         .get_value(view_meta..unwrap())
            //         .unwrap(),
            //     &[]
            // );
            raytrace_pass.set_bind_group(
                1,
                voxel_volume_meta.raybox_intersections_bind_group
                    .get_value(voxel_volume_meta.raybox_intersections_bind_group_key.unwrap())
                    .unwrap()
                    .value(),
                &[]
            );

            
            raytrace_pass.set_bind_group(
                2,
                render_texture_bind_group.value(),
                &[]
            );
            raytrace_pass.dispatch(1000, 1000, 1);
        }

        {
            let rasterize_pass = &mut command_encoder.begin_render_pass(&rasterize_pass_descriptor);
            rasterize_pass.set_pipeline(&voxel_shaders.render_pipeline);
            rasterize_pass.set_vertex_buffer(0, *voxel_volume_meta.quad_vertices.buffer().unwrap().slice(..));
            rasterize_pass.set_index_buffer(*voxel_volume_meta.quad_indices.buffer().unwrap().slice(..), IndexFormat::Uint32);
            rasterize_pass.set_bind_group(
                0,
                render_texture_bind_group.value(),
                &[]
            );
            rasterize_pass.draw_indexed(0..3, 0, 0..1);
        }

        Ok(())
    }
}
