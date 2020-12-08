use bevy::prelude::*;

pub struct PlayerBoat {
    pub thrust: f32,
    pub steer: f32,
    pub world_rotation: f32, // y angle in radians
    pub speed: f32,
    pub last_normal: Quat,
    pub nose_angle: f32,
    pub airborne: Option<(Vec3, f32, f32)>,
}

