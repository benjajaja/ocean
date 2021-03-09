#version 450

layout(location=0) out vec4 o_Target;

layout(set = 0, binding = 0) uniform Camera {
    mat4 ViewProj;
};
layout(set = 1, binding = 0) uniform Transform {
    mat4 Model;
};


void main() {
    o_Target = vec4(1.0, 0.5, 0.5, 1.0);
}


