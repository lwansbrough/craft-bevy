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

use crate::utilities::Gradient;

pub struct Terrain<'a> {
    chunk_size: i32,
    x: i32,
    y: i32,
    z: i32,
    generator: &'a Select<'a, [f64; 3]>,
    // generator: Perlin
}

impl<'a> Terrain<'a> {
    pub fn new(generator: &'a Select<'a, [f64; 3]>, chunk_size: i32, x: i32, y: i32, z: i32) -> Terrain<'a> {
        Terrain {
            generator,
            chunk_size,
            x,
            y,
            z
        }
    }
}

impl<'a> Source for Terrain<'a> {
    fn sample(&self, x: f32, y: f32, z: f32) -> f32 {
        // println!("x: {}", (self.x as f32 + x as f32));
        // println!("y: {}", (self.y as f32 + y as f32));
        // println!("z: {}", (self.z as f32 + z as f32));
        self.generator.get([
            (self.x as f32 + x as f32) as f64,
            (self.y as f32 + y as f32) as f64,
            (self.z as f32 + z as f32) as f64,
        ]) as f32
    }
}

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

    pub fn generate(&self, chunk_x: i32, chunk_y: i32, chunk_z: i32) -> Mesh {
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

        let terrain = CentralDifference::new_with_epsilon(Terrain::new(&generator, self.chunk_size as i32, chunk_x, chunk_y, chunk_z), 0.1);

        let mut vertices = vec![];
        let mut indices = vec![];
        let mut normals = vec![];

        let mut marching_cubes = MarchingCubes::new(self.chunk_size);

        marching_cubes.extract_with(
            &terrain,
            |v: isosurface::math::Vec3| {
                let n = terrain.sample_normal(v.x, v.y, v.z);
                // if n != isosurface::math::Vec3::zero() {
                //     println!("{:?}", n);
                // }
                vertices.push([v.x, v.y, v.z]);
                normals.push([n.x, n.y, n.z]);
            },
            &mut indices
        );

        // indices.reverse();

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, VertexAttributeValues::Float3(vertices));
        mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, VertexAttributeValues::Float3(normals));
        mesh.set_indices(Some(Indices::U32(indices)));
        mesh
    }
}
