[[block]]
struct View {
    view_proj: mat4x4<f32>;
    world_position: vec3<f32>;
};

[[group(0), binding(0)]]
var view: View;
[[group(2), binding(0)]] var renderSampler : sampler;
[[group(2), binding(1)]] var renderTexture : texture_2d<f32>;

[[stage(compute), workgroup_size(64)]]
fn raytrace([[builtin(global_invocation_id)]] GlobalInvocationID : vec3<u32>) {
    var texIndex = global_invocation_id.y * width + global_invocation_id.x;
    textureStore(renderTexture, texIndex, vec4<f32>(vec3<f32>(1.0, 0.0, 0.0), 1.0));
}
