#version 450

layout(set = 1, binding = 1) uniform WaterMaterial_time {
    float time;
    float intensity;
    vec4 color;
    vec3 camera;
};
layout(location=1) in vec3 Vertex_Normal;
layout(location=2) in vec4 World_Position;
layout(location=3) in vec4 Original_World_Position;
layout(location=4) in vec3 o_Vertex_Position;

layout(location=0) out vec4 o_Target;


const vec3 light_direction = normalize(vec3(0.0, 1, 0.8));
const float FADE_DROPOFF = 0.75;

void main() {
    float specular_intensity = .1;
    vec3 specular = pow(dot(
        o_Vertex_Position,
        /* normalize((camera - World_Position.xyz)), */
        reflect(light_direction, Vertex_Normal)
    ), specular_intensity) * vec3(1.0, 1.0, 1.0);

    vec3 diffuse = color.rgb * (dot(Vertex_Normal, light_direction));

    vec3 local_position = o_Vertex_Position;
    /* float fade = (FADE_DROPOFF - abs(local_position.x)) * (FADE_DROPOFF - abs(local_position.z)); */
    float fade = 1 - smoothstep(0.75, 0.9, sqrt(dot(local_position.xz, local_position.xz)));

    float stripe = step(.95, (sin(World_Position.x * 10)) * 1)
      + step(.95, (sin(World_Position.z * 10)) * 1);

    o_Target = vec4(
        stripe * fade,
        sin(gl_FragCoord.y / 2 - time * 10) * 0.1 * fade,
        (1 - (specular.y)) * fade + (stripe * fade),
        1.
    );
    /* o_Target = vec4(specular, 1.); */
    /* o_Target = vec4(Vertex_Normal / 2., 1); */
    /* o_Target = vec4(0., 0., World_Position.y, 1.); */
}
