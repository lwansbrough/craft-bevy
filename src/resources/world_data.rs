use bevy::{
    prelude::*,
    render::{
        mesh::*,
        pipeline::PrimitiveTopology
    }
};
use std::{collections::{HashMap, HashSet}, iter::FromIterator};

use crate::VoxelData;

pub struct WorldData {
    center: Option<[i32; 3]>,
    chunks: HashMap<[i32; 3], Vec<VoxelData>>
}

/// Generates terrain features in chunks
impl WorldData {
    pub fn new() -> WorldData {
        WorldData {
            center: None,
            chunks: HashMap::<[i32; 3], Vec<VoxelData>>::with_capacity(27)
        }
    }

    pub fn add_chunk(&mut self, coord: [i32; 3], chunk: Vec<VoxelData>) {
        self.chunks.insert(coord, chunk);
    }

    pub fn move_to(&mut self, coord: [i32; 3]) -> Vec<[i32; 3]> {
        let mut old_coords = HashSet::<[i32; 3]>::new();
        let mut new_coords = HashSet::<[i32; 3]>::new();

        for z in -1..=1 {
            for y in -1..=1 {
                for x in -1..=1 {
                    if let Some(center) = self.center {
                        let current_coord = [center[0] + x, center[1] + y, center[2] + z];
                        old_coords.insert(current_coord);
                    }
                }
            }
        }

        for z in -1..=1 {
            for y in -1..=1 {
                for x in -1..=1 {
                    let current_coord = [coord[0] + x, coord[1] + y, coord[2] + z];

                    if old_coords.contains(&current_coord) {
                        old_coords.remove(&current_coord);
                    } else {
                        new_coords.insert(current_coord);
                    }
                }
            }
        }

        for old_coord in &old_coords {
            self.chunks.remove(old_coord);
        }

        for new_coord in &new_coords {
            self.chunks.insert(*new_coord, vec![]);
        }

        self.center = Some(coord);

        Vec::from_iter(new_coords)
    }
}
