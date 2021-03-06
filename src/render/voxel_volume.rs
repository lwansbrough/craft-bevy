use bevy::{
    prelude::*,
    core::Byteable,
    render::{
        renderer::{RenderResource, RenderResources},
    },
    reflect::TypeUuid,
};

#[derive(Debug, RenderResources, RenderResource, TypeUuid)]
#[uuid = "9c15ff5b-12ae-4f62-a489-c3a71ebda138"]
pub struct VoxelVolume {
    pub palette: Vec<Vec4>,
    #[render_resources(buffer)]
    pub data: Vec<VoxelData>,
    pub size: Vec3,
}

unsafe impl Byteable for VoxelVolume {}

impl Default for VoxelVolume {
    fn default() -> VoxelVolume {
        VoxelVolume {
            palette: Vec::new(),
            data: Vec::new(),
            size: Vec3::zero()
        }
    }
}

#[derive(Debug)]
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