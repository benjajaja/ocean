use super::water;
use crate::water::Water;
use bevy::prelude::*;
use core::f32::consts::FRAC_PI_4;

pub struct PlayerBoat {
    pub throttle: f32,
    pub steer: f32,

    pub velocity: Vec3,
    pub speed: f32,

    pub world_rotation: f32, // y angle in radians
    pub last_normal: Quat,
    pub nose_angle: f32,
    pub airborne: Option<(Vec3, f32, f32)>,
}

pub struct BoatJet;
// {
// pub transform: Transform,
// pub global_transform: GlobalTransform,
// }
// impl Default for BoatJet {
// fn default() -> Self {
// Self {
// transform: Default::default(),
// global_transform: Default::default(),
// }
// }
// }

#[derive(Debug)]
pub struct MoveEvent {
    pub jump: Vec3,
    pub translation: Vec3,
}

const DRAG: f32 = 0.2;
const FRICTION: f32 = 2.;
const ENGINE_FORCE: f32 = 2000.;
const BOAT_MASS: f32 = 20.;

pub fn boat_physics_system(
    time: Res<Time>,
    mut paddle_query: Query<(&mut BoatJet, &mut Transform), Without<PlayerBoat>>,
    mut boat_query: Query<(&mut PlayerBoat, &mut Transform), Without<BoatJet>>,
    water_query: Query<&Water>,
    mut ev_move: EventWriter<MoveEvent>,
) {
    if let Ok((mut boat, mut boat_transform)) = boat_query.single_mut() {
        let throttle_rotation = Quat::from_rotation_y(FRAC_PI_4 * boat.steer);
        for (_paddle, mut paddle_transform) in paddle_query.iter_mut() {
            paddle_transform.rotation = throttle_rotation;
        }
        boat.world_rotation += -boat.steer * time.delta_seconds();

        let world_rotation_quat = Quat::from_rotation_y(boat.world_rotation);

        let propulsion = (world_rotation_quat * Vec3::Z) * ENGINE_FORCE * boat.throttle;
        let drag = -DRAG * boat.velocity * boat.speed;
        let friction = -FRICTION * boat.velocity;

        let sum_force = propulsion + drag + friction;
        let acceleration = sum_force / BOAT_MASS;
        boat.velocity = boat.velocity + (acceleration * time.delta_seconds());
        boat.speed = boat.velocity.length();

        let jump = boat.velocity * time.delta_seconds();
        let mut new_translation = boat_transform.translation + jump;

        if let Ok(water) = water_query.single() {
            let wavedata = water.wave_data_at_point(
                Vec2::new(new_translation.x, new_translation.z),
                time.seconds_since_startup() as f32 * water.wave_speed,
            );
            let takeoff_speed = (boat.speed / 50.).clamp(0., 1.);
            new_translation.y = wavedata.position.y * (1. - takeoff_speed) + takeoff_speed * 4.;

            let normal_quat = water::surface_quat(&wavedata);
            boat_transform.rotation = boat_transform.rotation.slerp(
                normal_quat.lerp(Quat::IDENTITY, takeoff_speed)
                    * world_rotation_quat
                    * Quat::from_rotation_z(FRAC_PI_4 * boat.steer),
                time.delta_seconds() * 2.,
            );
        }
        boat_transform.translation = new_translation;

        if jump.length() > 0. {
            ev_move.send(MoveEvent {
                jump,
                translation: boat_transform.translation,
            });
        }
        return;
        boat.world_rotation += -boat.steer * time.delta_seconds();
        let world_rotation_quat = Quat::from_rotation_y(boat.world_rotation);

        boat.speed =
            (boat.speed + (boat.throttle * 2. - 1.) * time.delta_seconds() * 1.).clamp(0., 1.);
        // + boat.throttle * boat.nose_angle.abs();
        let thrust_vector = Vec3::new(0., 0., boat.speed);
        let mut jump = boat_transform.rotation * thrust_vector;
        jump.y = 0.;

        let mut new_translation = boat_transform.translation + jump;

        if let Ok(water) = water_query.single() {
            let wavedata = water.wave_data_at_point(
                Vec2::new(new_translation.x, new_translation.z),
                time.seconds_since_startup() as f32 * water.wave_speed,
            );
            new_translation.y = wavedata.position.y * (1. - boat.speed.clamp(0., 1.))
                + boat.speed.clamp(0., 1.) * 4.;

            // + boat.speed.clamp(0., 2.) * 3.;
            boat_transform.translation = new_translation;

            let normal_quat = water::surface_quat(&wavedata);
            boat_transform.rotation = boat_transform.rotation.slerp(
                normal_quat.lerp(Quat::IDENTITY, boat.speed.clamp(0., 1.)) * world_rotation_quat,
                time.delta_seconds() * 2.,
            );
        }

        if jump.length() > 0. {
            ev_move.send(MoveEvent {
                jump,
                translation: boat_transform.translation,
            });
        }
    }
}
