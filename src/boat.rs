use super::water;
use crate::water::Water;
use crate::AppState;
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use core::f32::consts::FRAC_PI_4;

#[derive(Inspectable)]
pub struct PlayerBoat {
    pub throttle: f32,
    pub steer: f32,

    pub velocity: Vec3,
    pub speed: f32,

    pub world_rotation: f32, // y angle in radians
    pub last_normal: Quat,
    pub nose_angle: f32,
    pub airborne: Option<(Vec3, f32, f32)>,

    pub exhaust_last: f64,
}

pub struct BoatJet;

#[derive(Inspectable)]
pub struct BoatExhaustParticle {
    free: bool,
}

#[derive(Debug)]
pub struct MoveEvent {
    pub jump: Vec3,
    pub translation: Vec3,
}

fn paddle_transform() -> Transform {
    Transform::from_translation(Vec3::new(0.45, 0.0, 1.9))
}

pub fn add_systems(app: &mut bevy::prelude::AppBuilder) -> &mut bevy::prelude::AppBuilder {
    app.add_startup_system(boat_startup_system.system())
        // must run after input to avoid some jankiness
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(boat_physics_system.system().label("physics").after("input"))
                .with_system(boat_exhaust_system.system()),
        )
}

fn boat_startup_system(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut materials_color: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let material_handle = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        base_color_texture: Some(asset_server.load("textures/wood.png")),
        roughness: 0.2,
        metallic: 0.5,
        reflectance: 0.8,
        // unlit: true,
        ..Default::default()
    });

    commands
        .spawn_bundle(PbrBundle {
            mesh: asset_server.load("raft.glb#Mesh0/Primitive0"),
            material: material_handle,
            ..Default::default()
        })
        .insert(PlayerBoat {
            throttle: 0.,
            steer: 0.,
            velocity: Vec3::Z,
            speed: 0.,
            world_rotation: 0.,
            last_normal: Quat::IDENTITY,
            nose_angle: 0.,
            airborne: None,
            exhaust_last: 0.,
        })
        .insert(Name::new("PlayerBoat"))
        .with_children(|parent| {
            parent
                .spawn_bundle(PbrBundle {
                    transform: paddle_transform(),
                    ..Default::default()
                })
                .insert(BoatJet)
                .insert(Name::new("BoatJet"))
                .with_children(|parent| {
                    parent.spawn_bundle(PbrBundle {
                        mesh: asset_server.load("raft.glb#Mesh1/Primitive0"),
                        material: materials.add(Color::rgb(0.1, 0.0, 0.0).into()),
                        ..Default::default()
                    });
                });

            let sail_material_handle = materials.add(StandardMaterial {
                base_color: Color::WHITE,
                double_sided: true,
                ..Default::default()
            });
            parent
                .spawn_bundle(PbrBundle {
                    mesh: asset_server.load("raft.glb#Mesh2/Primitive0"),
                    material: sail_material_handle,
                    ..Default::default()
                })
                .insert(Name::new("Sail"));
        });

    let texture = asset_server.load("splash.png");
    for i in 0..0 {
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    size: Vec2::splat(0.01),
                    ..Default::default()
                },
                material: materials_color.add(ColorMaterial {
                    texture: Some(texture.clone()),
                    ..Default::default()
                }),
                visible: Visible {
                    is_visible: false,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(BoatExhaustParticle { free: true })
            .insert(Name::new(format!("BoatExhaustParticle-{}", i)));
    }
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

        let propulsion = (world_rotation_quat * -Vec3::Z) * ENGINE_FORCE * boat.throttle;
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
            new_translation.y = wavedata.position.y; // * (1. - takeoff_speed) + takeoff_speed * 5.;

            let normal_quat = water::surface_quat(&wavedata);
            boat_transform.rotation = boat_transform.rotation.slerp(
                // normal_quat.lerp(Quat::IDENTITY, takeoff_speed)
                normal_quat * world_rotation_quat,
                // * Quat::from_rotation_z(
                // FRAC_PI_4 * -boat.steer * (boat.speed / 100.).clamp(0., 1.),
                // ), // bank
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
    }
}

pub fn boat_exhaust_system(
    time: Res<Time>,
    mut boat_query: Query<(&mut PlayerBoat, &Transform), Without<BoatExhaustParticle>>,
    mut particale_query: Query<
        (&mut BoatExhaustParticle, &mut Transform, &mut Visible),
        Without<PlayerBoat>,
    >,
) {
    if let Ok((mut boat, boat_transform)) = boat_query.single_mut() {
        let now = time.seconds_since_startup();
        if now - boat.exhaust_last > 0.1 {
            for (mut particle, mut particle_transform, mut visible) in particale_query.iter_mut() {
                if particle.free {
                    particle.free = false;
                    visible.is_visible = true;
                    let mut transform = boat_transform.clone()
                        * paddle_transform()
                        * Transform::from_xyz(0., -1., 0.);
                    transform.scale = Vec3::splat(0.01);
                    particle_transform.clone_from(&transform);
                    break;
                }
            }
            boat.exhaust_last = now;
        }
    }

    for (mut particle, mut transform, mut visible) in particale_query.iter_mut() {
        if !particle.free {
            transform.scale += Vec3::splat(-(time.delta_seconds() / 100.).clamp(0., 1.));
            if transform.scale.x <= 0. {
                visible.is_visible = false;
                particle.free = true;
            }
        }
    }
}
