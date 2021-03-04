use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::{
        pipeline::{PipelineDescriptor, RenderPipeline},
        render_graph::{base, AssetRenderResourcesNode, RenderGraph},
        renderer::RenderResources,
        shader::{ShaderStage, ShaderStages},
    },
};
use std::f32::consts::{FRAC_PI_2, PI};
mod boat;
use boat::PlayerBoat;
mod stripe;
mod ui;
mod water;

fn main() {
    let mut app = App::build();
    let islands = vec![
        SkyDomeIsland {
            rotation: Quat::from_rotation_x(FRAC_PI_2),
        },
        SkyDomeIsland {
            rotation: Quat::identity(),
        },
    ];
    app.add_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_asset::<SkyMaterial>()
        .add_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .add_resource(SkyDome {
            rotation: Quat::identity(),
            islands,
        })
        .add_startup_system(setup.system())
        .add_startup_system(spawn_sky.system())
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .add_system(keyboard_input_system.system())
        .add_system(boat::boat_physics_system.system())
        .add_system(skydome_system.system())
        .add_system(camera_system.system());

    water::add_systems(&mut app);
    ui::add_systems(&mut app);
    app.run();
}

struct CameraTracker {
    bobber: Transform,
    looking_up: LookingUp,
}
enum LookingUp {
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

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "0320b9b8-dead-beef-8bfa-c94008177b17"]
struct SkyMaterial {
    texture: Handle<Texture>,
}
pub struct SkyDomeLayer;

pub struct SkyDome {
    rotation: Quat,
    islands: Vec<SkyDomeIsland>,
}
pub struct SkyDomeIsland {
    rotation: Quat,
}

const SKY_VERTEX_SHADER: &str = include_str!("../assets/shaders/sky.vert");
const SKY_FRAGMENT_SHADER: &str = include_str!("../assets/shaders/sky.frag");

fn setup(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // let palmtree = asset_server.load("palmera.glb");
    let mut palmtree_transform = Transform::from_translation(Vec3::new(-5.0, -3.0, 5.0));
    palmtree_transform.scale = Vec3::new(4., 4., 4.);
    let palmtree = PbrBundle {
        mesh: asset_server.load("palmera.glb#Mesh3/Primitive0"),
        material: materials.add(Color::rgb(0.9, 0.9, 0.6).into()),
        // material: materials.add(Color::rgb(0.8, 0.5, 0.0).into()),
        transform: palmtree_transform,
        ..Default::default()
    };

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
        })
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
        })
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
            material: materials.add(StandardMaterial {
                ..Default::default()
            }),
            transform: Transform::from_translation(Vec3::new(-10.0, 0.0, 5.0)),
            ..Default::default()
        })
        .with(water::Swimmer::default())
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 50.0, 4.0)),
            ..Default::default()
        })
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
        })
        .spawn(Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0))
                .looking_at(Vec3::new(0.0, 5.0, 1000.0), Vec3::unit_y()),
            ..Default::default()
        })
        .with(CameraTracker {
            bobber: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            looking_up: LookingUp::None,
        })
        .with(water::WaterCamera);
}

fn spawn_sky(
    commands: &mut Commands,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut sky_materials: ResMut<Assets<SkyMaterial>>,
    mut render_graph: ResMut<RenderGraph>,
    asset_server: Res<AssetServer>,
    skydome: Res<SkyDome>,
) {
    let sky_pipeline_handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        vertex: shaders.add(Shader::from_glsl(ShaderStage::Vertex, SKY_VERTEX_SHADER)),
        fragment: Some(shaders.add(Shader::from_glsl(
            ShaderStage::Fragment,
            SKY_FRAGMENT_SHADER,
        ))),
    }));
    render_graph.add_system_node(
        "SkyMaterial",
        AssetRenderResourcesNode::<SkyMaterial>::new(true),
    );
    render_graph
        .add_node_edge("SkyMaterial", base::node::MAIN_PASS)
        .unwrap();

    let texture_handle: Handle<Texture> = asset_server.load("star.png");
    let texture_handle_islands: Handle<Texture> = asset_server.load("palmtree_sky.png");

    let render_pipelines =
        RenderPipelines::from_pipelines(vec![RenderPipeline::new(sky_pipeline_handle)]);

    let sky_material = sky_materials.add(SkyMaterial {
        texture: texture_handle,
    });

    let sky_material_islands = sky_materials.add(SkyMaterial {
        texture: texture_handle_islands,
    });

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(stripe::bg_stars()),
            render_pipelines: render_pipelines.clone(),
            ..Default::default()
        })
        .with(sky_material)
        .with(SkyDomeLayer);

    let island_stars: Vec<stripe::StarDef> = skydome
        .islands
        .iter()
        .map(|island| stripe::StarDef {
            quat: island.rotation,
            size: 0.025,
        })
        .collect();
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(stripe::island_stars(island_stars)),
            render_pipelines: render_pipelines.clone(),
            ..Default::default()
        })
        .with(sky_material_islands)
        .with(SkyDomeLayer);
}

const INPUT_ACCEL: f32 = 10.0;
const INPUT_DECAY: f32 = 10.0;
const STEER_ACCEL: f32 = 20.0;
const BOAT_MAX_THRUST: f32 = 2.;
fn keyboard_input_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut boat_query: Query<&mut PlayerBoat>,
    mut camera_query: Query<(&mut Transform, &mut CameraTracker)>,
) {
    for mut boat in &mut boat_query.iter_mut() {
        let mut print = false;

        if keyboard_input.pressed(KeyCode::W) {
            if boat.thrust < BOAT_MAX_THRUST {
                boat.thrust =
                    (boat.thrust + INPUT_ACCEL * time.delta_seconds()).min(BOAT_MAX_THRUST);
                print = true;
            }
        } else if boat.thrust > 0.0 {
            boat.thrust = (boat.thrust - INPUT_DECAY * time.delta_seconds()).max(0.0);
            print = true;
        }

        if keyboard_input.pressed(KeyCode::A) {
            if boat.steer > -1.0 {
                boat.steer = (boat.steer - STEER_ACCEL * time.delta_seconds()).max(-1.0);
                print = true;
            }
        } else if boat.steer < 0.0 {
            boat.steer = (boat.steer + INPUT_DECAY * time.delta_seconds()).min(0.0);
            print = true;
        }
        if keyboard_input.pressed(KeyCode::D) {
            if boat.steer < 1.0 {
                boat.steer = (boat.steer + STEER_ACCEL * time.delta_seconds()).min(1.0);
                print = true;
            }
        } else if boat.steer > 0.0 {
            boat.steer = (boat.steer - INPUT_DECAY * time.delta_seconds()).max(0.0);
            print = true;
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

        // if print {
        // println!("boat {} / {}", boat.thrust, boat.steer);
        // }
    }
}

const CAMERA_ROTATION_FACTOR: f32 = 10.0;
fn camera_system(
    time: Res<Time>,
    boat_query: Query<(&PlayerBoat, &Transform)>,
    mut camera_query: Query<(&mut Transform, &mut CameraTracker)>,
) {
    if let Some((mut transform, mut camera)) = camera_query.iter_mut().next() {
        if let Some((boat, boat_transform)) = boat_query.iter().next() {
            let boat_translation = boat_transform.translation;

            camera.bobber.translation.x = boat_translation.x;
            camera.bobber.translation.z = boat_translation.z;

            camera.bobber.rotation = camera.bobber.rotation.slerp(
                Quat::from_axis_angle(Vec3::unit_y(), boat.world_rotation).normalize(),
                time.delta_seconds() * CAMERA_ROTATION_FACTOR,
            );

            transform.translation = camera.bobber.translation
                + camera.bobber.rotation.mul_vec3(Vec3::new(0.0, 5.0, -15.0))
                + Vec3::new(0.0, -boat.thrust * 1.5, 0.0);

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
        }
    }
}

fn skydome_system(
    skydome: Res<SkyDome>,
    mut skydome_query: Query<(&SkyDomeLayer, &mut Transform)>,
) {
    for (_, mut sky_transform) in skydome_query.iter_mut() {
        sky_transform.rotation = skydome.rotation;
    }

    let sky_vec = skydome.rotation * Vec3::unit_z();
    // let sky_inverse = skydome.rotation.conjugate();
    for island in skydome.islands.iter() {
        let island_vec = island.rotation * Vec3::unit_z();
        let angle = island_vec.dot(sky_vec);
        println!("angle: {:?}", angle);
    }
}
