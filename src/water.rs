use bevy::prelude::*;
use std::ops::AddAssign;

pub struct Water {
    pub waves: [WaveProperties; 3],
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
    let k = 2. * std::f32::consts::PI / props.wavelength;
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

pub fn surface_quat(wavedata: WaveData, world_rotation: f32) -> Quat {
    let world_rotation = Quat::from_axis_angle(
        Vec3::unit_y(),
        world_rotation
    ).normalize();

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
    return quat * world_rotation;
}
