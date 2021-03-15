use std::{cmp::Ordering, collections::{BTreeMap, BTreeSet}, mem, ops::DerefMut};

use bevy::{math::{Vec3Mask, Vec3Swizzles, Vec4Swizzles}, prelude::*, render::{camera::{ActiveCameras, Camera}, render_graph::base::camera}};
use bincode::Options;
use crate::{VOXELS_PER_METER, VoxelData, VoxelVolume, resources::*};

const MAX_RAY_STEPS: i32 = 512;

/// This system prints out all mouse events as they come in
pub fn player_focus(
    commands: &mut Commands,
    mut voxel_volumes: ResMut<Assets<VoxelVolume>>,
    mut player_focus: ResMut<PlayerFocus>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    voxel_entities_query: Query<(Entity, &Handle<VoxelVolume>, &GlobalTransform)>,
    active_cameras: Res<ActiveCameras>
    // Equipped item
    // Player Focus (3D point being looked at, voxel volume handle, voxel coord within volume, material, type (terrain, building, object))
) {
    // TODO:
    // 1. Find entities in view frustum
    // 2. Ray box intersect all entities
    // 3. Collect intersections (point + entity/volume) and sort them from nearest to farthest
    // 4. Iterate over intersections from near to far, ray marching through each volume from the
    // intersection point until we find a non-empty voxel

    let (camera, camera_transform) = if let Some(entity) = active_cameras.get(camera::CAMERA_3D) {
        camera_query.get(entity).unwrap()
    } else {
        return;
    };

    let mut intersections = Vec::<(Vec3, Entity, &GlobalTransform, &Handle<VoxelVolume>)>::new();
    
    for (entity, voxel_volume_handle, entity_transform) in voxel_entities_query.iter() {
        if let Some(voxel_volume) = voxel_volumes.get(voxel_volume_handle) {
            // 1.
            // https://stackoverflow.com/questions/12836967/extracting-view-frustum-planes-gribb-hartmann-method
            let (is_intersecting, intersection_point) = is_intersecting(camera, camera_transform, &*entity_transform, voxel_volume.size);
            
            if is_intersecting {
                // voxel_volume.palette[1] = Vec4::new(1.0, 0.0, 0.0, 1.0);
                intersections.push((intersection_point, entity, entity_transform, voxel_volume_handle));
            }
        }
    }

    intersections.sort_by(|a, b| a.0.length().partial_cmp(&b.0.length()).unwrap());

    for (point, entity, entity_transform, voxel_volume_handle) in intersections {
        let camera_view_matrix = camera.projection_matrix * camera_transform.compute_matrix().inverse();
        let voxel_volume = if let Some(voxel_volume) = voxel_volumes.get(voxel_volume_handle) {
            voxel_volume
        } else {
            continue;
        };

        let camera_to_model = entity_transform.compute_matrix().inverse() * camera_view_matrix.inverse();
        let camera_ray_origin = Vec3::zero();
        let model_ray_origin = (camera_to_model * Vec4::new(camera_ray_origin.x, camera_ray_origin.y, camera_ray_origin.z, 1.0)).xyz();
        let model_ray_direction = (point - model_ray_origin).normalize();
        
        let scale = voxel_volume.size / 16.0;
        // Convert the local space position into voxel space, ie. [-1, 1] -> [0, 32]
        let scaled_position: Vec3 = point * voxel_volume.size / scale;
        let ray_direction = model_ray_direction;

        // Do ray marching, starting at the front face position in voxel space
        let ray_position: Vec3 = scaled_position + 0.001 * ray_direction;
        let mut map_pos = ray_position;

        let delta_dist: Vec3 = (Vec3::new(ray_direction.length(), ray_direction.length(), ray_direction.length()) / ray_direction).abs();
        
        let ray_step: Vec3 = ray_direction.signum();

        let mut side_dist: Vec3 = (ray_direction.signum() * (map_pos.signum() - ray_position) + (ray_direction.signum() * 0.5) + Vec3::new(0.5, 0.5, 0.5)) * delta_dist; 
        
        let mut voxel: Option<VoxelData> = None;
        let mut mask: Vec3Mask;

        for _ in 0..MAX_RAY_STEPS {
            if voxel.is_some() || map_pos.cmpge(voxel_volume.size).any() || map_pos.cmple(Vec3::zero()).any() {
                break;
            }

            voxel = voxel_volume.voxel(map_pos);

            mask = side_dist.cmple(side_dist.yzx().min(side_dist.zxy()));
            side_dist += mask.select(Vec3::one(), Vec3::zero()) * delta_dist;
            map_pos += mask.select(Vec3::one(), Vec3::zero()) * ray_step;
        }

        if let Some(v) = voxel {
            if let Some(mut voxel_volume) = voxel_volumes.get_mut(voxel_volume_handle) {
                player_focus.entity = Some(entity);
                player_focus.voxel_coord = Some(map_pos.floor());
                voxel_volume.set_voxel(Vec3::new(31.0, 31.0, 31.0), Some(VoxelData { material: 2 }));
                // println!("focus: {:?} ({:?})", v.material, player_focus.voxel_coord);
            }

            // let voxel_volume: Option<VoxelVolume> = voxel_volumes.remove(voxel_volume_handle);

            // if let Some(voxel_volume) = voxel_volume {
            //     voxel_volumes.set(voxel_volume_handle, voxel_volume);
            // }

            break;
        }
    }
}

// fn get_frustum_planes(camera: &Camera, transform: &GlobalTransform) -> Vec<(Vec3, Vec3)> {
//     let mat = (camera.projection_matrix * transform.compute_matrix().inverse()).to_cols_array_2d();

//     let left = Vec3::new(
//         mat[0][3] + mat[0][0],
//         mat[1][3] + mat[1][0],
//         mat[2][3] + mat[2][0]
//     );

//     let right = Vec3::new(
//         mat[0][3] - mat[0][0],
//         mat[1][3] - mat[1][0],
//         mat[2][3] - mat[2][0]
//     );

//     let bottom = Vec3::new(
//         mat[0][3] + mat[0][1],
//         mat[1][3] + mat[1][1],
//         mat[2][3] + mat[2][1]
//     );

//     let top = Vec3::new(
//         mat[0][3] - mat[0][1],
//         mat[1][3] - mat[1][1],
//         mat[2][3] - mat[2][1]
//     );

//     let near = Vec3::new(
//         mat[0][3] + mat[0][2],
//         mat[1][3] + mat[1][2],
//         mat[2][3] + mat[2][2]
//     );

//     let far = Vec3::new(
//         mat[0][3] - mat[0][2],
//         mat[1][3] - mat[1][2],
//         mat[2][3] - mat[2][2]
//     );

//     let planes = Vec::with_capacity(6);
//     planes.push((Vec3::new(-1.0, 0.0, 0.0), left));
//     planes.push((Vec3::new(1.0, 0.0, 0.0), right));
//     planes.push((Vec3::new(0.0, -1.0, 0.0), bottom));
//     planes.push((Vec3::new(0.0, 1.0, 0.0), top));
//     planes.push((Vec3::new(0.0, 0.0, 1.0), near));
//     planes.push((Vec3::new(0.0, 0.0, -1.0), far));

//     planes
// }

// fn is_intersecting(camera: &Camera, camera_transform: &GlobalTransform, entity_transform: &GlobalTransform, volume_size: Vec3) -> (bool, Vec3) {
//     let camera_view_matrix = camera.projection_matrix * camera_transform.compute_matrix().inverse();

//     let ray_origin = Vec3::zero();
//     let ray_direction = camera_view_matrix.z_axis;

//     let camera_to_model = entity_transform.compute_matrix().inverse() * camera_view_matrix.inverse();
//     let model_ray_direction = camera_to_model * ray_direction;
//     let model_ray_origin = (camera_to_model * Vec4::new(ray_origin.x, ray_origin.y, ray_origin.z, 1.0)).xyz();

//     let min_x = -volume_size.x / 2.0 / VOXELS_PER_METER;
//     let max_x = volume_size.x / 2.0 / VOXELS_PER_METER;
//     let min_y = -volume_size.y / 2.0 / VOXELS_PER_METER;
//     let max_y = volume_size.y / 2.0 / VOXELS_PER_METER;
//     let min_z = -volume_size.z / 2.0 / VOXELS_PER_METER;
//     let max_z = volume_size.z / 2.0 / VOXELS_PER_METER;

//     let mut tmin = (min_x - model_ray_origin.x) / -model_ray_direction.x;
//     let mut tmax = (max_x - model_ray_origin.x) / -model_ray_direction.x;
 
//     if tmin > tmax {
//         mem::swap(&mut tmin, &mut tmax);
//     }
 
//     let mut tymin = (min_y - model_ray_origin.y) / -model_ray_direction.y; 
//     let mut tymax = (max_y - model_ray_origin.y) / -model_ray_direction.y; 
 
//     if tymin > tymax {
//         mem::swap(&mut tymin, &mut tymax);
//     }
 
//     if (tmin > tymax) || (tymin > tmax) {
//         return (false, Vec3::zero());
//     }
 
//     if tymin > tmin {
//         tmin = tymin;
//     }
 
//     if tymax < tmax {
//         tmax = tymax;
//     }
 
//     let mut tzmin = (min_z - model_ray_origin.z) / -model_ray_direction.z; 
//     let mut tzmax = (max_z - model_ray_origin.z) / -model_ray_direction.z; 

//     if tzmin > tzmax {
//         mem::swap(&mut tzmin, &mut tzmax);
//     }
 
//     if (tmin > tzmax) || (tzmin > tmax) { 
//         return (false, Vec3::zero());
//     }
 
//     if tzmin > tmin {
//         tmin = tzmin;
//     }

//     if tzmax < tmax {
//         tmax = tzmax;
//     }
 
//     (true, ray_direction.xyz() * Vec3::new(tmin, tymin, tzmin) + ray_origin)
// }

fn is_intersecting(camera: &Camera, camera_transform: &GlobalTransform, entity_transform: &GlobalTransform, volume_size: Vec3) -> (bool, Vec3) {
    let camera_view_matrix = camera.projection_matrix * camera_transform.compute_matrix().inverse();

    let ray_origin = Vec3::zero();
    let ray_direction = camera_view_matrix.z_axis;

    let camera_to_model = entity_transform.compute_matrix().inverse() * camera_view_matrix.inverse();
    let model_ray_direction = camera_to_model * ray_direction;
    let model_ray_direction_inv = -model_ray_direction;
    let model_ray_origin = (camera_to_model * Vec4::new(ray_origin.x, ray_origin.y, ray_origin.z, 1.0)).xyz();

    let min = [
        -volume_size.x / 2.0 / VOXELS_PER_METER,
        -volume_size.y / 2.0 / VOXELS_PER_METER,
        -volume_size.z / 2.0 / VOXELS_PER_METER
    ];

    let max = [
        volume_size.x / 2.0 / VOXELS_PER_METER,
        volume_size.y / 2.0 / VOXELS_PER_METER,
        volume_size.z / 2.0 / VOXELS_PER_METER
    ];

    let mut t1 = (min[0] - model_ray_origin[0]) * model_ray_direction_inv[0];
    let mut t2 = (max[0] - model_ray_origin[0]) * model_ray_direction_inv[0];

    let mut tmin = t1.min(t2);
    let mut tmax = t1.max(t2);

    for i in 1..3 {
        t1 = (min[i] - model_ray_origin[i]) * model_ray_direction_inv[i];
        t2 = (max[i] - model_ray_origin[i]) * model_ray_direction_inv[i];

        tmin = tmin.min(t1.min(t2));
        tmax = tmax.max(t1.max(t2));
    }

    (tmax > tmin.max(0.0), ray_direction.xyz() * tmin + ray_origin)
}
