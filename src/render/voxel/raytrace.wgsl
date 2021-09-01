[[block]]
struct View {
    view_proj: mat4x4<f32>;
    world_position: vec3<f32>;
};

[[group(0), binding(0)]]
var view: View;
[[group(2), binding(0)]] var renderSampler: sampler;
[[group(2), binding(1)]] var renderTexture: [[access(write)]] texture_storage_2d<rgba8unorm>;

[[stage(compute), workgroup_size(64)]]
fn raytrace([[builtin(global_invocation_id)]] global_invocation_id: vec3<u32>) {
    // var width: u32 = 1000u;
    // var texIndex: vec2<i32> = global_invocation_id.y * width + global_invocation_id.x;
    var texIndex: vec2<i32> = vec2<i32>(i32(global_invocation_id.x), i32(global_invocation_id.y));
    textureStore(renderTexture, texIndex, vec4<f32>(vec3<f32>(1.0, 0.0, 0.0), 1.0));
}
