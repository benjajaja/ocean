use crate::boat;
use crate::camera::{CameraTracker, LookingUp};
use crate::AppState;
use bevy::{input::mouse::MouseMotion, prelude::*};
// use bevy_inspector_egui::WorldInspectorParams;

const INPUT_ACCEL: f32 = 1.0;
const INPUT_DECAY: f32 = 4.0;
const STEER_ACCEL: f32 = 10.0;
const BOAT_MAX_THRUST: f32 = 1.0;

pub fn add_systems(app: &mut bevy::prelude::App) -> &mut bevy::prelude::App {
    app.add_system(bevy::window::close_on_esc)
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(ingame_keyboard_input_system.label("input"))
                .with_system(mouse_input_system.label("input")),
        )
        .add_system_set(
            SystemSet::on_update(AppState::Menu)
                .with_system(menu_keyboard_input_system.label("input")),
        )
}

pub fn ingame_keyboard_input_system(
    time: Res<Time>,
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut boat_query: Query<&mut boat::PlayerBoat>,
    mut camera_query: Query<(&mut Transform, &mut CameraTracker)>,
    mut state: ResMut<State<AppState>>,
    mut windows: ResMut<Windows>,
    // mut inspector_params: ResMut<WorldInspectorParams>,
) {
    if keyboard_input.just_pressed(KeyCode::E) {
        if let Ok(_) = state.set(AppState::Menu) {
            let window = windows.get_primary_mut().unwrap();
            window.set_cursor_visibility(true);
            keyboard_input.reset(KeyCode::E);
        }
    }

    // if keyboard_input.just_pressed(KeyCode::V) {
    // inspector_params.enabled = !inspector_params.enabled;
    // if let Some(window) = windows.get_primary_mut() {
    // window.set_cursor_visibility(inspector_params.enabled);
    // }
    // }

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

pub fn menu_keyboard_input_system(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut state: ResMut<State<AppState>>,
    mut windows: ResMut<Windows>,
) {
    if keyboard_input.just_pressed(KeyCode::E) {
        if let Ok(_) = state.set(AppState::InGame) {
            let window = windows.get_primary_mut().unwrap();
            window.set_cursor_visibility(false);
            keyboard_input.reset(KeyCode::E);
        }
    }
}

pub fn mouse_input_system(
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut camera_query: Query<&mut CameraTracker>,
    // inspector_params: ResMut<WorldInspectorParams>,
) {
    // if inspector_params.enabled {
    // return;
    // }
    if let Some(mut camera) = camera_query.iter_mut().next() {
        for event in mouse_motion_events.iter() {
            camera.input_rotation = (camera.input_rotation
                * Quat::from_axis_angle(Vec3::Y, -event.delta.x * 0.001)
                * Quat::from_axis_angle(Vec3::X, event.delta.y * 0.001))
            .normalize();
        }
    }
}
