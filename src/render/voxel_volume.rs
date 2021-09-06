use std::ops::DerefMut;

use serde::{Serialize, Deserialize};
use bevy::{core::{FromBytes}, prelude::*, reflect::TypeUuid, render::{renderer::{BufferInfo, RenderResource, RenderResourceBinding, RenderResourceContext, RenderResourceId, RenderResources}}, render2::{render_asset::{RenderAsset, RenderAssetPlugin}, render_resource::{Buffer, BufferInitDescriptor, BufferUsage}, renderer::{RenderDevice, RenderQueue}}, utils::{HashMap, HashSet}};

// use crate::Octree;

pub const VOXELS_PER_METER: f32 = 16.0;

#[derive(Debug, TypeUuid, Clone)]
#[uuid = "9c15ff5b-12ae-4f62-a489-c3a71ebda138"]
pub struct VoxelVolume {
    pub palette: Vec<u32>,
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
        // let palette_bytes = self.palette.as_bytes();
        // let size_bytes = self.size.as_bytes();
        // let data_bytes = self.data.as_bytes();

        let mut buffer = vec![0; self.byte_len()];

        // let mut offset = 0;
        // buffer[offset..palette_bytes.len()].copy_from_slice(palette_bytes);

        // offset = palette_bytes.len();
        // buffer[offset..(offset + size_bytes.len())].copy_from_slice(size_bytes);

        // offset = palette_bytes.len() + size_bytes.len();
        // buffer[offset..(offset + data_bytes.len())].copy_from_slice(data_bytes);

        buffer
    }

    pub fn byte_len(&self) -> usize {
        std::mem::size_of::<u32>() * self.palette.len() +
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
                0x00000000u32, 0xffffffffu32, 0xffccffffu32, 0xff99ffffu32, 0xff66ffffu32, 0xff33ffffu32, 0xff00ffffu32, 0xffffccffu32, 0xffccccffu32, 0xff99ccffu32, 0xff66ccffu32, 0xff33ccffu32, 0xff00ccffu32, 0xffff99ffu32, 0xffcc99ffu32, 0xff9999ffu32,
                0xff6699ffu32, 0xff3399ffu32, 0xff0099ffu32, 0xffff66ffu32, 0xffcc66ffu32, 0xff9966ffu32, 0xff6666ffu32, 0xff3366ffu32, 0xff0066ffu32, 0xffff33ffu32, 0xffcc33ffu32, 0xff9933ffu32, 0xff6633ffu32, 0xff3333ffu32, 0xff0033ffu32, 0xffff00ffu32,
                0xffcc00ffu32, 0xff9900ffu32, 0xff6600ffu32, 0xff3300ffu32, 0xff0000ffu32, 0xffffffccu32, 0xffccffccu32, 0xff99ffccu32, 0xff66ffccu32, 0xff33ffccu32, 0xff00ffccu32, 0xffffccccu32, 0xffccccccu32, 0xff99ccccu32, 0xff66ccccu32, 0xff33ccccu32,
                0xff00ccccu32, 0xffff99ccu32, 0xffcc99ccu32, 0xff9999ccu32, 0xff6699ccu32, 0xff3399ccu32, 0xff0099ccu32, 0xffff66ccu32, 0xffcc66ccu32, 0xff9966ccu32, 0xff6666ccu32, 0xff3366ccu32, 0xff0066ccu32, 0xffff33ccu32, 0xffcc33ccu32, 0xff9933ccu32,
                0xff6633ccu32, 0xff3333ccu32, 0xff0033ccu32, 0xffff00ccu32, 0xffcc00ccu32, 0xff9900ccu32, 0xff6600ccu32, 0xff3300ccu32, 0xff0000ccu32, 0xffffff99u32, 0xffccff99u32, 0xff99ff99u32, 0xff66ff99u32, 0xff33ff99u32, 0xff00ff99u32, 0xffffcc99u32,
                0xffcccc99u32, 0xff99cc99u32, 0xff66cc99u32, 0xff33cc99u32, 0xff00cc99u32, 0xffff9999u32, 0xffcc9999u32, 0xff999999u32, 0xff669999u32, 0xff339999u32, 0xff009999u32, 0xffff6699u32, 0xffcc6699u32, 0xff996699u32, 0xff666699u32, 0xff336699u32,
                0xff006699u32, 0xffff3399u32, 0xffcc3399u32, 0xff993399u32, 0xff663399u32, 0xff333399u32, 0xff003399u32, 0xffff0099u32, 0xffcc0099u32, 0xff990099u32, 0xff660099u32, 0xff330099u32, 0xff000099u32, 0xffffff66u32, 0xffccff66u32, 0xff99ff66u32,
                0xff66ff66u32, 0xff33ff66u32, 0xff00ff66u32, 0xffffcc66u32, 0xffcccc66u32, 0xff99cc66u32, 0xff66cc66u32, 0xff33cc66u32, 0xff00cc66u32, 0xffff9966u32, 0xffcc9966u32, 0xff999966u32, 0xff669966u32, 0xff339966u32, 0xff009966u32, 0xffff6666u32,
                0xffcc6666u32, 0xff996666u32, 0xff666666u32, 0xff336666u32, 0xff006666u32, 0xffff3366u32, 0xffcc3366u32, 0xff993366u32, 0xff663366u32, 0xff333366u32, 0xff003366u32, 0xffff0066u32, 0xffcc0066u32, 0xff990066u32, 0xff660066u32, 0xff330066u32,
                0xff000066u32, 0xffffff33u32, 0xffccff33u32, 0xff99ff33u32, 0xff66ff33u32, 0xff33ff33u32, 0xff00ff33u32, 0xffffcc33u32, 0xffcccc33u32, 0xff99cc33u32, 0xff66cc33u32, 0xff33cc33u32, 0xff00cc33u32, 0xffff9933u32, 0xffcc9933u32, 0xff999933u32,
                0xff669933u32, 0xff339933u32, 0xff009933u32, 0xffff6633u32, 0xffcc6633u32, 0xff996633u32, 0xff666633u32, 0xff336633u32, 0xff006633u32, 0xffff3333u32, 0xffcc3333u32, 0xff993333u32, 0xff663333u32, 0xff333333u32, 0xff003333u32, 0xffff0033u32,
                0xffcc0033u32, 0xff990033u32, 0xff660033u32, 0xff330033u32, 0xff000033u32, 0xffffff00u32, 0xffccff00u32, 0xff99ff00u32, 0xff66ff00u32, 0xff33ff00u32, 0xff00ff00u32, 0xffffcc00u32, 0xffcccc00u32, 0xff99cc00u32, 0xff66cc00u32, 0xff33cc00u32,
                0xff00cc00u32, 0xffff9900u32, 0xffcc9900u32, 0xff999900u32, 0xff669900u32, 0xff339900u32, 0xff009900u32, 0xffff6600u32, 0xffcc6600u32, 0xff996600u32, 0xff666600u32, 0xff336600u32, 0xff006600u32, 0xffff3300u32, 0xffcc3300u32, 0xff993300u32,
                0xff663300u32, 0xff333300u32, 0xff003300u32, 0xffff0000u32, 0xffcc0000u32, 0xff990000u32, 0xff660000u32, 0xff330000u32, 0xff0000eeu32, 0xff0000ddu32, 0xff0000bbu32, 0xff0000aau32, 0xff000088u32, 0xff000077u32, 0xff000055u32, 0xff000044u32,
                0xff000022u32, 0xff000011u32, 0xff00ee00u32, 0xff00dd00u32, 0xff00bb00u32, 0xff00aa00u32, 0xff008800u32, 0xff007700u32, 0xff005500u32, 0xff004400u32, 0xff002200u32, 0xff001100u32, 0xffee0000u32, 0xffdd0000u32, 0xffbb0000u32, 0xffaa0000u32,
                0xff880000u32, 0xff770000u32, 0xff550000u32, 0xff440000u32, 0xff220000u32, 0xff110000u32, 0xffeeeeeeu32, 0xffddddddu32, 0xffbbbbbbu32, 0xffaaaaaau32, 0xff888888u32, 0xff777777u32, 0xff555555u32, 0xff444444u32, 0xff222222u32, 0xff111111u32
            ],
            size: Vec3::ZERO,
            data: Vec::new(),
            // data: Octree::new(8),
        }
    }
}

#[derive(Debug, Copy, Clone)]
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

pub struct VoxelVolumePlugin;

impl Plugin for VoxelVolumePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RenderAssetPlugin::<VoxelVolume>::default())
            .add_asset::<VoxelVolume>();
    }
}


#[derive(Debug, Clone)]
pub struct GpuVoxelVolume {
    pub buffer: Buffer,
}

impl RenderAsset for VoxelVolume {
    type ExtractedAsset = VoxelVolume;
    type PreparedAsset = GpuVoxelVolume;

    fn extract_asset(&self) -> Self::ExtractedAsset {
        self.clone()
    }

    fn prepare_asset(
        material: Self::ExtractedAsset,
        render_device: &RenderDevice,
        _render_queue: &RenderQueue,
    ) -> Self::PreparedAsset {
        let mut flags = StandardMaterialFlags::NONE;
        if material.base_color_texture.is_some() {
            flags |= StandardMaterialFlags::BASE_COLOR_TEXTURE;
        }
        if material.emissive_texture.is_some() {
            flags |= StandardMaterialFlags::EMISSIVE_TEXTURE;
        }
        if material.metallic_roughness_texture.is_some() {
            flags |= StandardMaterialFlags::METALLIC_ROUGHNESS_TEXTURE;
        }
        if material.occlusion_texture.is_some() {
            flags |= StandardMaterialFlags::OCCLUSION_TEXTURE;
        }
        if material.double_sided {
            flags |= StandardMaterialFlags::DOUBLE_SIDED;
        }
        if material.unlit {
            flags |= StandardMaterialFlags::UNLIT;
        }
        let value = StandardMaterialUniformData {
            base_color: material.base_color.as_rgba_linear().into(),
            emissive: material.emissive.into(),
            roughness: material.perceptual_roughness,
            metallic: material.metallic,
            reflectance: material.reflectance,
            flags: flags.bits,
        };
        let value_std140 = value.as_std140();

        let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            label: None,
            usage: BufferUsage::UNIFORM | BufferUsage::COPY_DST,
            contents: value_std140.as_bytes(),
        });
        GpuVoxelVolume {
            buffer,
        }
    }
}