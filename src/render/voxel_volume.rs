use serde::{Serialize, Deserialize};
use bevy::{core::{AsBytes, Byteable, FromBytes}, prelude::*, reflect::TypeUuid, render::{renderer::{BufferInfo, BufferUsage, RenderResource, RenderResourceBinding, RenderResourceContext, RenderResourceId, RenderResources}}, utils::{HashMap, HashSet}};

// use crate::Octree;

#[derive(Debug, TypeUuid, Serialize)]
#[uuid = "9c15ff5b-12ae-4f62-a489-c3a71ebda138"]
pub struct VoxelVolume {
    pub palette: Vec<Vec4>,
    // pub data: Octree,
    pub size: Vec3,
    pub data: Vec<VoxelData>,
}

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

    pub fn data_len(&self) -> usize {
        std::mem::size_of::<VoxelData>() * self.data.len()
    }

    pub fn palette_len(&self) -> usize {
        std::mem::size_of::<Vec4>() * self.palette.len()
    }

    pub fn byte_len(&self) -> usize {
        std::mem::size_of::<Vec4>() * self.palette.len() +
        std::mem::size_of::<Vec3>() +
        std::mem::size_of::<VoxelData>() * self.data.len()
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

#[derive(Debug, Clone, Serialize)]
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
