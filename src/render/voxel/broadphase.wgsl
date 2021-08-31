[[block]]
struct View {
    view_proj: mat4x4<f32>;
    world_position: vec3<f32>;
};

[[block]]
struct VoxelVolumes {
    transforms: array<mat4x4<f32>>;
};

[[group(0), binding(0)]]
var view: View;

[[group(1), binding(0)]]
var<storage, read> volumes: VoxelVolumes;
[[group(1), binding(1)]]
var<storage, read_write> collisions: array<u32>;

[[stage(compute), workgroup_size(64)]]
fn broadphase([[builtin(global_invocation_id)]] GlobalInvocationID : vec3<u32>) {
    var index: u32 = GlobalInvocationID.x;

    for (var i: u32 = 0u; i < arrayLength(&volumes.transforms); i = i + 1u) {
        // ray box intersection from index converted to screen position and screen position into ray direction via view projection
    }
}
