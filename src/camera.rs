use crate::boat::PlayerBoat;
use crate::sky::SkyDomeLayer;
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

pub fn camera_system(
    time: Res<Time>,
    mut camera_query: Query<
        (&mut CameraTracker, &mut Transform),
        (Without<PlayerBoat>, Without<SkyDomeLayer>),
    >,
    mut skydome_query: Query<
        (&SkyDomeLayer, &mut Transform),
        (Without<PlayerBoat>, Without<CameraTracker>),
    >,
) {
    if let Ok((mut camera, mut camera_transform)) = camera_query.single_mut() {
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

        camera_transform.rotation = camera_transform.looking_at(looking_at, Vec3::Y).rotation;

        for (_, mut sky_transform) in skydome_query.iter_mut() {
            sky_transform.translation = camera_transform.translation;
        }
    }
}
