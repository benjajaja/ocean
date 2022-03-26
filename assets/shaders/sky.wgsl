#import bevy_pbr::mesh_view_bind_group
#import bevy_pbr::mesh_struct

struct CustomMaterial {
    color: vec4<f32>;
    background: i32;
};
[[group(1), binding(0)]]
var<uniform> material: CustomMaterial;


[[group(2), binding(0)]]
var<uniform> mesh: Mesh;

struct FragmentInput {
    [[builtin(front_facing)]] is_front: bool;
    [[builtin(position)]] frag_coord: vec4<f32>;
    [[location(0)]] world_position: vec4<f32>;
    [[location(1)]] world_normal: vec3<f32>;
    [[location(2)]] uv: vec2<f32>;
#ifdef VERTEX_TANGENTS
    [[location(3)]] world_tangent: vec4<f32>;
#endif
};

[[stage(fragment)]]
fn fragment(in: FragmentInput) -> [[location(0)]] vec4<f32> {
    return material.color * (0.1 + in.world_normal.y * .2);
}
