use bevy::{
    pbr::AmbientLight,
    prelude::*,
    render::wireframe::{Wireframe, WireframePlugin},
    wgpu::{WgpuFeature, WgpuFeatures, WgpuOptions},
};
use core::f32::consts::{FRAC_PI_4, PI};
mod boat;
use boat::{BoatJet, PlayerBoat};
mod camera;
mod input;
use camera::CameraTracker;

mod sky;
mod stripe;
mod ui;
mod water;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    // Menu,
    InGame,
}

#[derive(Debug, PartialEq)]
pub enum DayTime {
    Day,
    Night,
}

#[derive(Debug)]
pub struct InGameState {
    time: DayTime,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Island {
    Home,
    IslandA,
    IslandB,
}

fn main() {
    let mut app = App::build();
    app.insert_resource(WindowDescriptor {
        width: 500.,
        height: 300.,
        scale_factor_override: Some(2.),
        // cursor_locked: true,
        cursor_visible: false,
        ..Default::default()
    });
    app.insert_resource(WgpuOptions {
        features: WgpuFeatures {
            // The Wireframe requires NonFillPolygonMode feature
            features: vec![WgpuFeature::NonFillPolygonMode],
        },
        ..Default::default()
    });

    app.insert_resource(Msaa { samples: 4 });
    app.add_plugins(DefaultPlugins);
    app.add_plugin(WireframePlugin);

    app.add_state(AppState::InGame);
    app.add_event::<NavigationEvent>();
    app.add_event::<boat::MoveEvent>();

    app.insert_resource(InGameState {
        time: DayTime::Night,
    });

    app.add_startup_system(setup.system());

    app.add_system(bevy::input::system::exit_on_esc_system.system())
        .add_system(input::keyboard_input_system.system().label("input"))
        .add_system(input::mouse_input_system.system().label("input"))
        .add_system(
            boat::boat_physics_system
                .system()
                .label("physics")
                .after("input"),
        )
        .add_system(
            camera::camera_system
                .system()
                .label("camera")
                .after("physics"),
        )
        .add_system(island_enter_leave.system());

    sky::add_systems(&mut app);
    water::add_systems(&mut app);
    ui::add_systems(&mut app);
    app.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 1.0 / 5.0f32,
    });
    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Setup our world
    commands
        .spawn_bundle(PbrBundle {
            mesh: asset_server.load("flota1.glb#Mesh0/Primitive0"),
            material: materials.add(Color::rgb(0.0, 0.9, 0.6).into()),
            transform: Transform::from_translation(Vec3::new(5.0, 0.0, 0.0)),
            ..Default::default()
        })
        .insert(water::Swimmer {
            world_rotation: PI / 4.,
            ..Default::default()
        });
    // .insert(Wireframe);

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.5, 0.9, 0.6).into()),
            transform: Transform::from_translation(Vec3::new(5.0, 0.0, 20.0)),
            ..Default::default()
        })
        .insert(water::Swimmer {
            ..Default::default()
        });

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
            material: materials.add(StandardMaterial {
                ..Default::default()
            }),
            transform: Transform::from_translation(Vec3::new(-10.0, 0.0, 5.0)),
            ..Default::default()
        })
        .insert(water::Swimmer::default());

    commands.spawn_bundle(LightBundle {
        transform: Transform::from_translation(Vec3::new(4.0, 50.0, 4.0)),
        ..Default::default()
    });

    commands
        .spawn_bundle(PbrBundle {
            mesh: asset_server.load("correolas.glb#Mesh0/Primitive0"),
            material: materials.add(Color::rgb(0.2, 0.8, 0.6).into()),
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
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(PbrBundle {
                    transform: Transform::from_translation(Vec3::new(0., 0.5, -4.)),
                    ..Default::default()
                })
                .insert(BoatJet)
                .with_children(|parent| {
                    parent.spawn_bundle(PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Box::new(0.5, 0.5, 1.))),
                        material: materials.add(Color::rgb(0.8, 0.2, 0.6).into()),
                        transform: Transform::from_translation(Vec3::new(0., 0., -0.5))
                            * Transform::from_rotation(Quat::from_rotation_z(FRAC_PI_4)),
                        ..Default::default()
                    });
                });
        });

    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0))
                .looking_at(Vec3::new(0.0, 5.0, 1000.0), Vec3::Y),
            ..Default::default()
        })
        .insert(CameraTracker {
            bobber: Transform::from_translation(Vec3::new(0.0, 5.0, 0.0)),
            looking_up: camera::LookingUp::None,
            input_rotation: Quat::IDENTITY,
        })
        .insert(water::WaterCamera);

    println!("SkyCamera added.");
}

#[derive(Debug)]
pub enum NavigationEvent {
    Enter(Island, Quat, Vec3),
    Approach(f32),
    Leave,
}

pub struct WorldIsland {
    #[allow(dead_code)]
    island: Island,
    sky_rotation: Quat,
}

fn island_enter_leave(
    mut state: ResMut<InGameState>,
    mut event_reader: EventReader<NavigationEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    worldisland_query: Query<(&WorldIsland, Entity)>,
) {
    for ev in event_reader.iter() {
        match ev {
            NavigationEvent::Enter(island, sky_rotation, translation) => match state.time {
                DayTime::Night => {
                    println!("sunrise");
                    state.time = DayTime::Day;

                    let mut palmtree_transform = Transform::from_translation(*translation);

                    palmtree_transform.scale = Vec3::new(4., 4., 4.);
                    let palmtree = PbrBundle {
                        mesh: asset_server.load("palmera.glb#Mesh3/Primitive0"),
                        material: materials.add(Color::rgb(0.9, 0.9, 0.6).into()),
                        transform: palmtree_transform,
                        ..Default::default()
                    };
                    commands.spawn_bundle(palmtree).insert(WorldIsland {
                        island: *island,
                        sky_rotation: *sky_rotation,
                    });
                }
                DayTime::Day => {
                    println!("enter at day?");
                }
            },
            NavigationEvent::Approach(_distance) => match state.time {
                DayTime::Day => {}
                DayTime::Night => {
                    panic!("approach at night");
                }
            },
            NavigationEvent::Leave => match state.time {
                DayTime::Day => {
                    println!("sunset");
                    state.time = DayTime::Night;

                    for (_, entity) in worldisland_query.iter() {
                        commands.entity(entity).despawn();
                    }
                }
                DayTime::Night => {
                    println!("leave at night");
                }
            },
        }
    }
}
