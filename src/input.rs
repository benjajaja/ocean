use crate::boat;
use crate::ui;
use bevy::{input::mouse::MouseMotion, prelude::*};

pub struct CameraTracker {
    pub bobber: Transform,
    pub free_look: Option<Vec2>,
}

const INPUT_ACCEL: f32 = 10.0;
const INPUT_DECAY: f32 = 10.0;
const STEER_ACCEL: f32 = 20.0;
const BOAT_MAX_THRUST: f32 = 2.;
pub fn keyboard_input_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut boat_query: Query<&mut boat::PlayerBoat>,
    mut camera_query: Query<(&mut Transform, &mut CameraTracker)>,
    mut crosshair_query: Query<&mut Draw, With<ui::Crosshair>>,
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
            if let Some((transform, mut camera)) = camera_query.iter_mut().next() {
                camera.free_look = Some(Vec2::new(0., boat.world_rotation));

                for mut draw in crosshair_query.iter_mut() {
                    draw.is_visible = true;
                }
            }
        } else if keyboard_input.just_released(KeyCode::Space) {
            if let Some((_transform, mut camera)) = camera_query.iter_mut().next() {
                camera.free_look = None;
                for mut draw in crosshair_query.iter_mut() {
                    draw.is_visible = false;
                }
            }
        }
    }
}

/// Hold readers for events
#[derive(Default)]
pub struct InputState {
    pub reader_motion: EventReader<MouseMotion>,
}

const MOUSE_LOOK_FACTOR: f32 = 2.0;
pub fn mouse_input_system(
    time: Res<Time>,
    windows: Res<Windows>,
    mut state: Local<InputState>,
    ev_motion: Res<Events<MouseMotion>>,
    mut camera_query: Query<(&mut Transform, &mut CameraTracker)>,
) {
    if let Some((_, mut camera)) = camera_query.iter_mut().next() {
        match camera.free_look {
            None => {
                return;
            }
            Some(xy) => {
                for ev in state.reader_motion.iter(&ev_motion) {
                    println!("mouse: {}", ev.delta);
                    camera.free_look = Some();
                }
            }
        }
    }
}
