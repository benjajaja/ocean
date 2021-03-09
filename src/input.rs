use crate::boat;
use crate::camera::{CameraTracker, LookingUp};
use bevy::{input::mouse::MouseMotion, prelude::*};

#[derive(Default)]
pub struct State {
    pub mouse_motion_event_reader: EventReader<MouseMotion>,
}

const INPUT_ACCEL: f32 = 10.0;
const INPUT_DECAY: f32 = 10.0;
const STEER_ACCEL: f32 = 10.0;
const BOAT_MAX_THRUST: f32 = 2.;
pub fn keyboard_input_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut boat_query: Query<&mut boat::PlayerBoat>,
    mut camera_query: Query<(&mut Transform, &mut CameraTracker)>,
) {
    for mut boat in &mut boat_query.iter_mut() {
        if keyboard_input.pressed(KeyCode::W) {
            if boat.thrust < BOAT_MAX_THRUST {
                boat.thrust =
                    (boat.thrust + INPUT_ACCEL * time.delta_seconds()).min(BOAT_MAX_THRUST);
            }
        } else if boat.thrust > 0.0 {
            boat.thrust = (boat.thrust - INPUT_DECAY * time.delta_seconds()).max(0.0);
        }

        if keyboard_input.pressed(KeyCode::A) {
            if boat.steer > -1.0 {
                boat.steer = (boat.steer - STEER_ACCEL * time.delta_seconds()).max(-1.0);
            }
        } else if boat.steer < 0.0 {
            boat.steer = (boat.steer + INPUT_DECAY * time.delta_seconds()).min(0.0);
        }
        if keyboard_input.pressed(KeyCode::D) {
            if boat.steer < 1.0 {
                boat.steer = (boat.steer + STEER_ACCEL * time.delta_seconds()).min(1.0);
            }
        } else if boat.steer > 0.0 {
            boat.steer = (boat.steer - INPUT_DECAY * time.delta_seconds()).max(0.0);
        }

        if keyboard_input.just_pressed(KeyCode::Space) {
            if let Some((_transform, mut camera)) = camera_query.iter_mut().next() {
                camera.looking_up = LookingUp::LookingUp(camera.looking_up.value());
            }
        } else if keyboard_input.just_released(KeyCode::Space) {
            if let Some((_transform, mut camera)) = camera_query.iter_mut().next() {
                camera.looking_up = LookingUp::LookingDown(camera.looking_up.value());
            }
        }
    }
}

pub fn mouse_input_system(
    mut state: Local<State>,
    mouse_motion_events: Res<Events<MouseMotion>>,
    mut camera_query: Query<&mut CameraTracker>,
) {
    if let Some(mut camera) = camera_query.iter_mut().next() {
        for event in state.mouse_motion_event_reader.iter(&mouse_motion_events) {
            camera.input_rotation = (camera.input_rotation
                * Quat::from_axis_angle(Vec3::unit_y(), -event.delta.x * 0.001)
                * Quat::from_axis_angle(Vec3::unit_x(), event.delta.y * 0.001))
            .normalize();
        }
    }
}
