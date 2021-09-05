[[group(0), binding(0)]] var renderTexture: [[access(read)]] texture_storage_2d<rgba8unorm>;

struct VertexOutput {
  [[location(0)]] uv: vec2<f32>;
  [[builtin(position)]] position: vec4<f32>;
};

[[stage(vertex)]]
fn vertex([[location(0)]] vertex_index: u32) -> VertexOutput {
  var output: VertexOutput;
  output.uv = vec2<f32>(f32((vertex_index << 1u) & 2u), f32(vertex_index & 2u));
  output.position = vec4<f32>(output.uv * 2.0 - 1.0, 0.0, 1.0);
  return output;
}

[[stage(fragment)]]
fn fragment(in: VertexOutput) -> [[location(0)]] vec4<f32> {
  return vec4<f32>(in.uv, 0.0, 1.0);
  // return textureLoad(renderTexture, vec2<i32>(i32(fragUV.x), i32(fragUV.y)));
}
