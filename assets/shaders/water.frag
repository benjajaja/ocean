#version 450
layout(location = 0) out vec4 o_Target;
layout(location = 0) in vec3 v_color;
void main() {
    o_Target = vec4(v_color, 1.0);
    // o_Target = vec4(0.3, 0.0, 0.5, 1.0);
}
