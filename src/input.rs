use crate::boat;
use crate::camera::{CameraTracker, LookingUp};
use crate::AppState;
use bevy::{input::mouse::MouseMotion, prelude::*};

const INPUT_ACCEL: f32 = 1.0;
const INPUT_DECAY: f32 = 4.0;
const STEER_ACCEL: f32 = 10.0;
const BOAT_MAX_THRUST: f32 = 1.0;

pub fn add_systems(app: &mut bevy::prelude::AppBuilder) -> &mut bevy::prelude::AppBuilder {
    app.add_system(bevy::input::system::exit_on_esc_system.system())
        .add_system(keyboard_input_system.system().label("input"))
        .add_system(mouse_input_system.system().label("input"))
}

pub fn keyboard_input_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut boat_query: Query<&mut boat::PlayerBoat>,
    mut camera_query: Query<(&mut Transform, &mut CameraTracker)>,
    mut state: ResMut<State<AppState>>,
    mut windows: ResMut<Windows>,
) {
    if keyboard_input.just_pressed(KeyCode::E) {
        let window = windows.get_primary_mut().unwrap();
        if state.current().to_owned() == AppState::InGame {
            state.set(AppState::Menu).unwrap();
            window.set_cursor_visibility(true);
        } else {
            state.set(AppState::InGame).unwrap();
            window.set_cursor_visibility(false);
        }
        println!("Changed: {:?}", state.current());
    }

    for mut boat in &mut boat_query.iter_mut() {
        if keyboard_input.pressed(KeyCode::W) {
            if boat.throttle < BOAT_MAX_THRUST {
                boat.throttle =
                    (boat.throttle + INPUT_ACCEL * time.delta_seconds()).min(BOAT_MAX_THRUST);
            }
        } else if boat.throttle > 0.0 {
            boat.throttle = (boat.throttle - INPUT_DECAY * time.delta_seconds()).max(0.0);
        }
        if keyboard_input.pressed(KeyCode::S) {
            if boat.throttle > -BOAT_MAX_THRUST / 2. {
                boat.throttle =
                    (boat.throttle - INPUT_ACCEL * time.delta_seconds()).max(-BOAT_MAX_THRUST);
            }
        } else if boat.throttle < 0.0 {
            boat.throttle = (boat.throttle + INPUT_DECAY * time.delta_seconds()).max(0.0);
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
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut camera_query: Query<&mut CameraTracker>,
) {
    if let Some(mut camera) = camera_query.iter_mut().next() {
        for event in mouse_motion_events.iter() {
            camera.input_rotation = (camera.input_rotation
                * Quat::from_axis_angle(Vec3::Y, -event.delta.x * 0.001)
                * Quat::from_axis_angle(Vec3::X, event.delta.y * 0.001))
            .normalize();
        }
    }
}
