use bevy::prelude::*;
use bevy_prototype_debug_lines::*;
use std::f32::consts::{FRAC_PI_2, PI};
mod boat;
use boat::PlayerBoat;
mod camera;
mod input;
use camera::CameraTracker;
mod sky;
mod stripe;
mod ui;
mod water;

fn main() {
    let mut app = App::build();
    app.add_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugLinesPlugin)
        .add_asset::<sky::SkyMaterial>()
        .add_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .add_resource(sky::SkyDome::new())
        .add_startup_system(setup.system())
        .add_startup_system(sky::spawn_sky.system())
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .add_system(input::keyboard_input_system.system())
        .add_system(input::mouse_input_system.system())
        .add_system(boat::boat_physics_system.system())
        .add_system(sky::skydome_system.system())
        .add_system(camera::camera_system.system());

    water::add_systems(&mut app);
    ui::add_systems(&mut app);
    app.run();
}

fn setup(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Setup our world
    commands
        // .spawn_scene(asset_server.load("palmera.glb"))
        .spawn(PbrBundle {
            // mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            mesh: asset_server.load("flota1.glb#Mesh0/Primitive0"),
            material: materials.add(Color::rgb(0.0, 0.9, 0.6).into()),
            // material: materials.add(StandardMaterial {
            // shaded: false,
            // ..Default::default()
            // }),
            transform: Transform::from_translation(Vec3::new(5.0, 0.0, 0.0)),
            ..Default::default()
        })
        .with(water::Swimmer {
            world_rotation: PI / 4.,
            ..Default::default()
        });

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            // mesh: asset_server.load("helios/scene.gltf#Mesh0/Primitive0"),
            material: materials.add(Color::rgb(0.5, 0.9, 0.6).into()),
            // material: materials.add(Color::rgb(0.0, 0.9, 0.6).into()),
            // material: materials.add(StandardMaterial {
            // shaded: false,
            // ..Default::default()
            // }),
            // transform: Transform::from_rotation(Quat::from_rotation_x(-PI / 2.)),
            transform: Transform::from_translation(Vec3::new(5.0, 0.0, 20.0)),
            ..Default::default()
        })
        .with(water::Swimmer {
            ..Default::default()
        });

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
            material: materials.add(StandardMaterial {
                ..Default::default()
            }),
            transform: Transform::from_translation(Vec3::new(-10.0, 0.0, 5.0)),
            ..Default::default()
        })
        .with(water::Swimmer::default());

    commands.spawn(LightBundle {
        transform: Transform::from_translation(Vec3::new(4.0, 50.0, 4.0)),
        ..Default::default()
    });

    commands
        .spawn((Transform::default(), GlobalTransform::default()))
        .with_children(|parent| {
            parent.spawn(PbrBundle {
                mesh: asset_server.load("flota1.glb#Mesh0/Primitive0"),
                material: materials.add(Color::rgb(0.2, 0.8, 0.6).into()),
                transform: Transform::from_rotation(Quat::from_rotation_y(FRAC_PI_2)), // the gltf is not looking at -z
                ..Default::default()
            });
            parent.spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
                material: materials.add(Color::rgb(1.0, 0.8, 0.6).into()),
                ..Default::default()
            });
        })
        .with(PlayerBoat {
            thrust: 0.,
            steer: 0.,
            world_rotation: 0.,
            speed: 0.,
            last_normal: Quat::identity(),
            nose_angle: 0.,
            airborne: None,
        });

    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0))
                .looking_at(Vec3::new(0.0, 5.0, 1000.0), Vec3::unit_y()),
            ..Default::default()
        })
        // .with(sky::SkyCamera)
        .with(CameraTracker {
            bobber: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            looking_up: camera::LookingUp::None,
            input_rotation: Quat::identity(),
        })
        .with(water::WaterCamera);
    println!("SkyCamera added.");
}
