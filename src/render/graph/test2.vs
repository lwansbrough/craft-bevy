#version 450

layout(location = 0) in vec3 Vertex_Position;
layout(location = 1) in vec3 Vertex_Normal;
layout(location = 2) in vec2 Vertex_Uv;

layout(location = 0) out vec3 v_Position;
layout(location = 1) out vec3 v_Normal;
layout(location = 2) out vec2 v_Uv;
layout(location = 3) out vec4 v_Near;
layout(location = 4) out vec4 v_Far;

layout(set = 0, binding = 0) uniform Camera {
    mat4 ViewProj;
    mat4 View;
};

layout(set = 1, binding = 0) uniform Transform {
    mat4 Model;
};

void main() {
    // v_Normal = mat3(Model) * Vertex_Normal;
    // v_Position = (Model * vec4(Vertex_Position, 1.0)).xyz;
    // v_Uv = Vertex_Uv;
    // gl_Position = ViewProj * vec4(v_Position, 1.0);

    v_Normal = mat3(Model) * Vertex_Normal;
    v_Position = Vertex_Position;
    v_Uv = Vertex_Uv;
    gl_Position = ViewProj * vec4((Model * vec4(Vertex_Position, 1.0)).xyz, 1.0);

    vec2 pos = gl_Position.xy / gl_Position.w;
    v_Near = inverse(ViewProj) * vec4(pos, -1.0, 1.0);
    v_Far = v_Near + inverse(ViewProj)[2];
    v_Near.xyz /= v_Near.w;
    v_Far.xyz /= v_Far.w;
}