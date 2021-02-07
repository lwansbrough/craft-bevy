use bevy::{
    prelude::*,
    render::{
        mesh::*,
        pipeline::PrimitiveTopology
    }
};
use noise::{
    *,
    utils::*
};
use isosurface::source::Source;
use isosurface::source::HermiteSource;
use isosurface::marching_cubes::MarchingCubes;
use isosurface::source::CentralDifference;

use crate::{VoxelData, VoxelVolume, utilities::Gradient};

pub struct WorldGenerator {
    chunk_size: usize
}

/// Generates terrain features in chunks
impl WorldGenerator {
    pub fn new(chunk_size: usize) -> WorldGenerator {
        
        WorldGenerator {
            chunk_size
        }
    }

    pub fn chunk_size(&self) -> usize {
        self.chunk_size
    }

    pub fn generate(&self, chunk_x: i32, chunk_y: i32, chunk_z: i32) -> VoxelVolume {
        let ground_gradient = Gradient::new()
            .set_x_start(0.0)
            .set_y_stop(1.0);

        let lowland_shape_fractal = Billow::new()
            .set_octaves(2)
            .set_frequency(0.25);

        let lowland_autocorrect = Clamp::<[f64; 3]>::new(&lowland_shape_fractal) // TODO: Should use AutoCorrect not Clamp
            .set_bounds(-1.0, 1.0);

        let lowland_scale = ScaleBias::new(&lowland_autocorrect)
            .set_bias(-0.45)
            .set_scale(0.125);

        let lowland_y_scale = ScalePoint::new(&lowland_scale)
            .set_y_scale(0.0);
        
        let lowland_terrain = Displace::new(
            &ground_gradient,
            Constant::new(0.0),
            &lowland_y_scale,
            Constant::new(0.0),
            Constant::new(0.0)
        );

        let highland_shape_fractal = Fbm::new()
            .set_octaves(4)
            .set_frequency(2.0);

        let highland_autocorrect = Clamp::<[f64; 3]>::new(&highland_shape_fractal) // TODO: Should use AutoCorrect not Clamp
            .set_bounds(-1.0, 1.0);

        let highland_scale = ScaleBias::new(&highland_autocorrect)
            .set_bias(0.0)
            .set_scale(0.25);

        let highland_y_scale = ScalePoint::new(&highland_scale)
            .set_y_scale(0.0);

        let highland_terrain = Displace::new(
            &ground_gradient,
            Constant::new(0.0),
            &highland_y_scale,
            Constant::new(0.0),
            Constant::new(0.0)
        );

        let mountain_shape_fractal = RidgedMulti::new()
            .set_octaves(4)
            .set_frequency(2.0);
        
        let mountain_autocorrect = Clamp::new(&mountain_shape_fractal) // TODO: Should use AutoCorrect not Clamp
            .set_bounds(-1.0, 1.0);

        let mountain_scale = ScaleBias::new(&mountain_autocorrect)
            .set_bias(0.15)
            .set_scale(0.45);

        let mountain_y_scale = ScalePoint::new(&mountain_scale)
            .set_y_scale(0.25);

        let mountain_terrain = Displace::new(
            &ground_gradient,
            Constant::new(0.0),
            &mountain_y_scale,
            Constant::new(0.0),
            Constant::new(0.0)
        );

        let terrain_type_fractal = Fbm::new()
            .set_octaves(3)
            .set_frequency(0.125);

        let terrain_type_autocorrect = Clamp::new(&terrain_type_fractal) // TODO: Should use AutoCorrect not Clamp
            .set_bounds(-1.0, 1.0);

        let terrain_type_y_scale = ScalePoint::new(&terrain_type_autocorrect)
            .set_y_scale(0.0);

        let terrain_type_cache = Cache::new(&terrain_type_y_scale);

        let highland_mountain_select = Select::new(&highland_terrain, &mountain_terrain, &terrain_type_cache)
            .set_falloff(0.2);

        let highland_lowland_select = Select::new(&lowland_terrain, &highland_mountain_select, &terrain_type_cache)
            .set_falloff(0.15);

        let highland_lowland_select_cache = Cache::new(&highland_lowland_select);

        let source1 = Constant::new(0.0);
        let source2 = Constant::new(1.0);
        let generator = Select::new(&source1, &source2, &highland_lowland_select_cache);

        let mut voxels = Vec::with_capacity(self.chunk_size * self.chunk_size * self.chunk_size);
        let mut palette = vec![Vec4::zero(); 255];
        palette[1] = Vec4::new(0.086, 0.651, 0.780, 1.0);  // Blue
        palette[2] = Vec4::new(0.900, 0.894, 0.737, 1.0); // Yellow
        palette[3] = Vec4::new(0.196, 0.659, 0.321, 1.0); // Green
        palette[4] = Vec4::new(0.545, 0.271, 0.075, 1.0); // Brown
        palette[5] = Vec4::new(0.502, 0.502, 0.502, 1.0); // Grey
        palette[6] = Vec4::new(1.0, 0.98, 0.98, 1.0);     // White

        let scale = 1.0/32.0;

        for z in 0..self.chunk_size as u32 {
            for y in 0..self.chunk_size as u32 {
                for x in 0..self.chunk_size as u32 {
                    let coord = [
                        scale * ((chunk_x as f64 * self.chunk_size as f64) + x as f64),
                        scale * ((chunk_y as f64 * self.chunk_size as f64) + y as f64),
                        scale * ((chunk_z as f64 * self.chunk_size as f64) + z as f64)
                    ];

                    let value = generator.get(coord);
    
                    if value == 0.0 {
                        voxels.push(VoxelData { material: 0 });
                        continue;
                    }

                    let global_y = chunk_y as u32 * self.chunk_size as u32 + y as u32;

                    voxels.push(VoxelData {
                        material: match global_y {
                            y if y < 25 => 1, // Blue
                            y if y < 27 => 2, // Yellow
                            y if y < 35 => 3, // Green
                            y if y < 50 => 4, // Brown
                            y if y < 70 => 5, // Grey
                            _ => 6,           // White
                        },
                    });
                }
                
            }
        }

        VoxelVolume {
            data: voxels,
            palette,
            size: Vec3::new(self.chunk_size as f32, self.chunk_size as f32, self.chunk_size as f32)
        }
    }
}
