#version 450

layout(location=1) in vec3 o_Vertex_Position;
layout(location=0) out vec4 o_Target;

layout(set = 0, binding = 0) uniform Camera {
    mat4 ViewProj;
};

void main() {
    o_Target = vec4(
      0.8,
      0.6,
      0.7,
      /* sin(gl_FragCoord.y / 2 ) * 0.1, */
      1.
    );
}

