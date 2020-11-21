use bevy::prelude::*;

pub struct Water {
    pub waves: [WaveProperties; 3],
}
impl Water {
    pub fn height_at_point(self: &Self, point: Vec2, time: f32) -> f32{
        let input_point = Vec3::new(point.x, 0., point.y);

        let first_pass = wave_sequence(input_point, time, &self.waves);
        first_pass.y
    }
}

#[derive(Debug)]
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
    pub fn to_vec4(self: &Self) -> Vec4 {
        Vec4::new(self.direction.x, self.direction.y, self.wavelength, self.steepness)
    }
}

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

fn wave_sequence(position: Vec3, time: f32, waves: &[WaveProperties; 3]) -> Vec3 {
    let mut position = position;
    for wave in waves {
        position = gerstner_wave(position, time, wave);
    }
    position
}

pub fn set_waves(water: &mut Water, intensity: f32) -> () {
    water.waves = get_waves(intensity);
}

pub fn get_waves(intensity: f32) -> [WaveProperties; 3] {
    [
        WaveProperties::new(intensity,
                            intensity / 250.,
                            Vec2::new(1.0, 0.0)),
        WaveProperties::new(intensity / 5.,
                            intensity / 200.,
                            Vec2::new(0.1, 0.9)),
        WaveProperties::new(intensity / 33.3,
                            intensity / 333.,
                            Vec2::new(0.1, -0.2)),
    ]
}
