extern crate lazy_static_include;
use bevy::{pbr::AmbientLight, prelude::*};

mod boat;
mod camera;
mod input;
mod sky;
mod ui;
mod water;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    InGame,
    Menu,
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
    let mut app = App::new();
    app.insert_resource(WindowDescriptor {
        width: 500.,
        height: 300.,
        scale_factor_override: Some(2.),
        // cursor_locked: true,
        cursor_visible: false,
        ..Default::default()
    });

    app.insert_resource(Msaa { samples: 4 });
    app.add_plugins(DefaultPlugins);
    // app.add_plugin(WorldInspectorPlugin::new());
    // let mut registry = app
    // .world_mut()
    // .get_resource_or_insert_with(InspectableRegistry::default);
    // registry.register::<water::Water>();
    // app.insert_resource(WorldInspectorParams {
    // sort_components: true,
    // enabled: false,
    // ..Default::default()
    // });

    app.add_state(AppState::InGame);
    app.add_event::<NavigationEvent>();
    app.add_event::<boat::MoveEvent>();

    app.insert_resource(InGameState {
        time: DayTime::Night,
    });
    app.add_startup_system(setup);

    camera::add_systems(&mut app);
    input::add_systems(&mut app);

    app.add_system(island_enter_leave);

    boat::add_systems(&mut app);
    sky::add_systems(&mut app);
    water::add_systems(&mut app);
    ui::add_systems(&mut app);
    app.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.01,
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
            transform: Transform::from_translation(Vec3::new(10., 10., -10.)),
            ..Default::default()
        })
        .insert(Name::new("Flotante1"))
        .insert(water::Swimmer::default());

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.5, 0.9, 0.6).into()),
            transform: Transform::from_translation(Vec3::new(5.0, 0.0, 20.0)),
            ..Default::default()
        })
        .insert(Name::new("Flotante2"))
        .insert(water::Swimmer::default());

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
            material: materials.add(StandardMaterial {
                ..Default::default()
            }),
            transform: Transform::from_translation(Vec3::new(-10.0, 0.0, 5.0)),
            ..Default::default()
        })
        .insert(Name::new("Flotante3"))
        .insert(water::Swimmer::default());

    let theta = std::f32::consts::FRAC_PI_4;
    let light_transform = Mat4::from_euler(EulerRot::ZYX, 0.0, std::f32::consts::FRAC_PI_2, -theta);
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 10000.0,
            shadow_projection: OrthographicProjection {
                left: -10.0,
                right: 10.0,
                bottom: -10.0,
                top: 10.0,
                near: -50.0,
                far: 50.0,
                ..Default::default()
            },
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_matrix(light_transform),
        ..Default::default()
    });
}

#[derive(Debug)]
pub enum NavigationEvent {
    Enter(Island, Quat, Vec3),
    Approach(f32),
    Leave,
}

#[derive(Component)]
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
    // mut scene_spawner: ResMut<SceneSpawner>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
    worldisland_query: Query<(&WorldIsland, Entity)>,
) {
    for ev in event_reader.iter() {
        match ev {
            NavigationEvent::Enter(island, sky_rotation, translation) => match state.time {
                DayTime::Night => {
                    println!("sunrise {:?}", island);
                    state.time = DayTime::Day;

                    let mut palmtree_transform = Transform::from_translation(*translation);

                    palmtree_transform.scale = Vec3::new(1., 1., 1.);
                    // let palmtree = PbrBundle {
                    // mesh: asset_server.load("palmera.glb#Mesh3/Primitive0"),
                    // material: materials.add(Color::rgb(0.9, 0.9, 0.6).into()),
                    // transform: palmtree_transform,
                    // ..Default::default()
                    // };
                    let scene_handle = asset_server.load("palmera2.glb#Scene0");
                    commands
                        .spawn_bundle((palmtree_transform, GlobalTransform::identity()))
                        .insert(WorldIsland {
                            island: *island,
                            sky_rotation: *sky_rotation,
                        })
                        .with_children(|parent| {
                            parent.spawn_bundle(SceneBundle {
                                scene: scene_handle,
                                ..Default::default()
                            });
                            // scene_spawner.spawn_as_child(scene_handle, parent.parent_entity());
                            // scene_spawner.spawn_dynamic(scene_handle);
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
                        commands.entity(entity).despawn_recursive();
                        println!("despawn {:?}", entity);
                    }
                }
                DayTime::Night => {
                    println!("leave at night");
                }
            },
        }
    }
}
