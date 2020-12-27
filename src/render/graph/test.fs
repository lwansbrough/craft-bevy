#version 450

precision highp float;

layout(location = 0) in vec2 v_Position;
layout(location = 1) in vec2 v_Uv;

layout(location = 0) out vec4 o_Target;

// layout(set = 1, binding = 0) uniform Time {
//     float Time;
// };

void main(void) {
    // vec3 col = 0.5 + 0.5 * cos(Time + uv.xyx + vec3(0,2,4));
    // o_Target = vec4(col,1.0);
    o_Target = vec4(v_Uv, 0.0, 1.0);
}
