#version 450

layout(location=1) in vec3 Vertex_Normal;
layout(location=2) in vec4 World_Position;
layout(location=3) in vec4 Original_World_Position;
layout(location=4) in vec3 o_Vertex_Position;

layout(location=0) out vec4 o_Target;

layout(set = 0, binding = 0) uniform Camera {
    mat4 ViewProj;
};

void main() {
    o_Target = vec4(0.8, 0.6, 0.7, 1.);
}

