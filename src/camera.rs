use crate::boat::PlayerBoat;
use crate::sky::SkyDomeLayerBg;
use crate::water::WaterCamera;
use bevy::prelude::*;

#[derive(Component)]
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

pub fn add_systems(app: &mut bevy::prelude::App) -> &mut bevy::prelude::App {
    app.add_startup_system(camera_startup_system);
    app.add_system(camera_system.label("camera").after("physics"));
    app
}

pub fn camera_startup_system(mut commands: Commands) {
    commands
        .spawn_bundle(Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0))
                .looking_at(Vec3::default(), Vec3::Y),
            ..Default::default()
        })
        .insert(CameraTracker {
            bobber: Transform::from_translation(Vec3::new(0.0, 15.0, 0.0)),
            looking_up: LookingUp::None,
            input_rotation: Quat::IDENTITY,
        })
        .insert(WaterCamera);
}

const CAMERA_ROTATION_FACTOR: f32 = 10.0;
const CAMERA_Z_TRANSLATION: f32 = 50.0;
pub fn camera_system(
    time: Res<Time>,
    mut camera_query: Query<
        (&mut CameraTracker, &mut Transform),
        (Without<PlayerBoat>, Without<SkyDomeLayerBg>),
    >,
    mut skydome_query: Query<
        (&SkyDomeLayerBg, &mut Transform),
        (Without<PlayerBoat>, Without<CameraTracker>),
    >,
    mut boat_query: Query<(&mut PlayerBoat, &mut Transform), Without<CameraTracker>>,
) {
    if let Ok((mut camera, mut camera_transform)) = camera_query.get_single_mut() {
        if let Ok((boat, boat_transform)) = boat_query.get_single_mut() {
            camera.bobber.translation.x = boat_transform.translation.x;
            camera.bobber.translation.z = boat_transform.translation.z;
            // camera.bobber.translation.y = 15. + camera.looking_up.value() * 100.;
            camera.bobber.rotation = camera.bobber.rotation.slerp(
                Quat::from_axis_angle(Vec3::Y, boat.world_rotation).normalize()
                    * camera.input_rotation,
                time.delta_seconds() * CAMERA_ROTATION_FACTOR,
            );
        }

        let camera_z =
            CAMERA_Z_TRANSLATION - (camera.looking_up.value() * (CAMERA_Z_TRANSLATION - 0.01));

        camera_transform.translation =
            camera.bobber.translation + (camera.bobber.rotation * Vec3::new(0.0, -5.0, camera_z));

        let mut looking_at = camera.bobber.translation;
        match camera.looking_up {
            LookingUp::LookingUp(mut look) => {
                look += look + time.delta_seconds() * 0.005;
                look = look.min(1.);
                //.lo(1.1).min(1.);
                looking_at += Vec3::new(0., 1000. * look, 0.);
                camera.looking_up = LookingUp::LookingUp(look);
            }
            LookingUp::LookingDown(mut look) => {
                look -= time.delta_seconds() * 2.5;
                look = look.powf(1.1).max(0.);
                looking_at += Vec3::new(0., 1000. * look, 0.);

                if look > 0. {
                    camera.looking_up = LookingUp::LookingDown(look);
                } else {
                    camera.looking_up = LookingUp::None;
                }
            }
            LookingUp::None => {}
        }

        camera_transform.rotation = camera_transform
            .looking_at(camera.bobber.translation, Vec3::Y)
            .rotation;
        // * Quat::from_axis_angle(Vec3::X, camera.looking_up.value() * PI);

        for (_, mut sky_transform) in skydome_query.iter_mut() {
            sky_transform.translation = camera_transform.translation;
        }
    }
}
