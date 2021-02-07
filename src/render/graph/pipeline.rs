use bevy::{
    asset::{Assets, HandleUntyped},
    render::{
        pipeline::{
            BlendDescriptor, BlendFactor, BlendOperation, ColorStateDescriptor, ColorWrite,
            CompareFunction, CullMode, DepthStencilStateDescriptor, FrontFace, PipelineDescriptor,
            RasterizationStateDescriptor, StencilStateDescriptor, StencilStateFaceDescriptor
        },
        shader::{Shader, ShaderStage, ShaderStages}, texture::TextureFormat,
    },
    reflect::{TypeUuid}
};

pub const PIPELINE_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(PipelineDescriptor::TYPE_UUID, 12585943984739023957);

pub(crate) fn build_pipeline(shaders: &mut Assets<Shader>) -> PipelineDescriptor {
    PipelineDescriptor {
        depth_stencil_state: Some(DepthStencilStateDescriptor {
            format: TextureFormat::Depth32Float,
            depth_write_enabled: true,
            depth_compare: CompareFunction::Less,
            stencil: StencilStateDescriptor {
                front: StencilStateFaceDescriptor::IGNORE,
                back: StencilStateFaceDescriptor::IGNORE,
                read_mask: 0,
                write_mask: 0,
            },
        }),
        ..PipelineDescriptor::default_config(ShaderStages {
            vertex: shaders.add(Shader::from_glsl(
                ShaderStage::Vertex,
                include_str!("voxel.vs"),
            )),
            fragment: Some(shaders.add(Shader::from_glsl(
                ShaderStage::Fragment,
                include_str!("voxel.fs"),
            ))),
        })
    }
}
