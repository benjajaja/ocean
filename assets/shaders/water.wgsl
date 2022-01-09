#import bevy_pbr::mesh_view_bind_group
#import bevy_pbr::mesh_struct

struct WaterMaterial {
    color: vec4<f32>;
    time: f32;
    camera: vec3<f32>;
    wave1: vec4<f32>;
    wave2: vec4<f32>;
    wave3: vec4<f32>;
};
struct Transform {
  Model : mat4x4<f32>;
};
struct CameraViewProj {
  ViewProj : mat4x4<f32>;
};

struct Vertex {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] normal: vec3<f32>;
    [[location(2)]] uv: vec2<f32>;
};
struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] world_position: vec4<f32>;
    [[location(1)]] world_normal: vec3<f32>;
    [[location(2)]] uv: vec2<f32>;
};
[[group(1), binding(0)]]
var<uniform> material: WaterMaterial;

[[group(2), binding(0)]]
var<uniform> mesh: Mesh;

var<private> o_Vertex_Position : vec3<f32>;

var<private> Vertex_Position : vec3<f32>;

[[group(1), binding(0)]] var<uniform> x_32 : Transform;

//[[group(2), binding(3)]] var<uniform> x_55 : WaterUniform_wave1;

//[[group(2), binding(4)]] var<uniform> x_75 : WaterUniform_wave2;

//[[group(2), binding(5)]] var<uniform> x_94 : WaterUniform_wave3;

var<private> World_Position : vec4<f32>;

[[group(0), binding(0)]] var<uniform> x_126 : CameraViewProj;

var<private> Vertex_Normal : vec3<f32>;

//[[group(2), binding(2)]] var<uniform> x_145 : WaterUniform_camera;

var<private> specular : vec3<f32>;

//[[group(2), binding(1)]] var<uniform> x_206 : WaterUniform_time;

var<private> Vertex_Uv : vec2<f32>;

var<private> v_Uv : vec2<f32>;

//[[group(2), binding(0)]] var<uniform> x_311 : WaterUniform_color;

var<private> gl_Position : vec4<f32>;

fn gerstner_wave_vf3_vf3_vf3_vf3_vf4_(position : ptr<function, vec3<f32>>, target : ptr<function, vec3<f32>>, tangent : ptr<function, vec3<f32>>, binormal : ptr<function, vec3<f32>>, props : ptr<function, vec4<f32>>) {
  var d : vec2<f32>;
  var wavelength : f32;
  var steepness : f32;
  var k : f32;
  var c : f32;
  var f : f32;
  var a : f32;
  let x_171 : vec4<f32> = *(props);
  d = normalize(vec2<f32>(x_171.x, x_171.y));
  let x_177 : f32 = (*(props)).z;
  wavelength = x_177;
  let x_181 : f32 = (*(props)).w;
  steepness = x_181;
  let x_182 : f32 = steepness;
  if ((x_182 == 0.0)) {
    return;
  }
  let x_190 : f32 = wavelength;
  k = (6.283185482 / x_190);
  let x_194 : f32 = k;
  c = sqrt((9.800000191 / x_194));
  let x_198 : f32 = k;
  let x_199 : vec2<f32> = d;
  let x_200 : vec3<f32> = *(position);
  let x_203 : f32 = c;
  let x_209 : f32 = material.time;
  f = (x_198 * (dot(x_199, vec2<f32>(x_200.x, x_200.z)) - (x_203 * x_209)));
  let x_214 : f32 = steepness;
  let x_215 : f32 = k;
  a = (x_214 / x_215);
  let x_217 : vec3<f32> = *(target);
  let x_220 : f32 = d.x;
  let x_221 : f32 = a;
  let x_222 : f32 = f;
  let x_226 : f32 = a;
  let x_227 : f32 = f;
  let x_230 : f32 = a;
  let x_233 : f32 = d.y;
  let x_234 : f32 = a;
  let x_235 : f32 = f;
  *(target) = (x_217 + vec3<f32>((x_220 * (x_221 * cos(x_222))), ((x_226 * sin(x_227)) + x_230), (x_233 * (x_234 * cos(x_235)))));
  let x_241 : vec3<f32> = *(tangent);
  let x_243 : f32 = d.x;
  let x_246 : f32 = d.x;
  let x_248 : f32 = steepness;
  let x_249 : f32 = f;
  let x_254 : f32 = d.x;
  let x_255 : f32 = steepness;
  let x_256 : f32 = f;
  let x_261 : f32 = d.x;
  let x_264 : f32 = d.y;
  let x_266 : f32 = steepness;
  let x_267 : f32 = f;
  *(tangent) = (x_241 + vec3<f32>(((-(x_243) * x_246) * (x_248 * sin(x_249))), (x_254 * (x_255 * cos(x_256))), ((-(x_261) * x_264) * (x_266 * sin(x_267)))));
  let x_273 : vec3<f32> = *(binormal);
  let x_275 : f32 = d.x;
  let x_278 : f32 = d.y;
  let x_280 : f32 = steepness;
  let x_281 : f32 = f;
  let x_286 : f32 = d.y;
  let x_287 : f32 = steepness;
  let x_288 : f32 = f;
  let x_293 : f32 = d.y;
  let x_296 : f32 = d.y;
  let x_298 : f32 = steepness;
  let x_299 : f32 = f;
  *(binormal) = (x_273 + vec3<f32>(((-(x_275) * x_278) * (x_280 * sin(x_281))), (x_286 * (x_287 * cos(x_288))), ((-(x_293) * x_296) * (x_298 * sin(x_299)))));
  return;
}

fn main_1() {
  var light_direction : vec3<f32>;
  var Original_World_Position : vec4<f32>;
  var target_1 : vec3<f32>;
  var tangent_1 : vec3<f32>;
  var binormal_1 : vec3<f32>;
  var param : vec3<f32>;
  var param_1 : vec3<f32>;
  var param_2 : vec3<f32>;
  var param_3 : vec3<f32>;
  var param_4 : vec4<f32>;
  var param_5 : vec3<f32>;
  var param_6 : vec3<f32>;
  var param_7 : vec3<f32>;
  var param_8 : vec3<f32>;
  var param_9 : vec4<f32>;
  var param_10 : vec3<f32>;
  var param_11 : vec3<f32>;
  var param_12 : vec3<f32>;
  var param_13 : vec3<f32>;
  var param_14 : vec4<f32>;
  var light_reflect_direction : vec3<f32>;
  var view_direction : vec3<f32>;
  var light_see_direction : f32;
  var shininess : f32;
  light_direction = vec3<f32>(0.577350259, 0.577350259, -0.577350259);
  let x_27 : vec3<f32> = Vertex_Position;
  o_Vertex_Position = x_27;
  let x_37 : mat4x4<f32> = x_32.Model;
  let x_38 : vec3<f32> = Vertex_Position;
  Original_World_Position = (x_37 * vec4<f32>(x_38.x, x_38.y, x_38.z, 1.0));
  let x_46 : vec4<f32> = Original_World_Position;
  target_1 = vec3<f32>(x_46.x, x_46.y, x_46.z);
  tangent_1 = vec3<f32>(1.0, 0.0, 0.0);
  binormal_1 = vec3<f32>(0.0, 0.0, 1.0);
  let x_57 : vec4<f32> = Original_World_Position;
  param = vec3<f32>(x_57.x, x_57.y, x_57.z);
  let x_60 : vec3<f32> = target_1;
  param_1 = x_60;
  let x_62 : vec3<f32> = tangent_1;
  param_2 = x_62;
  let x_64 : vec3<f32> = binormal_1;
  param_3 = x_64;
  let x_68 : vec4<f32> = material.wave1;
  param_4 = x_68;
  gerstner_wave_vf3_vf3_vf3_vf3_vf4_(&(param), &(param_1), &(param_2), &(param_3), &(param_4));
  let x_70 : vec3<f32> = param_1;
  target_1 = x_70;
  let x_71 : vec3<f32> = param_2;
  tangent_1 = x_71;
  let x_72 : vec3<f32> = param_3;
  binormal_1 = x_72;
  let x_77 : vec4<f32> = Original_World_Position;
  param_5 = vec3<f32>(x_77.x, x_77.y, x_77.z);
  let x_80 : vec3<f32> = target_1;
  param_6 = x_80;
  let x_82 : vec3<f32> = tangent_1;
  param_7 = x_82;
  let x_84 : vec3<f32> = binormal_1;
  param_8 = x_84;
  let x_87 : vec4<f32> = material.wave2;
  param_9 = x_87;
  gerstner_wave_vf3_vf3_vf3_vf3_vf4_(&(param_5), &(param_6), &(param_7), &(param_8), &(param_9));
  let x_89 : vec3<f32> = param_6;
  target_1 = x_89;
  let x_90 : vec3<f32> = param_7;
  tangent_1 = x_90;
  let x_91 : vec3<f32> = param_8;
  binormal_1 = x_91;
  let x_96 : vec4<f32> = Original_World_Position;
  param_10 = vec3<f32>(x_96.x, x_96.y, x_96.z);
  let x_99 : vec3<f32> = target_1;
  param_11 = x_99;
  let x_101 : vec3<f32> = tangent_1;
  param_12 = x_101;
  let x_103 : vec3<f32> = binormal_1;
  param_13 = x_103;
  let x_106 : vec4<f32> = material.wave3;
  param_14 = x_106;
  gerstner_wave_vf3_vf3_vf3_vf3_vf4_(&(param_10), &(param_11), &(param_12), &(param_13), &(param_14));
  let x_108 : vec3<f32> = param_11;
  target_1 = x_108;
  let x_109 : vec3<f32> = param_12;
  tangent_1 = x_109;
  let x_110 : vec3<f32> = param_13;
  binormal_1 = x_110;
  let x_113 : vec3<f32> = target_1;
  World_Position = vec4<f32>(x_113.x, x_113.y, x_113.z, 1.0);
  let x_128 : mat4x4<f32> = x_126.ViewProj;
  let x_129 : vec4<f32> = World_Position;
  gl_Position = (x_128 * x_129);
  let x_133 : vec3<f32> = binormal_1;
  let x_134 : vec3<f32> = tangent_1;
  Vertex_Normal = normalize(cross(x_133, x_134));
  let x_138 : vec3<f32> = light_direction;
  let x_139 : vec3<f32> = -(x_138);
  let x_140 : vec3<f32> = Vertex_Normal;
  light_reflect_direction = reflect(x_139, x_140);
  let x_148 : vec3<f32> = material.camera;
  let x_149 : vec4<f32> = World_Position;
  view_direction = normalize((x_148 - vec3<f32>(x_149.x, x_149.y, x_149.z)));
  let x_155 : vec3<f32> = light_reflect_direction;
  let x_156 : vec3<f32> = view_direction;
  light_see_direction = max(0.0, dot(x_155, x_156));
  let x_160 : f32 = light_see_direction;
  shininess = pow(x_160, 100.0);
  let x_166 : f32 = shininess;
  specular = (vec3<f32>(0.5, 0.5, 0.5) * x_166);
  return;
}

struct main_out {
  [[location(3)]]
  o_Vertex_Position_1 : vec3<f32>;
  [[location(2)]]
  World_Position_1 : vec4<f32>;
  [[builtin(position)]]
  gl_Position : vec4<f32>;
  [[location(1)]]
  Vertex_Normal_1 : vec3<f32>;
  [[location(4)]]
  specular_1 : vec3<f32>;
  [[location(0)]]
  v_Uv_1 : vec2<f32>;
};

[[stage(vertex)]]
fn main([[location(0)]] Vertex_Position_param : vec3<f32>, [[location(1)]] Vertex_Uv_param : vec2<f32>) -> main_out {
  Vertex_Position = Vertex_Position_param;
  Vertex_Uv = Vertex_Uv_param;
  main_1();
  return main_out(o_Vertex_Position, World_Position, gl_Position, Vertex_Normal, specular, v_Uv);
}


//[[stage(vertex)]]
//fn vertex(vertex: Vertex) -> VertexOutput {
//    let world_position = mesh.model * vec4<f32>(vertex.position, 1.0);
//
//    var out: VertexOutput;
//    out.uv = vertex.uv;
//    out.world_position = world_position;
//    out.clip_position = view.view_proj * world_position;
//    out.world_normal = mat3x3<f32>(
//        mesh.inverse_transpose_model[0].xyz,
//        mesh.inverse_transpose_model[1].xyz,
//        mesh.inverse_transpose_model[2].xyz
//    ) * vertex.normal;
//#ifdef VERTEX_TANGENTS
//    out.world_tangent = vec4<f32>(
//        mat3x3<f32>(
//            mesh.model[0].xyz,
//            mesh.model[1].xyz,
//            mesh.model[2].xyz
//        ) * vertex.tangent.xyz,
//        vertex.tangent.w
//    );
//#endif
//    return out;
//}

fn hash22(p: vec2<f32>, iTime: f32) -> vec2<f32> {
    // Faster, but probably doesn't disperse things as nicely as other ways.
    let n: f32 = sin(dot(p, vec2<f32>(1., 113.)));
    let p: vec2<f32> = fract(vec2<f32>(8. * n, n) * 262144.);
    return sin(p * 6.2831853 + iTime * 2.);
}

// 3-tap Voronoi... kind of. I'm pretty sure I'm not the only one who's thought to try this.
//
// Due to the simplex grid setup, it's probably slightly more expensive than the 4-tap, square 
// grid version, but I believe the staggered cells make the patterns look a little nicer. I'd 
// imagine it's faster than the unrolled 9-tap version, but I couldn't say for sure. Anyway, 
// it's just a novelty, bordering on pointless, but I thought it might interest someone.

// I'm not perfectly happy with the random offset figure of ".125" or the normalization figure 
// of ".425." They might be right, but I'll determine those for sure later. They seem to work.
//
// Credits: Ken Perlin, Brian Sharpe, IQ, various Shadertoy people, etc.
//
fn Voronoi3Tap(p: vec2<f32>, iTime: f32) -> f32 {
    // Simplex grid stuff.
    //
    let s: vec2<f32> = floor(p + (p.x + p.y)*.3660254); // Skew the current point.
    let p1: vec2<f32> = p - s - (s.x + s.y)*.2113249; // Use it to attain the vector to the base vertice (from p).

    // Determine which triangle we're in -- Much easier to visualize than the 3D version. :)
    // The following is equivalent to "float i = step(p.y, p.x)," but slightly faster, I hear.
    let i = select(1., 0., p.x < p.y); // p.x<p.y? 0. : 1.;


    // Vectors to the other two triangle vertices.
    let p2 = p - vec2<f32>(i, 1. - i) + .2113249;
    let p3 = p - .5773502;


    // Add some random gradient offsets to the three vectors above.
    let hp1 = p1 + hash22(s, iTime)*.125;
    let hp2 = p2 + hash22(s +  vec2<f32>(i, 1. - i), iTime)*.125;
    let hp3 = p3 + hash22(s + 1., iTime)*.125;

    // Determine the minimum Euclidean distance. You could try other distance metrics, 
    // if you wanted.
    let d = min(min(dot(hp1, hp1), dot(hp2, hp2)), dot(hp3, hp3))/.425;

    // That's all there is to it.
    return sqrt(d); // Take the square root, if you want, but it's not mandatory.

}

[[stage(fragment)]]
fn fragment() -> [[location(0)]] vec4<f32> {
    return material.color;
}

