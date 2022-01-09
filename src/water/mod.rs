use crate::boat::PlayerBoat;
use crate::AppState;
use bevy::render::render_asset::RenderAssetPlugin;

use bevy::prelude::*;
// use bevy_inspector_egui::Inspectable;
use std::f32::consts::PI;
use std::ops::AddAssign;
mod material;
use material::WaterMaterial;

pub struct Weather {
    pub wave_intensity: f32,
}

#[derive(Component, Debug)]
pub struct Water {
    pub waves: [WaveProperties; 3],
    pub wave_speed: f32,
    pub color: Color,
}
impl Water {
    #[allow(dead_code)]
    pub fn height_at_point(self: &Self, point: Vec2, time: f32) -> f32 {
        let input_point = Vec3::new(point.x, 0., point.y);

        let first_pass = wave_sequence(input_point, time, &self.waves);
        first_pass.position.y
    }
    pub fn wave_data_at_point(self: &Self, point: Vec2, time: f32) -> WaveData {
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
        Vec4::new(
            self.direction.x,
            self.direction.y,
            self.wavelength,
            self.steepness,
        )
    }
}

#[derive(Component)]
pub struct Swimmer {
    pub world_rotation: f32, // y angle in radians
}
impl Default for Swimmer {
    #[inline]
    fn default() -> Self {
        Swimmer { world_rotation: 0. }
    }
}

// just to query, should be elsewhere
#[derive(Component)]
pub struct WaterCamera;

pub fn add_systems(app: &mut bevy::prelude::App) -> &mut bevy::prelude::App {
    // app.add_asset::<WaterMaterial>()
    app.insert_resource(Weather {
        wave_intensity: 2.0,
    })
    .add_plugin(MaterialPlugin::<WaterMaterial>::default())
    .add_plugin(RenderAssetPlugin::<WaterMaterial>::default())
    // .add_plugin(material::CustomMaterialPlugin)
    .add_startup_system(setup.system())
    .add_system_set(
        SystemSet::on_update(AppState::InGame)
            .with_system(update_system.system().label("water").after("physics"))
            .with_system(wave_probe_system.system().label("water").after("physics")),
    )
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    weather: Res<Weather>,
    mut water_materials: ResMut<Assets<WaterMaterial>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let water = Water {
        waves: get_waves(weather.wave_intensity),
        wave_speed: 0.8,
        // color: Color::rgb_u8(128, 0, 1),
        color: Color::rgb_u8(3, 0, 1),
    };

    let water_material = water_materials.add(WaterMaterial {
        time: 0.,
        color: water.color.into(),
        camera: Vec3::new(0., 0., 0.),
        wave1: water.waves[0].to_vec4(),
        wave2: water.waves[1].to_vec4(),
        wave3: water.waves[2].to_vec4(),
    });

    let mesh: Handle<Mesh> = asset_server.load("plano.glb#Mesh0/Primitive0");
    let water_entity = commands
        .spawn()
        .insert_bundle((
            mesh,
            Transform::from_scale(Vec3::new(500.0, 500.0, 500.0)),
            GlobalTransform::default(),
            water_material,
            // material::CustomMaterial,
            Visibility::default(),
            ComputedVisibility::default(),
            water,
            Name::new("Water"),
        ))
        .id();

    // let water_entity = commands
    // .spawn_bundle(MaterialMeshBundle {
    // mesh: asset_server.load("plano.glb#Mesh0/Primitive0"),
    // material: water_material,
    // transform: Transform::from_scale(Vec3::new(500.0, 500.0, 500.0)),
    // ..Default::default()
    // })
    // .insert(water)
    // .insert(Name::new("Water"))
    // .id();

    let water_bottom_plane = commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 5000.0 })),
            material: materials.add(Color::rgb(0.0, 0.0, 0.6).into()),
            transform: Transform::from_translation(Vec3::new(0.0, -1.0, 0.0)),
            ..Default::default()
        })
        .insert(Name::new("Water-plane"))
        .id();
    commands
        .entity(water_entity)
        .push_children(&[water_bottom_plane]);
}

fn gerstner_wave(
    position: Vec3,
    time: f32,
    target: &mut Vec3,
    tangent: &mut Vec3,
    binormal: &mut Vec3,
    props: &WaveProperties,
) -> () {
    if props.steepness == 0. {
        return;
    }
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
        d.y * (a * f.cos()),
    ));

    tangent.add_assign(Vec3::new(
        -d.x * d.x * (props.steepness * f.sin()),
        d.x * (props.steepness * f.cos()),
        -d.x * d.y * (props.steepness * f.sin()),
    ));
    binormal.add_assign(Vec3::new(
        -d.x * d.y * (props.steepness * f.sin()),
        d.y * (props.steepness * f.cos()),
        -d.y * d.y * (props.steepness * f.sin()),
    ));
}

fn wave_sequence(position: Vec3, time: f32, waves: &[WaveProperties; 3]) -> WaveData {
    let mut target = position.clone();
    let mut tangent = Vec3::X;
    let mut binormal = Vec3::Z;
    // gerstner_wave(position, time, &mut target, &mut tangent, &mut binormal, &waves[0]);
    // gerstner_wave(position, time, &mut target, &mut Vec3::unit_x(), &mut Vec3::unit_z(), &waves[1]);
    // gerstner_wave(position, time, &mut target, &mut Vec3::unit_x(), &mut Vec3::unit_z(), &waves[2]);
    for wave in waves {
        gerstner_wave(
            position,
            time,
            &mut target,
            &mut tangent,
            &mut binormal,
            wave,
        );
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
    const STEEPNESS_FACTOR: f32 = 0.1;
    [
        WaveProperties {
            wavelength: 60.,
            steepness: intensity * STEEPNESS_FACTOR,
            direction: Vec2::new(1.0, 0.0),
        },
        WaveProperties {
            wavelength: 31.,
            steepness: intensity * STEEPNESS_FACTOR,
            direction: Vec2::new(1.0, 0.6),
        },
        WaveProperties {
            wavelength: 18.,
            steepness: intensity * STEEPNESS_FACTOR,
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
    mut wave_probes_query: Query<(&Swimmer, &mut Transform), Without<Water>>,
    water_query: Query<(&Water, &Transform), Without<Swimmer>>,
) {
    if let Some((water, water_transform)) = water_query.iter().next() {
        for (swimmer, mut transform) in wave_probes_query.iter_mut() {
            let wavedata = water.wave_data_at_point(
                Vec2::new(transform.translation.x * 1., transform.translation.z * 1.),
                time.seconds_since_startup() as f32 * water.wave_speed,
            );
            transform.translation.y = wavedata.position.y + water_transform.translation.y;

            transform.rotation =
                surface_quat(&wavedata) * Quat::from_rotation_y(swimmer.world_rotation);
        }
    }
}

const WATER_TRANSLATE_STEP: f32 = 20.;
fn update_system(
    time: Res<Time>,
    mut water_material_query: Query<&Handle<WaterMaterial>>,
    mut water_mats: ResMut<Assets<WaterMaterial>>,
    mut water_query: Query<(&mut Water, &mut Transform), Without<PlayerBoat>>,
    boat_query: Query<(&PlayerBoat, &Transform), Without<Water>>,
    camera_query: Query<(&WaterCamera, &Transform), Without<Water>>,
) {
    if let Some(mut water_material) = water_material_query
        .get_single_mut()
        .ok()
        .and_then(|water_handle| water_mats.get_mut(water_handle))
    {
        // let mut boat_translation = Vec3::ZERO;
        // if let Ok((_, boat_transform)) = boat_query.
        // boat_translation = boat_transform.translation;
        // }
        if let Ok((water, mut water_transform)) = water_query.get_single_mut() {
            water_material.time = time.seconds_since_startup() as f32 * water.wave_speed;
            water_material.wave1 = water.waves[0].to_vec4();
            water_material.wave2 = water.waves[1].to_vec4();
            water_material.wave3 = water.waves[2].to_vec4();
            water_material.color = water.color.into();

            water_transform.translation.x = 0.;
            if let Ok((_, boat_transform)) = boat_query.get_single() {
                water_transform.translation.x = boat_transform.translation.x
                    - boat_transform.translation.x % WATER_TRANSLATE_STEP;
                water_transform.translation.z = boat_transform.translation.z
                    - boat_transform.translation.z % WATER_TRANSLATE_STEP;
            }
        }

        if let Ok((_, transform)) = camera_query.get_single() {
            water_material.camera = transform.translation;
        }
    }
}
