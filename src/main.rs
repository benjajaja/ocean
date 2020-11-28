use bevy::{
    prelude::*,
    render::{
        pipeline::{PipelineDescriptor, RenderPipeline},
        render_graph::{base, AssetRenderResourcesNode, RenderGraph},
        renderer::RenderResources,
        shader::{ShaderStage, ShaderStages},
    },
    type_registry::TypeUuid,
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
};
mod water;
mod stripe;

/// This example illustrates how to add a custom attribute to a mesh and use it in a custom shader.
fn main() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_asset::<WaterMaterial>()
        .add_asset::<SkyMaterial>()
        .add_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .add_resource(Weather {
            wave_intensity: 50.,
        })
        .add_startup_system(setup.system())
        .add_startup_system(spawn_sky.system())
        .add_system(bevy::input::system::exit_on_esc_system.system())

        .add_system(water_update_system.system())
        .add_system(keyboard_input_system.system())
        .add_system(boat_physics_system.system())
        .add_system(camera_system.system())
        .add_system(wave_probe_system.system())

        .add_system(text_update_system)
        .run();
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

struct Player;
struct PlayerBoat {
    thrust: f32,
    steer: f32,
}
struct WaveProbe;
struct WaterFloor;

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "0320b9b8-b3a3-4baa-8bfa-c94008177b17"]
struct WaterMaterial {
    pub time: f32,
    pub color: Vec4,
    pub camera: Vec3,
    pub wave1: Vec4,
    pub wave2: Vec4,
    pub wave3: Vec4,
}

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "0320b9b8-dead-beef-8bfa-c94008177b17"]
struct SkyMaterial {
    texture: Handle<Texture>,
}
struct SkyDome;

struct Weather {
    wave_intensity: f32,
}

struct FpsText;

const WATER_VERTEX_SHADER: &str = include_str!("../assets/shaders/water.vert");
const WATER_FRAGMENT_SHADER: &str = include_str!("../assets/shaders/water.frag");
const SKY_VERTEX_SHADER: &str = include_str!("../assets/shaders/sky.vert");
const SKY_FRAGMENT_SHADER: &str = include_str!("../assets/shaders/sky.frag");

const WAVE_SPEED: f32 = 0.8;

fn setup(
    commands: &mut Commands,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut water_materials: ResMut<Assets<WaterMaterial>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut render_graph: ResMut<RenderGraph>,
    asset_server: Res<AssetServer>,
    weather: Res<Weather>,
) {
    let water_pipeline_handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        vertex: shaders.add(Shader::from_glsl(ShaderStage::Vertex, WATER_VERTEX_SHADER)),
        fragment: Some(shaders.add(Shader::from_glsl(ShaderStage::Fragment, WATER_FRAGMENT_SHADER))),
    }));
    render_graph.add_system_node(
        "WaterMaterial",
        AssetRenderResourcesNode::<WaterMaterial>::new(true),
    );
    render_graph.add_node_edge(
        "WaterMaterial",
        base::node::MAIN_PASS,
    ).unwrap();


    // let palmtree = asset_server.load("palmera.glb");
    let mut palmtree_transform = Transform::from_translation(Vec3::new(-5.0, -3.0, 5.0));
    palmtree_transform.scale = Vec3::new(4., 4., 4.);
    let palmtree = PbrBundle {
        mesh: asset_server.load("palmera.glb#Mesh3/Primitive0"),
        // material: materials.add(Color::rgb(0.8, 0.5, 0.0).into()),
        transform: palmtree_transform,
        ..Default::default()
    };

    let mut water = water::Water {
        waves: water::get_waves(weather.wave_intensity),
    };
    water::set_waves(&mut water, weather.wave_intensity);

    let debug_waves = water::get_waves(weather.wave_intensity);
    println!("first wave: {} / {}", debug_waves[0].wavelength, debug_waves[0].steepness);

    let camera = Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0))
                .looking_at(Vec3::new(0.0, 5.0, 1000.0), Vec3::unit_y()),
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
        .with(WaveProbe)

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
        .with(WaveProbe)

        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
            material: materials.add(StandardMaterial {
                ..Default::default()
            }),
            transform: Transform::from_translation(Vec3::new(-10.0, 0.0, 5.0)),
            ..Default::default()
        })
        .with(WaveProbe)

        // .with(WaveProbe)

        .spawn(palmtree)
        // .with(WaterFloor)

        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 50.0, 4.0)),
            ..Default::default()
        })

        .spawn(MeshBundle {
            mesh: asset_server.load("plano.glb#Mesh0/Primitive0"),
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                water_pipeline_handle,
            )]),
            transform: Transform::from_scale(Vec3::new(200.0, 200.0, 200.0)),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Plane { size: 500.0 })),
                // material: materials.add(Color::rgb(0.0, 0.0, 0.6).into()),
                material: materials.add(StandardMaterial {
                    shaded: false,
                    albedo: Color::rgb(0.2, 0.0, 0.6).into(),
                    ..Default::default()
                }),
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
                ..Default::default()
            });
        })
        .with(water_materials.add(WaterMaterial {
            time: 0.,
            color: Vec4::new(0.1, 0.0, 0.5, 1.0),
            camera: Vec3::new(0., 0., 0.),
            wave1: water.waves[0].to_vec4(),
            wave2: water.waves[1].to_vec4(),
            wave3: water.waves[2].to_vec4(),
        }))
        .with(water)

        .spawn(PbrBundle {
            // mesh: asset_server.load("flota1.glb#Mesh0/Primitive0"),
            // material: materials.add(Color::rgb(0.2, 0.8, 0.6).into()),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(PbrBundle {
                mesh: asset_server.load("flota1.glb#Mesh0/Primitive0"),
                material: materials.add(Color::rgb(0.2, 0.8, 0.6).into()),
                transform: Transform::from_rotation(Quat::from_rotation_y(3.1415 / 2.)),
                ..Default::default()
            });
        })
        .with(Player { })
        .with(PlayerBoat {
            thrust: 0.,
            steer: 0.,
        })


        .spawn(camera)
        .with(CameraTracker {
            bobber: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            looking_up: LookingUp::None,
        })

        .spawn(UiCameraBundle::default())
        // texture
        .spawn(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            text: Text {
                value: "FPS:".to_string(),
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                style: TextStyle {
                    font_size: 60.0,
                    color: Color::WHITE,
                    ..Default::default()
                },
            },
            ..Default::default()
        })
        .with(FpsText);

}

fn spawn_sky(
    commands: &mut Commands,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut sky_materials: ResMut<Assets<SkyMaterial>>,
    mut render_graph: ResMut<RenderGraph>,
    asset_server: Res<AssetServer>,
) {
    let sky_pipeline_handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        vertex: shaders.add(Shader::from_glsl(ShaderStage::Vertex, SKY_VERTEX_SHADER)),
        fragment: Some(shaders.add(Shader::from_glsl(ShaderStage::Fragment, SKY_FRAGMENT_SHADER))),
    }));
    render_graph.add_system_node(
        "SkyMaterial",
        AssetRenderResourcesNode::<SkyMaterial>::new(true),
    );
    render_graph.add_node_edge(
        "SkyMaterial",
        base::node::MAIN_PASS,
    ).unwrap();

    let texture_handle: Handle<Texture> = asset_server.load("star.png");

    let render_pipelines = RenderPipelines::from_pipelines(vec![RenderPipeline::new(
        sky_pipeline_handle,
    )]);

    let sky_material = sky_materials.add(SkyMaterial {
        texture: texture_handle,
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(stripe::stars()),
        render_pipelines: render_pipelines.clone(),
        ..Default::default()
    })
    .with_children(|parent| {
        parent.spawn(PbrBundle {
            mesh: asset_server.load("palmera.glb#Mesh3/Primitive0"),
            render_pipelines: render_pipelines,
            transform: Transform::from_translation(Vec3::new(0., 0., 1000.))
                * Transform::from_scale(Vec3::new(10., 10., 10.)),
            ..Default::default()
        });
    })
    .with(sky_material)
    .with(SkyDome);
}

const WATER_TRANSLATE_STEP: f32 = 20.;
fn water_update_system(
    time: Res<Time>,
    // weather: Res<Weather>,
    mut water_mats: ResMut<Assets<WaterMaterial>>,
    water_material_query: Query<&Handle<WaterMaterial>>,
    mut water_transform_query: Query<(&mut water::Water, &mut Transform)>,
    camera_query: Query<(&Transform, &CameraTracker)>,
    boat_query: Query<(&Transform, &PlayerBoat)>,
    mut water_floored_query: Query<(&mut WaterFloor, &mut Transform)>,
) {
    if let Some(water_material) = water_material_query.iter().next()
            .and_then(|water_handle| water_mats.get_mut(water_handle))
    {
        water_material.time = time.seconds_since_startup as f32 * WAVE_SPEED;


        if let Some((transform, _)) = camera_query.iter().next() {
            water_material.camera = transform.translation;
        }

        // get boat transform
        if let Some((boat_transform, _)) = boat_query.iter().next() {
            if let Some((water, mut water_transform)) = water_transform_query.iter_mut().next() {

                // TODO: make weather_update_system
                // water::set_waves(&mut water, weather.wave_intensity);
                // water_material.wave1 = water.waves[0].to_vec4();
                // water_material.wave2 = water.waves[1].to_vec4();
                // water_material.wave3 = water.waves[2].to_vec4();

                water_transform.translation.x = boat_transform.translation.x - boat_transform.translation.x % WATER_TRANSLATE_STEP;
                water_transform.translation.z = boat_transform.translation.z - boat_transform.translation.z % WATER_TRANSLATE_STEP;
                let height = water.height_at_point(
                    Vec2::new(boat_transform.translation.x, boat_transform.translation.y),
                    time.seconds_since_startup as f32 * WAVE_SPEED
                );
                water_transform.translation.y = -height;

                if let Some((_, mut transform)) = water_floored_query.iter_mut().next() {
                    transform.translation.y = height;
                }
            }
        }
    }
}

const INPUT_ACCEL: f32 = 10.0;
const INPUT_DECAY: f32 = 10.0;
const BOAT_MAX_SPEED: f32 = 10.2;
fn keyboard_input_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut boat_query: Query<&mut PlayerBoat>,
    mut camera_query: Query<(&mut Transform, &mut CameraTracker)>,
) {
    for mut boat in &mut boat_query.iter_mut() {
        let mut print = false;

        if keyboard_input.pressed(KeyCode::W) {
            if boat.thrust < 1.0 {
                boat.thrust = (boat.thrust + INPUT_ACCEL * time.delta_seconds).min(BOAT_MAX_SPEED);
                print = true;
            }
        } else if boat.thrust > 0.0 {
            boat.thrust = (boat.thrust - INPUT_DECAY * time.delta_seconds).max(0.0);
            print = true;
        }

        if keyboard_input.pressed(KeyCode::A) {
            if boat.steer > -1.0 {
                boat.steer = (boat.steer - INPUT_ACCEL * time.delta_seconds).max(-1.0);
                print = true;
            }
        } else if boat.steer < 0.0 {
            boat.steer = (boat.steer + INPUT_DECAY * time.delta_seconds).min(0.0);
            print = true;
        }
        if keyboard_input.pressed(KeyCode::D) {
            if boat.steer < 1.0 {
                boat.steer = (boat.steer + INPUT_ACCEL * time.delta_seconds).min(1.0);
                print = true;
            }
        } else if boat.steer > 0.0 {
            boat.steer = (boat.steer - INPUT_DECAY * time.delta_seconds).max(0.0);
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

        if print {
            println!("boat {} / {}", boat.thrust, boat.steer);
        }
    }
}

fn boat_physics_system(
    time: Res<Time>,
    mut boat_query: Query<(&mut PlayerBoat, &mut Transform)>,
    mut skydome_query: Query<(&mut SkyDome, &mut Transform)>,
) {
    if let Some((boat, mut transform)) = boat_query.iter_mut().next() {
        if boat.steer != 0.0 || boat.thrust != 0.0 {
            transform.rotation = transform.rotation.slerp(
                transform.rotation.mul_quat(Quat::from_rotation_y(-boat.steer * 2.)),
                time.delta_seconds
            );

            let thrust_vector = Vec3::new(0., 0., boat.thrust * 0.6);
            let jump = transform.rotation.mul_vec3(thrust_vector);

            transform.translation += jump;

            if let Some((_sky, mut sky_transform)) = skydome_query.iter_mut().next() {
                let right_angle = Quat::from_rotation_y(std::f32::consts::PI / 2.);
                let rotation_axis = right_angle.mul_vec3(jump);
                let rotation = Quat::from_axis_angle(rotation_axis, -jump.length() * 0.01);
                sky_transform.rotation = rotation.mul_quat(sky_transform.rotation).normalize();
            }
        }

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
                boat_transform.rotation,
                time.delta_seconds * CAMERA_ROTATION_FACTOR
            );

            transform.translation = camera.bobber.translation
                + camera.bobber.rotation.mul_vec3(
                    Vec3::new(0.0, 5.0, -15.0)
                )
                + Vec3::new(0.0, -boat.thrust * 1.5, 0.0);

            let mut looking_at = camera.bobber.translation;
            match camera.looking_up {
                LookingUp::LookingUp(mut look) => {
                    look += time.delta_seconds * 0.5;
                    look = look.min(1.);
                    looking_at += Vec3::new(0., 100. * look, 0.);
                    camera.looking_up = LookingUp::LookingUp(look);
                }
                LookingUp::LookingDown(mut look) => {
                    look -= time.delta_seconds * 1.5;
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

            transform.rotation = transform.looking_at(
                looking_at,
                Vec3::unit_y()
            ).rotation;
        }
    }
}

fn wave_probe_system(
    time: Res<Time>,
    mut wave_probes_query: Query<(&WaveProbe, &mut Transform)>,
    water_query: Query<(&water::Water, &Transform)>,
) {
    if let Some((water, water_transform)) = water_query.iter().next() {
        for (_probe, mut transform) in wave_probes_query.iter_mut() {
            let wavedata = water.wave_data_at_point(
                Vec2::new(transform.translation.x * 1., transform.translation.z * 1.),
                time.seconds_since_startup as f32 * WAVE_SPEED
            );
            transform.translation.y = wavedata.position.y + water_transform.translation.y;

            let trans = Transform::identity().looking_at(wavedata.tangent, Vec3::unit_y());
            transform.rotation = trans.rotation;
            // transform.rotation = transform
                // .looking_at(transform.translation
                            // + wavedata.normal,
                            // Vec3::unit_y()).rotation;
            // transform.rotation = Quat::
        }
    }
}

fn text_update_system(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in query.iter_mut() {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                text.value = format!("FPS: {:.2}", average);
            }
        }
    }
}
