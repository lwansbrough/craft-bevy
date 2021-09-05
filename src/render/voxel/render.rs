use bevy::{core::{Pod}, core_pipeline::{ClearColor, ViewDepthTexture}, math::Mat4, prelude::{Assets, Commands, Entity, FromWorld, GlobalTransform, Handle, Query, Res, ResMut, World}, render2::{camera::{CameraPlugin, ExtractedCamera, ExtractedCameraNames}, render_graph::{Node, NodeRunError, RenderGraphContext, SlotInfo, SlotType, SlotValue}, render_phase::{DrawFunctions, RenderPhase, TrackedRenderPass}, render_resource::{BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingResource, BindingType, BlendComponent, BlendFactor, BlendOperation, BlendState, BufferBindingType, BufferId, BufferSize, BufferUsage, BufferVec, ColorTargetState, ColorWrite, ComputePipeline, ComputePipelineDescriptor, DynamicUniformVec, Extent3d, FragmentState, FrontFace, InputStepMode, MultisampleState, PipelineLayoutDescriptor, PolygonMode, PrimitiveState, PrimitiveTopology, RenderPipeline, RenderPipelineDescriptor, SamplerDescriptor, ShaderStage, StorageTextureAccess, TextureDescriptor, TextureDimension, TextureFormat, TextureUsage, TextureViewDescriptor, TextureViewDimension, VertexAttribute, VertexBufferLayout, VertexFormat, VertexState}, renderer::{RenderContext, RenderDevice}, shader::Shader, texture::{GpuImage, Image}, view::{ExtractedView, ExtractedWindows, ViewMeta, ViewUniform}}, utils::{HashMap, HashSet, slab::{FrameSlabMap, FrameSlabMapKey}}};
use crate::{VoxelPhase, VoxelVolume};

pub struct VoxelShaders {
    pub broadphase_pipeline: ComputePipeline,
    pub raytrace_pipeline: ComputePipeline,
    pub render_pipeline: RenderPipeline,
    pub view_layout: BindGroupLayout,
    pub voxels_layout: BindGroupLayout,
    pub render_texture_compute_layout: BindGroupLayout,
    pub render_texture_quad_layout: BindGroupLayout

}

impl FromWorld for VoxelShaders {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.get_resource::<RenderDevice>().unwrap();
        let broadphase_shader = Shader::from_wgsl(include_str!("broadphase.wgsl"));
        let raytrace_shader = Shader::from_wgsl(include_str!("raytrace.wgsl"));
        let quad_shader = Shader::from_wgsl(include_str!("quad.wgsl"));

        let broadphase_shader_module = render_device.create_shader_module(&broadphase_shader);
        let raytrace_shader_module = render_device.create_shader_module(&raytrace_shader);
        let quad_shader_module = render_device.create_shader_module(&quad_shader);

        let view_layout = render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("View Layout"),
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStage::COMPUTE,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: true,
                    min_binding_size: BufferSize::new(std::mem::size_of::<ViewUniform>() as u64),
                },
                count: None,
            }]
        });

        let voxels_layout = render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Voxels Layout"),
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStage::COMPUTE,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Storage {
                            read_only: true
                        },
                        has_dynamic_offset: true,
                        min_binding_size: None,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStage::COMPUTE,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Storage {
                            read_only: false
                        },
                        has_dynamic_offset: true,
                        min_binding_size: None,
                    },
                    count: None,
                }
            ]
        });

        let render_texture_compute_layout = render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Render Texture Compute Layout"),
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStage::COMPUTE,
                    ty: BindingType::StorageTexture {
                        access: StorageTextureAccess::WriteOnly,
                        format: TextureFormat::Rgba8Unorm,
                        view_dimension: TextureViewDimension::D2
                    },
                    count: None
                }
            ]
        });

        let render_texture_quad_layout = render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Render Texture Quad Layout"),
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStage::FRAGMENT,
                    ty: BindingType::StorageTexture {
                        access: StorageTextureAccess::ReadOnly,
                        format: TextureFormat::Rgba8Unorm,
                        view_dimension: TextureViewDimension::D2
                    },
                    count: None
                }
            ]
        });

        let broadphase_pipeline_layout = render_device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: None,
            push_constant_ranges: &[],
            bind_group_layouts: &[&view_layout, &voxels_layout],
        });

        let raytrace_pipeline_layout = render_device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: None,
            push_constant_ranges: &[],
            bind_group_layouts: &[&view_layout, &voxels_layout, &render_texture_compute_layout],
        });

        let render_pipeline_layout = render_device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: None,
            push_constant_ranges: &[],
            bind_group_layouts: &[&render_texture_quad_layout]
        });

        // Project rays into the scene and collect the voxel volumes each ray collides with
        let broadphase_pipeline = render_device.create_compute_pipeline(&ComputePipelineDescriptor {
            label: Some("Broadphase"),
            layout: Some(&broadphase_pipeline_layout),
            module: &broadphase_shader_module,
            entry_point: "broadphase"
        });

        // Trace through each volume from the broad phase until we have a solid colour 
        let raytrace_pipeline = render_device.create_compute_pipeline(&ComputePipelineDescriptor {
            label: Some("Raytrace"),
            layout: Some(&raytrace_pipeline_layout),
            module: &raytrace_shader_module,
            entry_point: "raytrace"
        });

        let render_pipeline = render_device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Rasterization"),
            layout: Some(&render_pipeline_layout),
            vertex: VertexState {
                buffers: &[VertexBufferLayout {
                    array_stride: 4,
                    step_mode: InputStepMode::Vertex,
                    attributes: &[
                        VertexAttribute {
                            format: VertexFormat::Uint32,
                            offset: 0,
                            shader_location: 0,
                        }
                    ],
                }],
                module: &quad_shader_module,
                entry_point: "vertex",
            },
            fragment: Some(FragmentState {
                module: &quad_shader_module,
                entry_point: "fragment",
                targets: &[ColorTargetState {
                    format: TextureFormat::Bgra8UnormSrgb,
                    blend: Some(BlendState {
                        color: BlendComponent {
                            src_factor: BlendFactor::SrcAlpha,
                            dst_factor: BlendFactor::OneMinusSrcAlpha,
                            operation: BlendOperation::Add,
                        },
                        alpha: BlendComponent {
                            src_factor: BlendFactor::One,
                            dst_factor: BlendFactor::One,
                            operation: BlendOperation::Add,
                        },
                    }),
                    write_mask: ColorWrite::ALL,
                }],
            }),
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: PolygonMode::Fill,
                clamp_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: MultisampleState::default(),
        });

        VoxelShaders {
            broadphase_pipeline,
            raytrace_pipeline,
            render_pipeline,
            view_layout,
            voxels_layout,
            render_texture_compute_layout,
            render_texture_quad_layout
        }
    }
}

struct ExtractedVoxelVolume {
    transform: Mat4,
    voxel_volume: Handle<VoxelVolume>,
    transform_binding_offset: u32
}

pub struct ExtractedVoxelVolumes {
    voxel_volumes: Vec<ExtractedVoxelVolume>
}

pub fn extract_voxel_volumes(
    mut commands: Commands,
    voxel_volumes: Res<Assets<VoxelVolume>>,
    query: Query<(&GlobalTransform, &Handle<VoxelVolume>)>,
) {
    let mut extracted_voxel_volumes = Vec::new();
    for (transform, voxel_volume_handle) in query.iter() {
        if !voxel_volumes.contains(voxel_volume_handle) {
            continue;
        }

        extracted_voxel_volumes.push(ExtractedVoxelVolume {
            transform: transform.compute_matrix(),
            voxel_volume: voxel_volume_handle.clone_weak(),
            transform_binding_offset: 0
        });
    }

    commands.insert_resource(ExtractedVoxelVolumes {
        voxel_volumes: extracted_voxel_volumes
    });
}

// #[repr(C)]
// #[derive(Copy, Clone, Pod)]
// struct QuadVertex {
//     pub position: [f32; 2],
//     pub uv: [f32; 2],
// }

pub struct VoxelVolumeMeta {
    pub transform_uniforms: DynamicUniformVec<Mat4>,
    pub voxel_transforms_bind_group: FrameSlabMap<BufferId, BindGroup>,
    pub voxel_transforms_bind_group_key: Option<FrameSlabMapKey<BufferId, BindGroup>>,
    pub raybox_intersections_bind_group: FrameSlabMap<BufferId, BindGroup>,
    pub raybox_intersections_bind_group_key: Option<FrameSlabMapKey<BufferId, BindGroup>>,
    pub render_texture_bind_group: Option<BindGroup>,
    pub render_texture: Option<GpuImage>,
    pub quad_vertices: BufferVec<u32>,
    pub quad_indices: BufferVec<u32>
}

impl Default for VoxelVolumeMeta {
    fn default() -> Self {
        Self {
            quad_vertices: BufferVec::new(BufferUsage::VERTEX),
            quad_indices: BufferVec::new(BufferUsage::INDEX),
            transform_uniforms: DynamicUniformVec::<Mat4>::default(),
            voxel_transforms_bind_group: FrameSlabMap::<BufferId, BindGroup>::default(),
            voxel_transforms_bind_group_key: None,
            raybox_intersections_bind_group: FrameSlabMap::<BufferId, BindGroup>::default(),
            raybox_intersections_bind_group_key: None,
            render_texture_bind_group: None,
            render_texture: None,
        }
    }
}

pub fn prepare_voxel_volumes(
    render_device: Res<RenderDevice>,
    mut voxel_volume_meta: ResMut<VoxelVolumeMeta>,
    mut extracted_voxel_volumes: ResMut<ExtractedVoxelVolumes>,
) {
    voxel_volume_meta
        .transform_uniforms
        .reserve_and_clear(extracted_voxel_volumes.voxel_volumes.len(), &render_device);
    
    for extracted_voxel_volume in extracted_voxel_volumes.voxel_volumes.iter_mut() {
        extracted_voxel_volume.transform_binding_offset = voxel_volume_meta.transform_uniforms.push(extracted_voxel_volume.transform);
    }

    voxel_volume_meta
        .transform_uniforms
        .write_to_staging_buffer(&render_device);
}

pub fn prepare_render_texture(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
    voxel_shaders: Res<VoxelShaders>,
    mut voxel_volume_meta: ResMut<VoxelVolumeMeta>,
) {
    if voxel_volume_meta.render_texture.is_none() {
        let texture = render_device.create_texture(&TextureDescriptor {
            label: Some("Full Screen Quad"),
            size: Extent3d {
                height: 1000,
                width: 1000,
                depth_or_array_layers: 1
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8Unorm,
            usage: TextureUsage::STORAGE,
        });

        let sampler = render_device.create_sampler(&SamplerDescriptor {
            label: Some("Full Screen Quad"),
            ..Default::default()
        });

        let texture_view = texture.create_view(&TextureViewDescriptor {
            format: Some(TextureFormat::Rgba8Unorm),
            ..Default::default()
        });
        
        voxel_volume_meta.render_texture = Some(GpuImage {
            texture,
            texture_view,
            sampler,
        });
    }

    if voxel_volume_meta.render_texture_bind_group.is_none() {
        let gpu_image = voxel_volume_meta.render_texture.as_ref().unwrap();
        voxel_volume_meta.render_texture_bind_group = Some(render_device.create_bind_group(&BindGroupDescriptor {
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(&gpu_image.texture_view),
                }
            ],
            label: None,
            layout: &voxel_shaders.render_texture_quad_layout,
        }));
    }

    voxel_volume_meta.quad_vertices.reserve_and_clear(3, &render_device);
    voxel_volume_meta.quad_indices.reserve_and_clear(3, &render_device);

    // voxel_volume_meta.quad_vertices.push(QuadVertex {
    //     position: [1.0, 1.0],
    //     uv: [1.0, 0.0]
    // });
    // voxel_volume_meta.quad_vertices.push(QuadVertex {
    //     position: [1.0, -1.0],
    //     uv: [1.0, 1.0]
    // });
    // voxel_volume_meta.quad_vertices.push(QuadVertex {
    //     position: [-1.0, -1.0],
    //     uv: [0.0, 1.0]
    // });
    // voxel_volume_meta.quad_vertices.push(QuadVertex {
    //     position: [1.0, 1.0],
    //     uv: [1.0, 0.0]
    // });
    // voxel_volume_meta.quad_vertices.push(QuadVertex {
    //     position: [-1.0, -1.0],
    //     uv: [0.0, 1.0]
    // });
    // voxel_volume_meta.quad_vertices.push(QuadVertex {
    //     position: [-1.0, 1.0],
    //     uv: [0.0, 0.0]
    // });

    voxel_volume_meta.quad_vertices.push(0);
    voxel_volume_meta.quad_vertices.push(1);
    voxel_volume_meta.quad_vertices.push(2);

    voxel_volume_meta.quad_indices.push(0);
    voxel_volume_meta.quad_indices.push(1);
    voxel_volume_meta.quad_indices.push(2);

    voxel_volume_meta.quad_vertices.write_to_staging_buffer(&render_device);
    voxel_volume_meta.quad_indices.write_to_staging_buffer(&render_device);
}

pub fn queue_voxel_volumes(
    mut commands: Commands,
    raw_functions: Res<DrawFunctions>,
    render_device: Res<RenderDevice>,
    voxel_shaders: Res<VoxelShaders>,
    mut voxel_volume_meta: ResMut<VoxelVolumeMeta>,
    view_meta: Res<ViewMeta>,
    mut extracted_voxel_volumes: ResMut<ExtractedVoxelVolumes>,
    mut views: Query<(
        Entity,
        &ExtractedView,
        &mut RenderPhase<VoxelPhase>,
    )>,
) {
    let voxel_volume_meta = voxel_volume_meta.into_inner();

    if view_meta.uniforms.is_empty() {
        return;
    }

    if extracted_voxel_volumes.voxel_volumes.is_empty() {
        return;
    }

    let transform_uniforms = &voxel_volume_meta.transform_uniforms;
    voxel_volume_meta.voxel_transforms_bind_group.next_frame();
    voxel_volume_meta.voxel_transforms_bind_group_key =
        Some(voxel_volume_meta.voxel_transforms_bind_group.get_or_insert_with(
            transform_uniforms.uniform_buffer().unwrap().id(),
            || {
                render_device.create_bind_group(&BindGroupDescriptor {
                    entries: &[BindGroupEntry {
                        binding: 0,
                        resource: transform_uniforms.binding(),
                    }],
                    label: None,
                    layout: &voxel_shaders.voxels_layout,
                })
            },
        ));

    voxel_volume_meta.raybox_intersections_bind_group.next_frame();
    voxel_volume_meta.raybox_intersections_bind_group_key =
        Some(voxel_volume_meta.raybox_intersections_bind_group.get_or_insert_with(
            transform_uniforms.uniform_buffer().unwrap().id(),
            || {
                render_device.create_bind_group(&BindGroupDescriptor {
                    entries: &[BindGroupEntry {
                        binding: 1,
                        resource: transform_uniforms.binding(),
                    }],
                    label: None,
                    layout: &voxel_shaders.voxels_layout,
                })
            },
        ));

    
}

pub struct VoxelVolumeNode;

impl Node for VoxelVolumeNode {
    fn run(
        &self,
        graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World
    ) -> Result<(), NodeRunError> {
        let extracted_cameras = world.get_resource::<ExtractedCameraNames>().unwrap();
        let extracted_windows = world.get_resource::<ExtractedWindows>().unwrap();

        if let Some(camera_3d) = extracted_cameras.entities.get(CameraPlugin::CAMERA_3D) {
            let extracted_camera = world.entity(*camera_3d).get::<ExtractedCamera>().unwrap();
            let extracted_window = extracted_windows.get(&extracted_camera.window_id).unwrap();
            let swap_chain_texture = extracted_window.swap_chain_frame.as_ref().unwrap().clone();
            
            graph.run_sub_graph(
                crate::draw_voxels_graph::NAME,
                vec![
                    SlotValue::Entity(*camera_3d),
                    SlotValue::TextureView(swap_chain_texture),
                ],
            )?;
        }

        Ok(())
    }
}
