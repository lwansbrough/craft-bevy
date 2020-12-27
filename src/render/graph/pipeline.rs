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
    PipelineDescriptor::default_config(ShaderStages {
        vertex: shaders.add(Shader::from_glsl(
            ShaderStage::Vertex,
            include_str!("test.vs"),
        )),
        fragment: Some(shaders.add(Shader::from_glsl(
            ShaderStage::Fragment,
            include_str!("test.fs"),
        ))),
    })
    // PipelineDescriptor {
    //     rasterization_state: Some(RasterizationStateDescriptor {
    //         front_face: FrontFace::Ccw,
    //         cull_mode: CullMode::Back,
    //         depth_bias: 0,
    //         depth_bias_slope_scale: 0.0,
    //         depth_bias_clamp: 0.0,
    //         clamp_depth: false,
    //     }),
    //     depth_stencil_state: Some(DepthStencilStateDescriptor {
    //         format: TextureFormat::Depth32Float,
    //         depth_write_enabled: true,
    //         depth_compare: CompareFunction::Less,
    //         stencil: StencilStateDescriptor {
    //             front: StencilStateFaceDescriptor::IGNORE,
    //             back: StencilStateFaceDescriptor::IGNORE,
    //             read_mask: 0,
    //             write_mask: 0,
    //         },
    //     }),
    //     color_states: vec![ColorStateDescriptor {
    //         format: TextureFormat::default(),
    //         color_blend: BlendDescriptor {
    //             src_factor: BlendFactor::SrcAlpha,
    //             dst_factor: BlendFactor::OneMinusSrcAlpha,
    //             operation: BlendOperation::Add,
    //         },
    //         alpha_blend: BlendDescriptor {
    //             src_factor: BlendFactor::One,
    //             dst_factor: BlendFactor::One,
    //             operation: BlendOperation::Add,
    //         },
    //         write_mask: ColorWrite::ALL,
    //     }],
    //     ..PipelineDescriptor::new(ShaderStages {
    //         vertex: shaders.add(Shader::from_glsl(
    //             ShaderStage::Vertex,
    //             include_str!("test.vs"),
    //         )),
    //         fragment: Some(shaders.add(Shader::from_glsl(
    //             ShaderStage::Fragment,
    //             include_str!("test.fs"),
    //         ))),
    //     })
    // }
}
