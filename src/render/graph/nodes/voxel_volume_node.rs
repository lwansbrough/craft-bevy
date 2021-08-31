use bevy::{core_pipeline::Transparent3dPhase, prelude::*, render2::{render_graph::{Node, NodeRunError, RenderGraphContext}, render_phase::{DrawFunctions, RenderPhase}, render_resource::DynamicUniformVec, renderer::{RenderContext, RenderDevice}, texture::{Extent3d, SamplerDescriptor, TextureDescriptor, TextureFormat, TextureUsage}, view::{ExtractedView, ViewMeta}}, utils::{HashMap, HashSet}};
use crate::VoxelVolume;
use crate::VoxelShaders;

#[derive(Debug)]
pub struct VoxelVolumeNode;

impl Node for VoxelVolumeNode {
    fn run(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World
    ) -> Result<(), NodeRunError> {
        Ok(())
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

#[derive(Default)]
pub struct VoxelVolumeMeta {
    transform_uniforms: DynamicUniformVec<Mat4>
}

pub fn prepare_voxel_volumes(
    render_device: Res<RenderDevice>,
    mut voxel_volume_meta: ResMut<VoxelVolumeMeta>,
    mut extracted_voxel_volumes: ResMut<ExtractedVoxelVolumes>,
) {
    voxel_volume_meta
        .transform_uniforms
        .reserve_and_clear(extracted_voxel_volumes.voxel_volumes.len(), &render_device);
    
    for extracted_voxel_volume in extracted_voxel_volumes.iter_mut() {
        extracted_voxel_volume.transform_binding_offset = voxel_volume_meta.transform_uniforms.push(extracted_voxel_volume.transform);
    }

    voxel_volume_meta
        .transform_uniforms
        .write_to_staging_buffer(&render_device);
}

pub fn queue_voxel_volumes(
    mut commands: Commands,
    raw_functions: Res<DrawFunctions>,
    render_device: Res<RenderDevice>,
    voxel_shaders: Res<VoxelShaders>,
    voxel_volume_meta: ResMut<VoxelVolumeMeta>,
    view_meta: Res<ViewMeta>,
    mut extracted_voxel_volumes: ResMut<ExtractedVoxelVolumes>,
    mut views: Query<(
        Entity,
        &ExtractedView,
        &mut RenderPhase<Transparent3dPhase>,
    )>,
) {
    let voxel_volume_meta = voxel_volume_meta.into_inner();

    if view_meta.uniforms.is_empty() {
        return;
    }

    if extracted_voxel_volumes.voxel_volumes.is_empty() {
        return;
    }

    
}

// #[derive(Default)]
// pub struct VoxelEntities {
//     entities: HashSet<Entity>,
// }

// #[derive(Default)]
// pub struct VoxelVolumeNodeState {
//     command_queue: CommandQueue,
//     voxel_entities: HashMap<Handle<VoxelVolume>, VoxelEntities>,
//     voxel_staging_buffers: HashMap<Handle<VoxelVolume>, BufferId>,
//     voxel_buffers: HashMap<Handle<VoxelVolume>, BufferId>,
// }

// pub const VOXEL_VOLUME_BUFFER_ID: u64 = 0;
// pub const VOXEL_VOLUME_STAGING_BUFFER_ID: u64 = 1;

// fn remove_resource_save(
//     render_resource_context: &dyn RenderResourceContext,
//     handle: &Handle<VoxelVolume>,
//     index: u64,
// ) {
//     if let Some(RenderResourceId::Buffer(buffer)) =
//         render_resource_context.get_asset_resource(&handle, index)
//     {
//         render_resource_context.remove_buffer(buffer);
//         render_resource_context.remove_asset_resource(handle, index);
//     }
// }
// fn remove_current_voxel_resources(
//     render_resource_context: &dyn RenderResourceContext,
//     handle: &Handle<VoxelVolume>,
// ) {
//     remove_resource_save(render_resource_context, handle, VOXEL_VOLUME_BUFFER_ID);
// }

// // Reference: https://github.com/bevyengine/bevy/blob/b17f8a4bce5551b418654fffb1fe97ff0f9852f0/crates/bevy_render/src/render_graph/nodes/render_resources_node.rs#L620

// pub fn voxel_node_system(
//     mut state: Local<VoxelVolumeNodeState>,
//     render_resource_context: Res<Box<dyn RenderResourceContext>>,
//     mut render_resource_bindings: ResMut<RenderResourceBindings>,
//     // TODO: this write on RenderResourceBindings will prevent this system from running in parallel with other systems that do the same
//     voxel_volumes: Res<Assets<VoxelVolume>>,
//     mut voxel_events: EventReader<AssetEvent<VoxelVolume>>,
//     // mut queries: QuerySet<(
//     //     Query<&mut RenderPipelines, With<Handle<VoxelVolume>>>,
//     //     Query<(Entity, &Handle<VoxelVolume>, &mut RenderPipelines), Changed<Handle<VoxelVolume>>>,
//     // )>,
//     mut queries: Query<(&Handle<VoxelVolume>, &mut RenderPipelines)>,
// ) {
//     let state = &mut state;
//     let mut changed_voxel_volumes = HashSet::default();
//     let render_resource_context = &**render_resource_context;

//     for event in voxel_events.iter() {
//         match event {
//             AssetEvent::Created { ref handle } => {
//                 changed_voxel_volumes.insert(handle.clone_weak());
//             }
//             AssetEvent::Modified { ref handle } => {
//                 changed_voxel_volumes.insert(handle.clone_weak());
//                 remove_current_voxel_resources(render_resource_context, handle);
//             }
//             AssetEvent::Removed { ref handle } => {
//                 remove_current_voxel_resources(render_resource_context, handle);
//                 // if voxel volume was modified and removed in the same update, ignore the modification
//                 // events are ordered so future modification events are ok
//                 changed_voxel_volumes.remove(handle);
//             }
//         }
//     }

//     let mut changed = HashSet::<Handle<VoxelVolume>>::default();

//     // update changed voxel data
//     for changed_voxel_volume_handle in changed_voxel_volumes.iter() {

//         if changed.contains(changed_voxel_volume_handle) {
//             continue;
//         }
        
//         if let Some(voxel_volume) = voxel_volumes.get(changed_voxel_volume_handle) {
//             changed.insert(changed_voxel_volume_handle.clone_weak());
//             // TODO: check for individual buffer changes in non-interleaved mode
//             let size = voxel_volume.byte_len();

//             if state.voxel_staging_buffers.contains_key(changed_voxel_volume_handle) {
//                 let staging_buffer = *state.voxel_staging_buffers.get(changed_voxel_volume_handle).unwrap();
//                 render_resource_context.map_buffer(staging_buffer, BufferMapMode::Write)
//             } else {
//                 let voxel_buffer = render_resource_context.create_buffer(
//                     BufferInfo {
//                         size,
//                         buffer_usage: BufferUsage::COPY_DST | BufferUsage::STORAGE,
//                         ..Default::default()
//                     }
//                 );
//                 state.voxel_buffers.insert(changed_voxel_volume_handle.clone_weak(), voxel_buffer);
    
//                 render_resource_context.set_asset_resource(
//                     changed_voxel_volume_handle,
//                     RenderResourceId::Buffer(voxel_buffer),
//                     VOXEL_VOLUME_BUFFER_ID,
//                 );
    
//                 let staging_buffer = render_resource_context.create_buffer(BufferInfo {
//                     size,
//                     buffer_usage: BufferUsage::COPY_SRC | BufferUsage::MAP_WRITE,
//                     mapped_at_creation: true,
//                 });
//                 state.voxel_staging_buffers.insert(changed_voxel_volume_handle.clone_weak(), staging_buffer);

//                 render_resource_context.set_asset_resource(
//                     changed_voxel_volume_handle,
//                     RenderResourceId::Buffer(staging_buffer),
//                     VOXEL_VOLUME_STAGING_BUFFER_ID,
//                 );
//             };

//             if let Some(voxel_volume) = voxel_volumes.get(changed_voxel_volume_handle) {
//                 let voxels_bytes = &voxel_volume.to_bytes();
//                 let voxels_size = voxels_bytes.len();
    
//                 let voxel_staging_buffer = *state.voxel_staging_buffers.get(changed_voxel_volume_handle).unwrap();
//                 let voxel_buffer = *state.voxel_buffers.get(changed_voxel_volume_handle).unwrap();
    
//                 render_resource_context.write_mapped_buffer(
//                     voxel_staging_buffer,
//                     0..voxels_size as u64,
//                     &mut |data, _renderer| {
//                         data[0..voxels_size].copy_from_slice(&voxels_bytes[..]);
//                     },
//                 );
//                 render_resource_context.unmap_buffer(voxel_staging_buffer);
    
//                 state.command_queue.copy_buffer_to_buffer(
//                     voxel_staging_buffer,
//                     0,
//                     voxel_buffer,
//                     0,
//                     voxels_size as u64,
//                 );
    
//                 for (voxel_volume_handle, mut render_pipelines) in queries.iter_mut() {
//                     if voxel_volume_handle == changed_voxel_volume_handle {
//                         render_pipelines.bindings.set(
//                             super::super::storage::VOXEL_VOLUME,
//                             RenderResourceBinding::Buffer {
//                                 buffer: voxel_buffer,
//                                 range: 0..voxels_size as u64,
//                                 dynamic_index: None,
//                             },
//                         );
//                     }
//                 }
//             }
//         }
//     }

//     // disabled because changes aren't detected on Changed
//     // for (entity, handle, mut render_pipelines) in queries.q1_mut().iter_mut() {
//     //     let voxel_entities = state
//     //         .voxel_entities
//     //         .entry(handle.clone_weak())
//     //         .or_insert_with(VoxelEntities::default);
        
//     //     voxel_entities.entities.insert(entity);

//     //     if let Some(voxel_volume) = voxel_volumes.get(handle) {
//     //         println!("writing buffer");
//     //         let voxels_bytes = &voxel_volume.to_bytes();
//     //         let voxels_size = voxels_bytes.len();

//     //         let voxel_staging_buffer = *state.voxel_staging_buffers.get(handle).unwrap();
//     //         let voxel_buffer = *state.voxel_buffers.get(handle).unwrap();

//     //         render_resource_context.write_mapped_buffer(
//     //             voxel_staging_buffer,
//     //             0..voxels_size as u64,
//     //             &mut |data, _renderer| {
//     //                 data[0..voxels_size].copy_from_slice(&voxels_bytes[..]);
//     //             },
//     //         );
//     //         render_resource_context.unmap_buffer(voxel_staging_buffer);

//     //         state.command_queue.copy_buffer_to_buffer(
//     //             voxel_staging_buffer,
//     //             0,
//     //             voxel_buffer,
//     //             0,
//     //             voxels_size as u64,
//     //         );

//     //         render_pipelines.bindings.set(
//     //             super::super::storage::VOXEL_VOLUME,
//     //             RenderResourceBinding::Buffer {
//     //                 buffer: voxel_buffer,
//     //                 range: 0..voxels_size as u64,
//     //                 dynamic_index: None,
//     //             },
//     //         );
//     //     }
//     // }
// }
