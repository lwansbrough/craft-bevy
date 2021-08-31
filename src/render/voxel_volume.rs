use std::ops::DerefMut;

use serde::{Serialize, Deserialize};
use bevy::{core::{FromBytes}, prelude::*, reflect::TypeUuid, render::{renderer::{BufferInfo, BufferUsage, RenderResource, RenderResourceBinding, RenderResourceContext, RenderResourceId, RenderResources}}, utils::{HashMap, HashSet}};

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
            palette: vec![
                0x00000000, 0xffffffff, 0xffccffff, 0xff99ffff, 0xff66ffff, 0xff33ffff, 0xff00ffff, 0xffffccff, 0xffccccff, 0xff99ccff, 0xff66ccff, 0xff33ccff, 0xff00ccff, 0xffff99ff, 0xffcc99ff, 0xff9999ff,
                0xff6699ff, 0xff3399ff, 0xff0099ff, 0xffff66ff, 0xffcc66ff, 0xff9966ff, 0xff6666ff, 0xff3366ff, 0xff0066ff, 0xffff33ff, 0xffcc33ff, 0xff9933ff, 0xff6633ff, 0xff3333ff, 0xff0033ff, 0xffff00ff,
                0xffcc00ff, 0xff9900ff, 0xff6600ff, 0xff3300ff, 0xff0000ff, 0xffffffcc, 0xffccffcc, 0xff99ffcc, 0xff66ffcc, 0xff33ffcc, 0xff00ffcc, 0xffffcccc, 0xffcccccc, 0xff99cccc, 0xff66cccc, 0xff33cccc,
                0xff00cccc, 0xffff99cc, 0xffcc99cc, 0xff9999cc, 0xff6699cc, 0xff3399cc, 0xff0099cc, 0xffff66cc, 0xffcc66cc, 0xff9966cc, 0xff6666cc, 0xff3366cc, 0xff0066cc, 0xffff33cc, 0xffcc33cc, 0xff9933cc,
                0xff6633cc, 0xff3333cc, 0xff0033cc, 0xffff00cc, 0xffcc00cc, 0xff9900cc, 0xff6600cc, 0xff3300cc, 0xff0000cc, 0xffffff99, 0xffccff99, 0xff99ff99, 0xff66ff99, 0xff33ff99, 0xff00ff99, 0xffffcc99,
                0xffcccc99, 0xff99cc99, 0xff66cc99, 0xff33cc99, 0xff00cc99, 0xffff9999, 0xffcc9999, 0xff999999, 0xff669999, 0xff339999, 0xff009999, 0xffff6699, 0xffcc6699, 0xff996699, 0xff666699, 0xff336699,
                0xff006699, 0xffff3399, 0xffcc3399, 0xff993399, 0xff663399, 0xff333399, 0xff003399, 0xffff0099, 0xffcc0099, 0xff990099, 0xff660099, 0xff330099, 0xff000099, 0xffffff66, 0xffccff66, 0xff99ff66,
                0xff66ff66, 0xff33ff66, 0xff00ff66, 0xffffcc66, 0xffcccc66, 0xff99cc66, 0xff66cc66, 0xff33cc66, 0xff00cc66, 0xffff9966, 0xffcc9966, 0xff999966, 0xff669966, 0xff339966, 0xff009966, 0xffff6666,
                0xffcc6666, 0xff996666, 0xff666666, 0xff336666, 0xff006666, 0xffff3366, 0xffcc3366, 0xff993366, 0xff663366, 0xff333366, 0xff003366, 0xffff0066, 0xffcc0066, 0xff990066, 0xff660066, 0xff330066,
                0xff000066, 0xffffff33, 0xffccff33, 0xff99ff33, 0xff66ff33, 0xff33ff33, 0xff00ff33, 0xffffcc33, 0xffcccc33, 0xff99cc33, 0xff66cc33, 0xff33cc33, 0xff00cc33, 0xffff9933, 0xffcc9933, 0xff999933,
                0xff669933, 0xff339933, 0xff009933, 0xffff6633, 0xffcc6633, 0xff996633, 0xff666633, 0xff336633, 0xff006633, 0xffff3333, 0xffcc3333, 0xff993333, 0xff663333, 0xff333333, 0xff003333, 0xffff0033,
                0xffcc0033, 0xff990033, 0xff660033, 0xff330033, 0xff000033, 0xffffff00, 0xffccff00, 0xff99ff00, 0xff66ff00, 0xff33ff00, 0xff00ff00, 0xffffcc00, 0xffcccc00, 0xff99cc00, 0xff66cc00, 0xff33cc00,
                0xff00cc00, 0xffff9900, 0xffcc9900, 0xff999900, 0xff669900, 0xff339900, 0xff009900, 0xffff6600, 0xffcc6600, 0xff996600, 0xff666600, 0xff336600, 0xff006600, 0xffff3300, 0xffcc3300, 0xff993300,
                0xff663300, 0xff333300, 0xff003300, 0xffff0000, 0xffcc0000, 0xff990000, 0xff660000, 0xff330000, 0xff0000ee, 0xff0000dd, 0xff0000bb, 0xff0000aa, 0xff000088, 0xff000077, 0xff000055, 0xff000044,
                0xff000022, 0xff000011, 0xff00ee00, 0xff00dd00, 0xff00bb00, 0xff00aa00, 0xff008800, 0xff007700, 0xff005500, 0xff004400, 0xff002200, 0xff001100, 0xffee0000, 0xffdd0000, 0xffbb0000, 0xffaa0000,
                0xff880000, 0xff770000, 0xff550000, 0xff440000, 0xff220000, 0xff110000, 0xffeeeeee, 0xffdddddd, 0xffbbbbbb, 0xffaaaaaa, 0xff888888, 0xff777777, 0xff555555, 0xff444444, 0xff222222, 0xff111111
            ],
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
    voxel_entities: HashMap<Handle<VoxelVolume>, VoxelEntities>,
}

pub fn voxel_resource_provider_system(
    mut state: Local<VoxelResourceProviderState>,
    render_resource_context: Res<Box<dyn RenderResourceContext>>,
    voxel_volumes: Res<Assets<VoxelVolume>>,
    mut voxel_events: EventReader<AssetEvent<VoxelVolume>>,
    mut queries: QuerySet<(
        Query<&mut RenderPipelines, With<Handle<VoxelVolume>>>,
        Query<(Entity, &Handle<VoxelVolume>, &mut RenderPipelines), Changed<Handle<VoxelVolume>>>,
    )>,
) {
    let mut changed_voxel_volumes = HashSet::default();
    let render_resource_context = &**render_resource_context;
    for event in voxel_events.iter() {
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
    // // for render_pipeline in render_pipelines.pipelines.iter_mut() {
    // //     render_pipeline.specialization.primitive_topology = voxel_volume.primitive_topology;
    // //     // TODO: don't allocate a new vertex buffer descriptor for every entity
    // //     render_pipeline.specialization.vertex_buffer_descriptor =
    // //     voxel_volume.get_vertex_buffer_descriptor();
    // //     render_pipeline.specialization.index_format = voxel_volume
    // //         .indices()
    // //         .map(|i| i.into())
    // //         .unwrap_or(IndexFormat::Uint32);
    // // }

    // if let Some(RenderResourceId::Buffer(voxel_volume_buffer_resource)) =
    //     render_resource_context.get_asset_resource(handle, VOXEL_VOLUME_BUFFER_ID)
    // {
    //     render_pipelines.bindings.set(
    //         super::super::storage::VOXEL_VOLUME,
    //         RenderResourceBinding::Buffer {
    //             buffer: voxel_volume_buffer_resource,
    //             range: 0..voxel_volume.data.as_bytes().len() as u64,
    //             dynamic_index: None,
    //         },
    //     );
    // }
}
