#version 450

// precision highp float;

layout(location = 0) in vec2 v_Position;
layout(location = 1) in vec2 v_Uv;

layout(location = 0) out vec4 o_Target;

layout(set = 0, binding = 0) uniform Camera {
    mat4 ViewProj;
};

layout(set = 1, binding = 0) uniform Time {
    double TimeElapsed;
};

void main(void) {
    vec3 ForwardVector = normalize(vec3(inverse(ViewProj)[2]));
    vec3 RightVector = normalize(cross(vec3(0.0, 1.0, 0.0), ForwardVector));
    vec3 UpVector = normalize(cross(ForwardVector, RightVector));
    // vec2 ViewportPosition = gl_FragCoord.xy / ScreenResolution;
    vec3 col = 0.5 + 0.5 * cos(vec3(TimeElapsed) + v_Uv.xyx + vec3(0,2,4));
    o_Target = vec4(col,1.0);
}
