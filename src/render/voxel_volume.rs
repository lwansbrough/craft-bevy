use std::ops::DerefMut;

use serde::{Serialize, Deserialize};
use bevy::{core::{AsBytes, Byteable, FromBytes}, prelude::*, reflect::TypeUuid, render::{renderer::{BufferInfo, BufferUsage, RenderResource, RenderResourceBinding, RenderResourceContext, RenderResourceId, RenderResources}}, utils::{HashMap, HashSet}};

// use crate::Octree;

pub const VOXELS_PER_METER: f32 = 16.0;

#[derive(Debug, TypeUuid, Serialize)]
#[uuid = "9c15ff5b-12ae-4f62-a489-c3a71ebda138"]
pub struct VoxelVolume {
    pub palette: Vec<Vec4>,
    // pub data: Octree,
    pub size: Vec3,
    pub data: Vec<VoxelData>,
}

// impl AsBytes for VoxelVolume {
//     

//     fn byte_len(&self) -> usize {
//         std::mem::size_of::<[Vec4; 255]>() + std::mem::size_of::<Vec3>() + (std::mem::size_of::<VoxelData>() * self.data.len())
//     }
// }

// impl FromBytes for VoxelVolume {
//     fn from_bytes(bytes: &[u8]) -> Self {
//         let mut offset = 0;
//         let palette = <Vec<Vec4>>::from_bytes(&bytes[0..std::mem::size_of::<[Vec4; 255]>()]);
//         offset = std::mem::size_of::<[Vec4; 255]>();
//         let size = <Vec3>::from_bytes(&bytes[offset..(offset + std::mem::size_of::<Vec3>())]);
//         offset = offset + std::mem::size_of::<Vec3>();
//         let data = <Vec<VoxelData>>::from_bytes(&bytes[offset..]);
        
//         VoxelVolume {
//             palette,
//             size,
//             data
//         }
//     }
// }

impl VoxelVolume {
    pub fn to_bytes(&self) -> Vec<u8> {
        let palette_bytes = self.palette.as_bytes();
        let size_bytes = self.size.as_bytes();
        let data_bytes = self.data.as_bytes();

        let mut buffer = vec![0; self.byte_len()];

        let mut offset = 0;
        buffer[offset..palette_bytes.len()].copy_from_slice(palette_bytes);

        offset = palette_bytes.len();
        buffer[offset..(offset + size_bytes.len())].copy_from_slice(size_bytes);

        offset = palette_bytes.len() + size_bytes.len();
        buffer[offset..(offset + data_bytes.len())].copy_from_slice(data_bytes);

        buffer
    }

    pub fn byte_len(&self) -> usize {
        std::mem::size_of::<Vec4>() * self.palette.len() +
        std::mem::size_of::<Vec3>() +
        std::mem::size_of::<VoxelData>() * self.data.len()
    }

    pub fn voxel(&self, position: Vec3) -> Option<VoxelData> {
        let voxel_data = self.data.get((position.x + self.size.x * (position.y + self.size.z * position.z)) as usize);
        if let Some(voxel) = voxel_data {
            Some(*voxel)
        } else {
            None
        }
    }

    pub fn set_voxel(&mut self, position: Vec3, data: Option<VoxelData>) {
        if let Some(voxel) = data {
            self.data[((position.x + self.size.x * (position.y + self.size.z * position.z)) as usize)] = voxel;
        }
    }
}

impl Default for VoxelVolume {
    fn default() -> VoxelVolume {
        VoxelVolume {
            palette: vec![Vec4::zero(); 255],
            size: Vec3::zero(),
            data: Vec::new(),
            // data: Octree::new(8),
        }
    }
}

#[derive(Debug, Copy, Clone, Serialize)]
pub struct VoxelData {
    pub material: u32,
}

unsafe impl Byteable for VoxelData {}

impl Default for VoxelData {
    fn default() -> Self {
        Self {
            material: 0,
        }
    }
}

pub const VOXEL_VOLUME_BUFFER_ID: u64 = 0;

fn remove_resource_save(
    render_resource_context: &dyn RenderResourceContext,
    handle: &Handle<VoxelVolume>,
    index: u64,
) {
    if let Some(RenderResourceId::Buffer(buffer)) =
        render_resource_context.get_asset_resource(&handle, index)
    {
        render_resource_context.remove_buffer(buffer);
        render_resource_context.remove_asset_resource(handle, index);
    }
}
fn remove_current_voxel_resources(
    render_resource_context: &dyn RenderResourceContext,
    handle: &Handle<VoxelVolume>,
) {
    remove_resource_save(render_resource_context, handle, VOXEL_VOLUME_BUFFER_ID);
}

#[derive(Default)]
pub struct VoxelEntities {
    entities: HashSet<Entity>,
}

#[derive(Default)]
pub struct VoxelResourceProviderState {
    voxel_volume_event_reader: EventReader<AssetEvent<VoxelVolume>>,
    voxel_entities: HashMap<Handle<VoxelVolume>, VoxelEntities>,
}

pub fn voxel_resource_provider_system(
    mut state: Local<VoxelResourceProviderState>,
    render_resource_context: Res<Box<dyn RenderResourceContext>>,
    voxel_volumes: Res<Assets<VoxelVolume>>,
    voxel_events: Res<Events<AssetEvent<VoxelVolume>>>,
    mut queries: QuerySet<(
        Query<&mut RenderPipelines, With<Handle<VoxelVolume>>>,
        Query<(Entity, &Handle<VoxelVolume>, &mut RenderPipelines), Changed<Handle<VoxelVolume>>>,
    )>,
) {
    let mut changed_voxel_volumes = HashSet::default();
    let render_resource_context = &**render_resource_context;
    for event in state.voxel_volume_event_reader.iter(&voxel_events) {
        match event {
            AssetEvent::Created { ref handle } => {
                changed_voxel_volumes.insert(handle.clone_weak());
            }
            AssetEvent::Modified { ref handle } => {
                changed_voxel_volumes.insert(handle.clone_weak());
                remove_current_voxel_resources(render_resource_context, handle);
            }
            AssetEvent::Removed { ref handle } => {
                remove_current_voxel_resources(render_resource_context, handle);
                // if voxel volume was modified and removed in the same update, ignore the modification
                // events are ordered so future modification events are ok
                changed_voxel_volumes.remove(handle);
            }
        }
    }

    // update changed voxel data
    for changed_voxel_volume_handle in changed_voxel_volumes.iter() {
        if let Some(voxel_volume) = voxel_volumes.get(changed_voxel_volume_handle) {
            // TODO: check for individual buffer changes in non-interleaved mode
            let data = voxel_volume.data.as_bytes();
                
            let voxel_buffer = render_resource_context.create_buffer_with_data(
                BufferInfo {
                    buffer_usage: BufferUsage::STORAGE,
                    ..Default::default()
                },
                &data,
            );

            render_resource_context.set_asset_resource(
                changed_voxel_volume_handle,
                RenderResourceId::Buffer(voxel_buffer),
                VOXEL_VOLUME_BUFFER_ID,
            );
            
            if let Some(voxel_entities) = state.voxel_entities.get_mut(changed_voxel_volume_handle) {
                for entity in voxel_entities.entities.iter() {
                    if let Ok(render_pipelines) = queries.q0_mut().get_mut(*entity) {
                        update_entity_voxel_volume(
                            render_resource_context,
                            voxel_volume,
                            changed_voxel_volume_handle,
                            render_pipelines,
                        );
                    }
                }
            }
        }
    }

    // handover buffers to pipeline
    for (entity, handle, render_pipelines) in queries.q1_mut().iter_mut() {
        let voxel_entities = state
            .voxel_entities
            .entry(handle.clone_weak())
            .or_insert_with(VoxelEntities::default);
            voxel_entities.entities.insert(entity);
        if let Some(voxel_volume) = voxel_volumes.get(handle) {
            update_entity_voxel_volume(render_resource_context, voxel_volume, handle, render_pipelines);
        }
    }
}


fn update_entity_voxel_volume(
    render_resource_context: &dyn RenderResourceContext,
    voxel_volume: &VoxelVolume,
    handle: &Handle<VoxelVolume>,
    mut render_pipelines: Mut<RenderPipelines>,
) {
    // for render_pipeline in render_pipelines.pipelines.iter_mut() {
    //     render_pipeline.specialization.primitive_topology = voxel_volume.primitive_topology;
    //     // TODO: don't allocate a new vertex buffer descriptor for every entity
    //     render_pipeline.specialization.vertex_buffer_descriptor =
    //     voxel_volume.get_vertex_buffer_descriptor();
    //     render_pipeline.specialization.index_format = voxel_volume
    //         .indices()
    //         .map(|i| i.into())
    //         .unwrap_or(IndexFormat::Uint32);
    // }

    if let Some(RenderResourceId::Buffer(voxel_volume_buffer_resource)) =
        render_resource_context.get_asset_resource(handle, VOXEL_VOLUME_BUFFER_ID)
    {
        render_pipelines.bindings.set(
            super::super::storage::VOXEL_VOLUME,
            RenderResourceBinding::Buffer {
                buffer: voxel_volume_buffer_resource,
                range: 0..voxel_volume.data.as_bytes().len() as u64,
                dynamic_index: None,
            },
        );
    }
}
