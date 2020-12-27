use bevy::{
    prelude::*,
    render::{renderer::RenderResources, shader::ShaderDefs},
    reflect::TypeUuid
};

#[derive(RenderResources, ShaderDefs, TypeUuid)]
#[uuid = "cd6ef1ac-bce4-405d-9ed3-31744a37d0c1"]
pub struct VoxelMaterial {
}

impl Default for VoxelMaterial {
    fn default() -> Self {
        VoxelMaterial { }
    }
}
