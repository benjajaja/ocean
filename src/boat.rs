use super::water;
use crate::water::Water;
use bevy::{prelude::*, utils::tracing::field::debug};

pub struct PlayerBoat {
    pub thrust: f32,
    pub steer: f32,
    pub world_rotation: f32, // y angle in radians
    pub speed: f32,
    pub last_normal: Quat,
    pub nose_angle: f32,
    pub airborne: Option<(Vec3, f32, f32)>,
}

#[derive(Debug)]
pub struct MoveEvent {
    pub jump: Vec3,
    pub translation: Vec3,
}

pub fn boat_physics_system(
    time: Res<Time>,
    mut boat_query: Query<(&mut PlayerBoat, &mut Transform)>,
    water_query: Query<&Water>,
    mut ev_move: EventWriter<MoveEvent>,
) {
    if let Ok((mut boat, mut boat_transform)) = boat_query.single_mut() {
        boat.world_rotation += -boat.steer * time.delta_seconds();
        let world_rotation_quat = Quat::from_rotation_y(boat.world_rotation);

        let speed = boat.thrust * time.delta_seconds() * 100.;
        // + boat.thrust * boat.nose_angle.abs();
        let thrust_vector = Vec3::new(0., 0., speed);
        let jump = world_rotation_quat * thrust_vector;

        let mut new_translation = boat_transform.translation + jump;

        boat.speed = jump.length();

        if let Ok(water) = water_query.single() {
            let wavedata = water.wave_data_at_point(
                Vec2::new(new_translation.x, new_translation.z),
                time.seconds_since_startup() as f32 * water.wave_speed,
            );
            new_translation.y = wavedata.position.y;
            boat_transform.translation = new_translation;

            let normal_quat = water::surface_quat(&wavedata);
            boat_transform.rotation = normal_quat * world_rotation_quat;
        }

        if jump.length() > 0. {
            ev_move.send(MoveEvent {
                jump,
                translation: boat_transform.translation,
            });
        }
    }
}
