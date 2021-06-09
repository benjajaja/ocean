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
    mut querys: QuerySet<(
        Query<(&mut CameraTracker, &mut Transform)>,
        Query<(&PlayerBoat, &Transform)>,
        Query<(&mut super::sky::SkyDomeLayer, &mut Transform)>,
    )>,
    // boat_query: Query<(&PlayerBoat, &Transform)>,
    // mut camera_query: Query<(&mut CameraTracker, &mut Transform)>,
    // mut skydome_query: Query<(&super::sky::SkyDomeLayer, &mut Transform)>,
) {
    let mut boat_translation = Vec3::ZERO;
    let mut boat_rotation: f32 = 0.0;
    if let Some((boat, boat_transform)) = querys.q1().iter().next() {
        boat_translation = boat_transform.translation;
        boat_rotation = boat.world_rotation;
    }

    let mut camera_transform_translation = Vec3::ZERO;
    if let Some((mut camera, mut transform)) = querys.q0_mut().iter_mut().next() {
        camera.bobber.translation.x = boat_translation.x;
        camera.bobber.translation.z = boat_translation.z;

        camera.bobber.rotation = camera.bobber.rotation.slerp(
            Quat::from_axis_angle(Vec3::Y, boat_rotation).normalize() * camera.input_rotation,
            time.delta_seconds() * CAMERA_ROTATION_FACTOR,
        );

        let camera_z = -15. + (camera.looking_up.value() * 14.99);
        transform.translation =
            camera.bobber.translation + (camera.bobber.rotation * Vec3::new(0.0, 5.0, camera_z));
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

        transform.rotation = transform.looking_at(looking_at, Vec3::Y).rotation;
        camera_transform_translation = transform.translation;
    }

    for (_, mut sky_transform) in querys.q2_mut().iter_mut() {
        sky_transform.translation = camera_transform_translation;
    }
}
