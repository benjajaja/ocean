use crate::sky::SkyDome;
use crate::DayTime;
use crate::InGameState;

use super::water;
use bevy::prelude::*;
use std::f32::consts::{FRAC_PI_2, PI};

pub struct PlayerBoat {
    pub thrust: f32,
    pub steer: f32,
    pub world_rotation: f32, // y angle in radians
    pub speed: f32,
    pub last_normal: Quat,
    pub nose_angle: f32,
    pub airborne: Option<(Vec3, f32, f32)>,
}

const WATER_TRANSLATE_STEP: f32 = 20.;
pub fn boat_physics_system(
    time: Res<Time>,
    mut boat_query: Query<(&mut PlayerBoat, &mut Transform)>,
    mut water_transform_query: Query<(&mut water::Water, &mut Transform)>,
    mut skydome: ResMut<SkyDome>,
    state: Res<InGameState>,
    worldisland_query: Query<(&super::WorldIsland, &Transform)>,
) {
    if let Some((mut boat, mut boat_transform)) = boat_query.iter_mut().next() {
        if let Some((water, mut water_transform)) = water_transform_query.iter_mut().next() {
            boat.world_rotation += -boat.steer * time.delta_seconds();
            let world_rotation_quat = Quat::from_rotation_y(boat.world_rotation);

            let speed = boat.thrust * time.delta_seconds() * 100.;
            // + boat.thrust * boat.nose_angle.abs();
            let thrust_vector = Vec3::new(0., 0., speed);
            let jump = world_rotation_quat * thrust_vector;

            let new_translation = boat_transform.translation + jump;

            boat.speed = jump.length();

            // TODO: make weather_update_system
            // water::set_waves(&mut water, weather.wave_intensity);
            // water_material.wave1 = water.waves[0].to_vec4();
            // water_material.wave2 = water.waves[1].to_vec4();
            // water_material.wave3 = water.waves[2].to_vec4();

            // move water plane along in steps to avoid vertex jither
            water_transform.translation.x =
                new_translation.x - new_translation.x % WATER_TRANSLATE_STEP;
            water_transform.translation.z =
                new_translation.z - new_translation.z % WATER_TRANSLATE_STEP;
            let wavedata = water.wave_data_at_point(
                Vec2::new(new_translation.x, new_translation.z),
                time.seconds_since_startup() as f32 * water.wave_speed,
            );

            if let Some((_origin, _radians, _t)) = boat.airborne {
                // let tt = t + time.delta_seconds();
                // let new_y =
                // (origin.y + boat.speed * tt * radians.sin() - 0.5 * 9.81 * tt * tt) * -1.;
                boat.airborne = None;
                println!(
                    "airborne ended {}/{}",
                    wavedata.position.y, water_transform.translation.y
                );
            // water_transform.translation.y = new_y;
            // if new_y > boat_transform.translation.y && wavedata.position.y >= -water_transform.translation.y {
            // boat.airborne = None;
            // println!("airborne ended {}/{}", wavedata.position.y, water_transform.translation.y);
            // } else {
            // boat.airborne = Some((origin, radians, tt));
            } else {
                let normal_quat = water::surface_quat(&wavedata);
                let world_rotation = normal_quat * world_rotation_quat;

                let forward_vec = world_rotation * Vec3::unit_z();
                let cosine = normal_quat.dot(boat.last_normal);
                boat.nose_angle = cosine;
                boat.last_normal = normal_quat;

                // "anchor" water plane at boat
                water_transform.translation.y = -wavedata.position.y;

                if forward_vec.y > 0. && boat.speed > 1.0 && false {
                    // boat.nose_angle > PI / 8. {
                    // boat.airborne = Some((forward_vec, boat.nose_angle, 0.));
                    println!("airborne now");
                    boat_transform.rotation = boat.last_normal * world_rotation_quat;
                } else {
                    boat_transform.rotation = normal_quat * world_rotation_quat;
                }
            }
            boat_transform.translation = new_translation;

            // rotate skydomes from boat jump only at night (change to events?)
            match state.time {
                DayTime::Night => {
                    skydome.rotation = (jump_skydome(jump) * skydome.rotation).normalize();
                }
                DayTime::Day => {
                    if let Some((island, island_transform)) = worldisland_query.iter().next() {
                        skydome.rotation = position_skydome(
                            boat_transform.translation,
                            island_transform.translation,
                            island.sky_rotation,
                        );
                    }
                }
            }
        }
    }
}

fn jump_skydome(jump: Vec3) -> Quat {
    let right_angle = Quat::from_rotation_y(FRAC_PI_2);
    let rotation_axis = right_angle * jump;
    let rotation = Quat::from_axis_angle(rotation_axis, -jump.length() * 0.0001);
    rotation
}

fn position_skydome(boat: Vec3, island: Vec3, sky_rotation: Quat) -> Quat {
    let pointer = sky_rotation * Vec3::unit_z();
    let mut transform = Transform::identity();
    transform.look_at(pointer, Vec3::unit_y());
    transform.rotation * Quat::from_rotation_y(PI)
}
