#version 450
// from: https://gitlab.com/TheZoq2/i_sjon_kan_ingen_hora_dig_skrika
layout(location = 0) in vec3 Vertex_Position;
layout(location = 1) in vec2 Vertex_Uv;
layout(location = 0) out vec2 v_Uv;

layout(location = 1) out vec3 Vertex_Normal;
layout(location = 2) out vec4 World_Position;
layout(location = 3) out vec3 o_Vertex_Position;
layout(location = 4) out vec3 specular;

layout(set = 0, binding = 0) uniform CameraViewProj {
    mat4 ViewProj;
};
layout(set = 1, binding = 0) uniform Transform {
    mat4 Model;
};
layout(set = 2, binding = 0) uniform WaterUniform_time {
    float time;
};
layout(set = 2, binding = 1) uniform WaterUniform_color {
    vec3 color;
};
layout(set = 2, binding = 2) uniform WaterUniform_camera {
    vec3 camera;
};
layout(set = 2, binding = 3) uniform WaterUniform_wave1 {
    vec4 wave1;
};
layout(set = 2, binding = 4) uniform WaterUniform_wave2 {
    vec4 wave2;
};
layout(set = 2, binding = 5) uniform WaterUniform_wave3 {
    vec4 wave3;
};

float sine_noise(float, float);
// https://catlikecoding.com/unity/tutorials/flow/waves/
float snoise(vec2);
void gerstner_wave(vec3 position, inout vec3 target, inout vec3 tangent, inout vec3 binormal, vec4 props);


const vec3 light_direction = normalize(vec3(1, 1, -1));
const float specular_intensity = 100;

void main() {
    o_Vertex_Position = Vertex_Position;
    vec4 Original_World_Position = Model * vec4(Vertex_Position, 1.0);

    vec3 target = Original_World_Position.xyz;
    vec3 tangent = vec3(1, 0, 0);
    vec3 binormal = vec3(0, 0, 1);
    gerstner_wave(Original_World_Position.xyz, target, tangent, binormal, wave1);
    gerstner_wave(Original_World_Position.xyz, target, tangent, binormal, wave2);
    gerstner_wave(Original_World_Position.xyz, target, tangent, binormal, wave3);

    /* float noise_x = sine_noise(Original_World_Position.x, time / 2); */
    /* float noise_z = sine_noise(Original_World_Position.z, time / 4); */
    /* wave.position.y += noise_x + noise_z; */

    World_Position = vec4(target, 1);
    gl_Position = ViewProj * World_Position;

    Vertex_Normal = normalize(cross(binormal, tangent));

    // light reflection
    vec3 light_reflect_direction = reflect(-light_direction, Vertex_Normal);
    vec3 view_direction = normalize(camera - World_Position.xyz);
    float light_see_direction = max(0.0, dot(light_reflect_direction, view_direction));
    float shininess = pow(light_see_direction, specular_intensity);
    specular = vec3(.5) * shininess;
}

float sine_noise(float v, float time) {
    return (sin(v) + sin(2.2 * v + 5.52) + sin(2.9 * v + 0.93) + sin(4.6 * v + 8.94 * time)) / 16;
}

const float M_PI = 3.1415926535897932384626433832795;

// Taken off https://stackoverflow.com/questions/4200224/random-noise-functions-for-glsl
float rand(vec2 co){
    return fract(sin(dot(co.xy ,vec2(12.9898,78.233))) * 43758.5453);
}

void gerstner_wave(
    vec3 position,
    inout vec3 target,
    inout vec3 tangent,
    inout vec3 binormal,
    vec4 props
) {
    vec2 d = normalize(props.xy);
    float wavelength = props.z;
    float steepness = props.w;
    if (steepness == 0) {
      return;
    }

    float k = 2 * M_PI / wavelength;
    float c = sqrt(9.8 / k); // Wave speed
    float f = k * (dot(d, position.xz) - c * time);
    /* float amp_noise = (1 + snoise(position.xz / 10 + vec2(time*0.1, 0)) * 0.6); */
    float a = steepness / k;// * amp_noise;

    target = target + vec3(
        d.x * (a * cos(f)),
        a * sin(f) + a,
        d.y * (a * cos(f))
    );

    tangent = tangent + vec3(
        -d.x * d.x * (steepness * sin(f)),
        d.x * (steepness * cos(f)),
        -d.x * d.y * (steepness * sin(f))
    );
    binormal = binormal + vec3(
        -d.x * d.y * (steepness * sin(f)),
        d.y * (steepness * cos(f)),
        -d.y * d.y * (steepness * sin(f))
    );
}


// Note: These are borrowed from
//    https://github.com/ashima/webgl-noise/blob/master/src/noise2D.glsl
//
// Description : Array and textureless GLSL 2D simplex noise function.
//      Author : Ian McEwan, Ashima Arts.
//  Maintainer : stegu
//     Lastmod : 20110822 (ijm)
//     License : Copyright (C) 2011 Ashima Arts. All rights reserved.
//               Distributed under the MIT License. See LICENSE file.
//               https://github.com/ashima/webgl-noise
//               https://github.com/stegu/webgl-noise
// 

vec3 mod289(vec3 x) {
    return x - floor(x * (1.0 / 289.0)) * 289.0;
}

vec2 mod289(vec2 x) {
    return x - floor(x * (1.0 / 289.0)) * 289.0;
}

vec3 permute(vec3 x) {
    return mod289(((x*34.0)+1.0)*x);
}

float snoise(vec2 v) {
    const vec4 C = vec4(0.211324865405187,    // (3.0-sqrt(3.0))/6.0
                                            0.366025403784439,    // 0.5*(sqrt(3.0)-1.0)
                                         -0.577350269189626,    // -1.0 + 2.0 * C.x
                                            0.024390243902439); // 1.0 / 41.0
// First corner
    vec2 i    = floor(v + dot(v, C.yy) );
    vec2 x0 = v -     i + dot(i, C.xx);

// Other corners
    vec2 i1;
    //i1.x = step( x0.y, x0.x ); // x0.x > x0.y ? 1.0 : 0.0
    //i1.y = 1.0 - i1.x;
    i1 = (x0.x > x0.y) ? vec2(1.0, 0.0) : vec2(0.0, 1.0);
    // x0 = x0 - 0.0 + 0.0 * C.xx ;
    // x1 = x0 - i1 + 1.0 * C.xx ;
    // x2 = x0 - 1.0 + 2.0 * C.xx ;
    vec4 x12 = x0.xyxy + C.xxzz;
    x12.xy -= i1;

// Permutations
    i = mod289(i); // Avoid truncation effects in permutation
    vec3 p = permute( permute( i.y + vec3(0.0, i1.y, 1.0 ))
		+ i.x + vec3(0.0, i1.x, 1.0 ));

    vec3 m = max(0.5 - vec3(dot(x0,x0), dot(x12.xy,x12.xy), dot(x12.zw,x12.zw)), 0.0);
    m = m*m ;
    m = m*m ;

// Gradients: 41 points uniformly over a line, mapped onto a diamond.
// The ring size 17*17 = 289 is close to a multiple of 41 (41*7 = 287)

    vec3 x = 2.0 * fract(p * C.www) - 1.0;
    vec3 h = abs(x) - 0.5;
    vec3 ox = floor(x + 0.5);
    vec3 a0 = x - ox;

// Normalise gradients implicitly by scaling m
// Approximation of: m *= inversesqrt( a0*a0 + h*h );
    m *= 1.79284291400159 - 0.85373472095314 * ( a0*a0 + h*h );

// Compute final noise value at P
    vec3 g;
    g.x    = a0.x    * x0.x    + h.x    * x0.y;
    g.yz = a0.yz * x12.xz + h.yz * x12.yw;
    return 130.0 * dot(m, g);
}

