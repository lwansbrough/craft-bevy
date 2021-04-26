use base::node::MAIN_PASS;
use bevy::{asset::Assets, prelude::{Color, HandleUntyped, StandardMaterial, Texture, World}, reflect::{TypeUuid}, render::{camera::ActiveCameras, pass::{LoadOp, Operations, PassDescriptor, RenderPassColorAttachmentDescriptor, RenderPassDepthStencilAttachmentDescriptor, TextureAttachment}, pipeline::PipelineDescriptor, render_graph::{AssetRenderResourcesNode, CameraNode, PassNode, RenderGraph, RenderResourcesNode, base}, shader::Shader, texture::{Extent3d, SamplerDescriptor, TextureDescriptor, TextureDimension, TextureFormat, TextureUsage}}, transform::prelude::GlobalTransform, window::Windows};

use nodes::TimeNode;

use self::nodes::VoxelVolumeNode;
use self::nodes::TextureNode;

use crate::resources::WindowResizeEventListenerState;

pub mod nodes;
pub mod pipeline;

pub const RENDER_TEXTURE_HANDLE: HandleUntyped = HandleUntyped::weak_from_u64(Texture::TYPE_UUID, 13378939762009864029);

pub mod node {
    pub const TIME: &str = "time";
    pub const TRANSFORM: &str = "transform";
    pub const VOXEL_VOLUME: &str = "voxel_volume";
    pub const TEXTURE_NODE: &str = "texture_node";
    pub const DEPTH_TEXTURE_NODE: &str = "depth_texture_node";
    pub const GBUFFER_PASS: &str = "gbuffer_pass";
    pub const GBUFFER_CAMERA: &str = "gbuffer_camera";
    pub const STANDARD_MATERIAL: &str = "standard_material";
}

pub mod uniform {
    pub const TIME: &str = "Time";
}

#[derive(Default)]
pub struct GBufferPass;

pub(crate) fn add_voxel_graph(world: &mut World) {
    let world = world.cell();
    let mut graph = world.get_resource_mut::<RenderGraph>().unwrap();
    let mut active_cameras = world.get_resource_mut::<ActiveCameras>().unwrap();

    let mut gbuffer_pass_node = PassNode::<&GBufferPass>::new(PassDescriptor {
        color_attachments: vec![RenderPassColorAttachmentDescriptor {
            attachment: TextureAttachment::Input("color_attachment".to_string()),
            resolve_target: None,
            ops: Operations {
                load: LoadOp::Clear(Color::rgb(0.1, 0.2, 0.3)),
                store: true,
            },
        }],
        depth_stencil_attachment: Some(RenderPassDepthStencilAttachmentDescriptor {
            attachment: TextureAttachment::Input("depth".to_string()),
            depth_ops: Some(Operations {
                load: LoadOp::Clear(1.0),
                store: true,
            }),
            stencil_ops: None
            // stencil_ops: Some(Operations {
            //     load: LoadOp::Clear(0),
            //     store: true
            // }),
        }),
        sample_count: 1,
    });
    gbuffer_pass_node.add_camera(node::GBUFFER_CAMERA);

    graph.add_system_node(
        node::TRANSFORM,
        RenderResourcesNode::<GlobalTransform>::new(true)
    );
    graph.add_system_node(
        node::STANDARD_MATERIAL,
        AssetRenderResourcesNode::<StandardMaterial>::new(true),
    );
    graph.add_system_node(node::TIME, TimeNode::new());
    graph.add_system_node(node::VOXEL_VOLUME, VoxelVolumeNode::new());
    graph.add_system_node(node::GBUFFER_CAMERA, CameraNode::new(node::GBUFFER_CAMERA));

    graph.add_node(node::GBUFFER_PASS, gbuffer_pass_node);

    active_cameras.add(node::GBUFFER_CAMERA);

    graph.add_node_edge(node::GBUFFER_CAMERA, node::GBUFFER_PASS).unwrap();

    graph.add_node(
        node::TEXTURE_NODE,
        TextureNode::new(
            TextureDescriptor {
                size: Extent3d::new(1920, 1080, 1),
                mip_level_count: 1,
                sample_count: 1,
                dimension: TextureDimension::D2,
                format: Default::default(),
                usage: TextureUsage::OUTPUT_ATTACHMENT | TextureUsage::SAMPLED,
            },
            Some(SamplerDescriptor::default()),
            Some(RENDER_TEXTURE_HANDLE),
        ),
    );

    graph.add_node(
        node::DEPTH_TEXTURE_NODE,
        TextureNode::new(
            TextureDescriptor {
                size: Extent3d::new(1920, 1080, 1),
                mip_level_count: 1,
                sample_count: 1,
                dimension: TextureDimension::D2,
                format: TextureFormat::Depth32Float,
                usage: TextureUsage::OUTPUT_ATTACHMENT | TextureUsage::SAMPLED,
            },
            None,
            None,
        ),
    );

    graph.add_node_edge(node::TIME, node::GBUFFER_PASS).unwrap();
    graph.add_node_edge(node::VOXEL_VOLUME, node::GBUFFER_PASS).unwrap();
    graph.add_node_edge(node::TRANSFORM, node::GBUFFER_PASS).unwrap();
    graph.add_node_edge(node::TEXTURE_NODE, node::GBUFFER_PASS).unwrap();

    graph.add_slot_edge(node::TEXTURE_NODE, TextureNode::TEXTURE, node::GBUFFER_PASS, "color_attachment").unwrap();
    graph.add_slot_edge(node::DEPTH_TEXTURE_NODE, TextureNode::TEXTURE, node::GBUFFER_PASS, "depth").unwrap();
    
    graph.add_node_edge(node::STANDARD_MATERIAL, base::node::MAIN_PASS).unwrap();
    graph.add_node_edge(node::TRANSFORM, base::node::MAIN_PASS).unwrap();
    graph.add_node_edge(node::GBUFFER_PASS, base::node::MAIN_PASS).unwrap();

    let mut shaders = world.get_resource_mut::<Assets<Shader>>().unwrap();
    let mut pipelines = world.get_resource_mut::<Assets<PipelineDescriptor>>().unwrap();

    let quad_pipeline = pipeline::build_quad_pipeline(&mut shaders);
    let voxel_pipeline = pipeline::build_voxel_pipeline(&mut shaders);
    
    pipelines.set_untracked(
        pipeline::QUAD_PIPELINE_HANDLE,
        quad_pipeline,
    );
    pipelines.set_untracked(
        pipeline::VOXEL_PIPELINE_HANDLE,
        voxel_pipeline,
    );
}
