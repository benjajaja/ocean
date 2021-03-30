use crate::boat::PlayerBoat;
use bevy::prelude::*;

pub struct CameraTracker {
    pub bobber: Transform,
    pub looking_up: LookingUp,
    pub input_rotation: Quat,
}
pub enum LookingUp {
    None,
    LookingUp(f32),
    LookingDown(f32),
}
impl LookingUp {
    pub fn value(&self) -> f32 {
        match *self {
            LookingUp::None => 0.,
            LookingUp::LookingUp(a) => a,
            LookingUp::LookingDown(a) => a,
        }
    }
}

const CAMERA_ROTATION_FACTOR: f32 = 10.0;
pub fn camera_system(
    time: Res<Time>,
    boat_query: Query<(&PlayerBoat, &Transform)>,
    mut camera_query: Query<(&mut Transform, &mut CameraTracker)>,
    mut skydome_query: Query<(&super::sky::SkyDomeLayer, &mut Transform)>,
) {
    if let Some((mut transform, mut camera)) = camera_query.iter_mut().next() {
        if let Some((boat, boat_transform)) = boat_query.iter().next() {
            let boat_translation = boat_transform.translation;

            camera.bobber.translation.x = boat_translation.x;
            camera.bobber.translation.z = boat_translation.z;

            camera.bobber.rotation = camera.bobber.rotation.slerp(
                Quat::from_axis_angle(Vec3::unit_y(), boat.world_rotation).normalize()
                    * camera.input_rotation,
                time.delta_seconds() * CAMERA_ROTATION_FACTOR,
            );

            transform.translation =
                camera.bobber.translation + (camera.bobber.rotation * Vec3::new(0.0, 5.0, -15.0));
            // + Vec3::new(0.0, -boat.thrust * 1.5, 0.0);

            let mut looking_at = camera.bobber.translation;
            match camera.looking_up {
                LookingUp::LookingUp(mut look) => {
                    look += look + time.delta_seconds() * 0.5;
                    look = look.min(1.);
                    looking_at += Vec3::new(0., 100. * look, 0.);
                    camera.looking_up = LookingUp::LookingUp(look);
                }
                LookingUp::LookingDown(mut look) => {
                    look -= time.delta_seconds() * 2.5;
                    look = look.max(0.);
                    looking_at += Vec3::new(0., 100. * look, 0.);

                    if look > 0. {
                        camera.looking_up = LookingUp::LookingDown(look);
                    } else {
                        camera.looking_up = LookingUp::None;
                    }
                }
                LookingUp::None => {}
            }

            transform.rotation = transform.looking_at(looking_at, Vec3::unit_y()).rotation;
            for (_, mut sky_transform) in skydome_query.iter_mut() {
                sky_transform.translation = transform.translation;
            }
        }
    }
}
