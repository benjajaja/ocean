#import bevy_pbr::mesh_view_bind_group

struct CustomMaterial {
    color: vec4<f32>;
    background: i32;
};
[[group(1), binding(0)]]
var<uniform> material: CustomMaterial;

[[stage(fragment)]]
fn fragment([[builtin(position)]] position: vec4<f32>) -> [[location(0)]] vec4<f32> {
    if (material.background == 0) {
      return material.color;
    }
    let uv = position.xy / vec2<f32>(view.width, view.height);
    // let color = textureSample(texture, texture_sampler, uv);
    return material.color * uv.y;
}
