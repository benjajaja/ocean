use bevy::{
    prelude::*,
    render::{
        pipeline::{PipelineDescriptor, RenderPipeline},
        render_graph::{base, AssetRenderResourcesNode, RenderGraph},
        renderer::RenderResources,
        shader::{ShaderStage, ShaderStages},
    },
    reflect::TypeUuid,
};
use std::ops::AddAssign;
use std::f32::consts::{PI};

struct Weather {
    wave_intensity: f32,
}

pub struct Water {
    pub waves: [WaveProperties; 3],
    pub wave_speed: f32,
}
impl Water {
    pub fn height_at_point(self: &Self, point: Vec2, time: f32) -> f32{
        let input_point = Vec3::new(point.x, 0., point.y);

        let first_pass = wave_sequence(input_point, time, &self.waves);
        first_pass.position.y
    }
    pub fn wave_data_at_point(self: &Self, point: Vec2, time: f32) -> WaveData{
        let input_point = Vec3::new(point.x, 0., point.y);

        wave_sequence(input_point, time, &self.waves)
    }
}

pub struct WaveData {
    pub position: Vec3,
    pub normal: Vec3,
    pub binormal: Vec3,
    pub tangent: Vec3,
}


#[derive(Debug)]
pub struct WaveProperties {
    pub wavelength: f32,
    pub steepness: f32,
    pub direction: Vec2,
}
impl WaveProperties {
    pub fn to_vec4(self: &Self) -> Vec4 {
        Vec4::new(self.direction.x, self.direction.y, self.wavelength, self.steepness)
    }
}

pub struct Swimmer {
    pub world_rotation: f32, // y angle in radians
}
impl Default for Swimmer {
    #[inline]
    fn default() -> Self {
        Swimmer {
            world_rotation: 0.,
        }
    }
}

pub struct WaterCamera;

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "0320b9b8-b3a3-4baa-8bfa-c94008177b17"]
struct WaterMaterial {
    pub time: f32,
    pub color: Vec4,
    pub camera: Vec3,
    pub wave1: Vec4,
    pub wave2: Vec4,
    pub wave3: Vec4,
}
const WATER_VERTEX_SHADER: &str = include_str!("../assets/shaders/water.vert");
const WATER_FRAGMENT_SHADER: &str = include_str!("../assets/shaders/water.frag");


pub fn add_systems(app: &mut bevy::prelude::AppBuilder) -> &mut bevy::prelude::AppBuilder {
    app
        .add_asset::<WaterMaterial>()
        .add_resource(Weather {
            wave_intensity: 1.0,
        })
        .add_startup_system(setup)
        .add_system(update_system)
        .add_system(wave_probe_system)
}

fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    weather: Res<Weather>,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut render_graph: ResMut<RenderGraph>,
    mut water_materials: ResMut<Assets<WaterMaterial>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let water_pipeline_handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        vertex: shaders.add(Shader::from_glsl(ShaderStage::Vertex, WATER_VERTEX_SHADER)),
        fragment: Some(shaders.add(Shader::from_glsl(ShaderStage::Fragment, WATER_FRAGMENT_SHADER))),
    }));
    render_graph.add_system_node(
        "WaterMaterial",
        AssetRenderResourcesNode::<WaterMaterial>::new(true),
    );
    render_graph.add_node_edge(
        "WaterMaterial",
        base::node::MAIN_PASS,
    ).unwrap();


    let mut water = Water {
        waves: get_waves(weather.wave_intensity),
        wave_speed: 0.8,
    };
    set_waves(&mut water, weather.wave_intensity);


    commands
        .spawn(MeshBundle {
            mesh: asset_server.load("plano.glb#Mesh0/Primitive0"),
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                water_pipeline_handle,
            )]),
            transform: Transform::from_scale(Vec3::new(200.0, 200.0, 200.0)),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Plane { size: 500.0 })),
                // material: materials.add(Color::rgb(0.0, 0.0, 0.6).into()),
                material: materials.add(StandardMaterial {
                    shaded: false,
                    albedo: Color::rgb(0.0, 0.01, 0.2).into(),
                    ..Default::default()
                }),
                transform: Transform::from_translation(Vec3::new(0.0, -1.0, 0.0)),
                ..Default::default()
            });
        })
        .with(water_materials.add(WaterMaterial {
            time: 0.,
            color: Vec4::new(0.1, 0.5, 0.5, 1.0),
            camera: Vec3::new(0., 0., 0.),
            wave1: water.waves[0].to_vec4(),
            wave2: water.waves[1].to_vec4(),
            wave3: water.waves[2].to_vec4(),
        }))
        .with(water);

}

fn gerstner_wave(
    position: Vec3,
    time: f32,
    target: &mut Vec3,
    tangent: &mut Vec3,
    binormal: &mut Vec3,
    props: &WaveProperties
) -> () {
    let d = props.direction.normalize();

    let position_xz = Vec2::new(position.x, position.z);
    let k = 2. * PI / props.wavelength;
    let c = (9.8 / k).sqrt(); // Wave speed
    let f = k * (position_xz.dot(d) - c * time);
    let amp_noise = 1.;
    let a = props.steepness / k * amp_noise;

    target.add_assign(Vec3::new(
        d.x * (a * f.cos()),
        a * f.sin() + a,
        d.y * (a * f.cos())
    ));

    tangent.add_assign(Vec3::new(
        -d.x * d.x * (props.steepness * f.sin()),
        d.x * (props.steepness * f.cos()),
        -d.x * d.y * (props.steepness * f.sin())
    ));
    binormal.add_assign(Vec3::new(
        -d.x * d.y * (props.steepness * f.sin()),
        d.y * (props.steepness * f.cos()),
        -d.y * d.y * (props.steepness * f.sin())
    ));
}

fn wave_sequence(position: Vec3, time: f32, waves: &[WaveProperties; 3]) -> WaveData {
    let mut target = position.clone();
    let mut tangent = Vec3::unit_x();
    let mut binormal = Vec3::unit_z();
    // gerstner_wave(position, time, &mut target, &mut tangent, &mut binormal, &waves[0]);
    // gerstner_wave(position, time, &mut target, &mut Vec3::unit_x(), &mut Vec3::unit_z(), &waves[1]);
    // gerstner_wave(position, time, &mut target, &mut Vec3::unit_x(), &mut Vec3::unit_z(), &waves[2]);
    for wave in waves {
        gerstner_wave(position, time, &mut target, &mut tangent, &mut binormal, wave);
    }
    WaveData {
        position: target,
        normal: binormal.cross(tangent).normalize(),
        binormal,
        tangent,
    }
}

pub fn set_waves(water: &mut Water, intensity: f32) -> () {
    water.waves = get_waves(intensity);
}

pub fn get_waves(intensity: f32) -> [WaveProperties; 3] {
    [
        WaveProperties {
            wavelength: intensity * 60.,
            steepness: intensity * 0.25,
            direction: Vec2::new(1.0, 0.0),
        },
        WaveProperties {
            wavelength: intensity * 31.,
            steepness: intensity * 0.25,
            direction: Vec2::new(1.0, 0.6),
        },
        WaveProperties {
            wavelength: intensity * 18.,
            steepness: intensity * 0.25,
            direction: Vec2::new(1.0, 1.3),
        },
    ]
}

pub fn surface_quat(wavedata: &WaveData) -> Quat {
    let normal = wavedata.normal;
    let quat: Quat;
    if normal.y > 0.99999 {
        quat = Quat::from_xyzw(0., 0., 0., 1.);
    } else if normal.y < -0.99999 {
        quat = Quat::from_xyzw(1., 0., 0., 0.);
    } else {
        let axis = Vec3::new(normal.z, 0., -normal.x).normalize();
        let radians = normal.y.acos();
        quat = Quat::from_axis_angle(axis, radians);
    }
    return quat;
}

pub fn wave_probe_system(
    time: Res<Time>,
    mut wave_probes_query: Query<(&Swimmer, &mut Transform)>,
    water_query: Query<(&Water, &Transform)>,
) {
    if let Some((water, water_transform)) = water_query.iter().next() {
        for (swimmer, mut transform) in wave_probes_query.iter_mut() {
            let wavedata = water.wave_data_at_point(
                Vec2::new(transform.translation.x * 1., transform.translation.z * 1.),
                time.seconds_since_startup() as f32 * water.wave_speed,
            );
            transform.translation.y = wavedata.position.y + water_transform.translation.y;

            transform.rotation = surface_quat(&wavedata) * Quat::from_rotation_y(swimmer.world_rotation);
        }
    }
}

fn update_system(
    time: Res<Time>,
    // weather: Res<Weather>,
    mut water_mats: ResMut<Assets<WaterMaterial>>,
    water_material_query: Query<&Handle<WaterMaterial>>,
    camera_query: Query<(&Transform, &WaterCamera)>,
    water_query: Query<&Water>,
) {
    if let Some(water_material) = water_material_query.iter().next()
            .and_then(|water_handle| water_mats.get_mut(water_handle))
    {
        if let Some(water) = water_query.iter().next() {
            water_material.time = time.seconds_since_startup() as f32 * water.wave_speed;
        }


        if let Some((transform, _)) = camera_query.iter().next() {
            water_material.camera = transform.translation;
        }
    }
}

