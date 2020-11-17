#version 450
layout(location = 0) in vec3 Vertex_Position;
layout(location = 1) in vec3 Vertex_Color;
layout(location = 0) out vec3 v_color;
layout(set = 0, binding = 0) uniform Camera {
    mat4 ViewProj;
};
layout(set = 1, binding = 0) uniform Transform {
    mat4 Model;
};
layout(set = 1, binding = 1) uniform WaterMaterial_time {
    float time;
};

float sine_noise(float);

void main() {
    gl_Position = ViewProj * Model * vec4(Vertex_Position, 1.0);
    gl_Position.y += sine_noise(gl_Position.x + time) + sine_noise(gl_Position.z + time);
    v_color = Vertex_Color;
}

float sine_noise(float v) {
    return (sin(v) + sin(2.2 * v + 5.52) + sin(2.9 * v + 0.93) + sin(4.6 * v + 8.94)) / 64;
}
