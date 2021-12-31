#version 450

layout(set = 2, binding = 0) uniform WaterUniform_time {
    float time;
};
layout(set = 2, binding = 1) uniform WaterUniform_color {
    vec3 color;
};
layout(set = 2, binding = 2) uniform WaterUniform_camera {
    vec3 camera;
};

layout(location = 0) in vec2 v_Uv;
layout(location = 1) in vec3 Vertex_Normal;
layout(location = 2) in vec4 World_Position;
layout(location = 3) in vec3 o_Vertex_Position;
layout(location = 4) in vec3 specular;

layout(location = 0) out vec4 o_Target;


const vec3 light_direction = normalize(vec3(1, 1, -1));
const float FADE_DROPOFF = 0.75;
const float specular_intensity = 100;

float Voronoi3Tap(vec2 p, float iTime);

void main() {
    /* vec3 light_reflect_direction = reflect(-light_direction, Vertex_Normal); */
    /* vec3 view_direction = normalize(camera - World_Position.xyz); */
    /* float light_see_direction = max(0.0, dot(light_reflect_direction, view_direction)); */
    /* float shininess = pow(light_see_direction, specular_intensity); */
    /* vec3 specular = vec3(.5) * shininess; */

    // voronoi:
    float fade = 1 - smoothstep(0.75, 0.9, sqrt(dot(o_Vertex_Position.xz, o_Vertex_Position.xz)));

    float grid = smoothstep(0.99, .999, (sin(World_Position.x * 4)) * 1)
      + smoothstep(0.99, .999, (sin(World_Position.z * 4)) * 1);

    float pixelate = .02;
    float voronoiTap = Voronoi3Tap(pixelate * floor(World_Position.xz * 0.03 / pixelate), time);
    float voronoi = pow(voronoiTap, 5);
    vec3 voronoi_sample = vec3(voronoi) * vec3(1.0, 0.8, 0.9);
    //smoothstep(0.5, 1.0, pow(c + .2, 10));

    vec3 diffuse = voronoi_sample * max(0., (dot(Vertex_Normal, light_direction)));

    vec3 texture_color = color + diffuse + specular;
      /* vec3(crest, crest, crest) */
      /* vec3(0., 0., 0.) +*/
      /* diffuse + specular + */
      /* vec3( */
          /* 0., */
          /* 0., */
          /* voronoi */
        /* ); */
    o_Target = vec4(texture_color, fade);
}

/*
	3-Tap 2D Voronoi
	----------------

	I saw member BH's hexagonal Voronoi example, which reminded me that I had a 3-tap simplex
	version gathering pixel dust on my harddrive, so here it is.

	I hastily added some coloring and very cheap highlights, just to break the visual monotony, 
	but you can safely ignore most of the code and head straight to the "Voronoi3Tap" function. 
	That's the main point. Like BH's example, this one is branchless. In fact, there's
	virtually no code at all.

	As mentioned below, 3-tap Voronoi is just a novelty, bordering on pointless, but I thought 
	it might provide a basis for anyone wishing to build a 3D simplex version. I also have a 
	4-tap Voronoi function that involves even less computation.

	By the way, the pattern is supposed to be concave. The reason I mention that is, if I stare 
	at a highlighted Voronoi pattern for too long, it sometimes looks inverted. Usually, I have 
	to close my eyes and reopen them to reinvert it. I've often wondered whether that happens to 
	everyone, or whether I'm just getting old. :)

	// Other Shadertoy examples:

	// Hexagonal Voronoi - By "BH."
    // By the way, his version has artifacts, but Dr2 and myself have some hexagonal Voronoi 
    // examples on here that are more robust.
	https://www.shadertoy.com/view/ltjXz1 - I'm looking forward to the finished version. :)

	// Voronoi fast, a 2x2 grid, 4tap version - By "davidbargo":
	https://www.shadertoy.com/view/4tsXRH

*/



// Standard 2x2 hash algorithm.
vec2 hash22(vec2 p, float iTime) { 

    // Faster, but probably doesn't disperse things as nicely as other ways.
    float n = sin(dot(p,vec2(1, 113))); 
    p = fract(vec2(8.*n, n)*262144.);
    return sin(p*6.2831853 + iTime*2.);
    
/* 
	return fract(sin(p)*43758.5453)*2. - 1.;
    
    //p = fract(sin(p)*43758.5453);
	//p = sin(p*6.2831853 + iTime);
    //return sign(p)*.25 + .75*p;
    
    //p = fract(sin(p)*43758.5453)*2. - 1.;
    //return (sign(p)*.25 + p*.75);    
 */   
    
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
float Voronoi3Tap(vec2 p, float iTime){
    
	// Simplex grid stuff.
    //
    vec2 s = floor(p + (p.x + p.y)*.3660254); // Skew the current point.
    p -= s - (s.x + s.y)*.2113249; // Use it to attain the vector to the base vertice (from p).

    // Determine which triangle we're in -- Much easier to visualize than the 3D version. :)
    // The following is equivalent to "float i = step(p.y, p.x)," but slightly faster, I hear.
    float i = p.x<p.y? 0. : 1.;
    
    
    // Vectors to the other two triangle vertices.
    vec2 p1 = p - vec2(i, 1. - i) + .2113249, p2 = p - .5773502; 

    // Add some random gradient offsets to the three vectors above.
    p += hash22(s, iTime)*.125;
    p1 += hash22(s +  vec2(i, 1. - i), iTime)*.125;
    p2 += hash22(s + 1., iTime)*.125;
    
    // Determine the minimum Euclidean distance. You could try other distance metrics, 
    // if you wanted.
    float d = min(min(dot(p, p), dot(p1, p1)), dot(p2, p2))/.425;
   
    // That's all there is to it.
    return sqrt(d); // Take the square root, if you want, but it's not mandatory.

}
