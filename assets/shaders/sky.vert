#version 450

layout (location = 0) in vec3 Vertex_Position;
layout(location = 1) in vec3 Vertex_Normal;
layout(location = 2) in vec2 Vertex_Uv;

layout(location = 0) out vec2 v_Uv;

layout(set = 0, binding = 0) uniform Camera {
    mat4 ViewProj;
};
layout(set = 1, binding = 0) uniform Transform {
    mat4 Model;
};

void main() {
    v_Uv = Vertex_Uv;

    mat4 OriginViewProj = ViewProj;

    // I don't know what [3][3] (w?) does
    OriginViewProj[3] = vec4(0., 0., 0., 0.0015);

    vec4 pos = OriginViewProj * Model * vec4(Vertex_Position, 1.0);
    gl_Position = pos;
    /* gl_Position.z = 100.; */
}

