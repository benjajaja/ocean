use bevy::prelude::*;

pub struct WaveProperties {
    pub wavelength: f32,
    pub steepness: f32,
    pub direction: Vec2,
}

impl WaveProperties {
    pub fn new(wavelength: f32, steepness: f32, direction: Vec2) -> Self {
        Self {
            wavelength, steepness, direction
        }
    }
}

// float wavelength = 1.5;
// float steepness = 0.4;
// vec2 direction = vec2(1., -0.6);
fn gerstner_wave(
    position: Vec3,
    time: f32,
    props: &WaveProperties
) -> Vec3 {
    let d = props.direction.normalize();

    let position_xz = Vec2::new(position.x, position.z);
    let k = 2. * std::f32::consts::PI / props.wavelength;
    let c = (9.8 / k).sqrt(); // Wave speed
    let f = k * (position_xz.dot(d) - c * time);
    let amp_noise = 1.;
    let a = props.steepness / k * amp_noise;

    Vec3::new(
        position.x + d.x * (a * f.cos()),
        position.y + a * f.sin(),
        position.z + d.y * (a * f.cos())
    )
}

fn wave_sequence(position: Vec3, time: f32, waves: &[WaveProperties]) -> Vec3 {
    let mut position = position;
    for wave in waves {
        position = gerstner_wave(position, time, wave);
    }
    position
}

pub fn height_at_point(point: Vec2, time: f32) -> f32{
    let waves = &[
        WaveProperties::new(50., 0.2, Vec2::new(1.0, 0.0)),
        WaveProperties::new(10., 0.25, Vec2::new(0.1, 0.9)),
        WaveProperties::new(1.5, 0.15, Vec2::new(0.1, -0.2)),
    ];

    let input_point = Vec3::new(point.x, 0., point.y);

    let first_pass = wave_sequence(input_point, time, waves);
    first_pass.y
    //
    // let new_point = Vec3::new(first_pass.x, 0., first_pass.y);
    // wave_sequence(
        // new_point,
        // time,
        // waves
    // ).y
}

