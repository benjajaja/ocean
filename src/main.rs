use bevy::{
    prelude::*,
    render::{
        pipeline::{PipelineDescriptor, RenderPipeline},
        render_graph::{base, AssetRenderResourcesNode, RenderGraph},
        renderer::RenderResources,
        shader::{ShaderStage, ShaderStages},
    },
    type_registry::TypeUuid,
};
mod water;
// use rand::prelude::*;

/// This example illustrates how to add a custom attribute to a mesh and use it in a custom shader.
fn main() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_asset::<WaterMaterial>()
        .add_resource(Weather {
            wave_intensity: 50.,
        })
        .add_startup_system(setup.system())
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .add_system(water_update_system.system())
        .add_system(keyboard_input_system.system())
        .add_system(boat_physics_system.system())
        .add_system(camera_system.system())
        .add_system(wave_probe_system.system())
        .run();
}

struct Camera {
    bobber: Transform,
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

struct Weather {
    wave_intensity: f32,
}

const VERTEX_SHADER: &str = include_str!("../assets/shaders/water.vert");
const FRAGMENT_SHADER: &str = include_str!("../assets/shaders/water.frag");

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
    // Create a new shader pipeline
    let pipeline_handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        vertex: shaders.add(Shader::from_glsl(ShaderStage::Vertex, VERTEX_SHADER)),
        fragment: Some(shaders.add(Shader::from_glsl(ShaderStage::Fragment, FRAGMENT_SHADER))),
    }));

    // Add an AssetRenderResourcesNode to our Render Graph. This will bind WaterMaterial resources to our shader
    render_graph.add_system_node(
        "WaterMaterial",
        AssetRenderResourcesNode::<WaterMaterial>::new(true),
    );
    // Add a Render Graph edge connecting our new "my_material" node to the main pass node. This ensures "my_material" runs before the main pass
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

    // Setup our world
    commands
        // .spawn_scene(asset_server.load("palmera.glb"))
        .spawn(PbrBundle {
            // mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            mesh: asset_server.load("flota1.glb#Mesh0/Primitive0"),
            material: materials.add(Color::rgb(0.0, 0.9, 0.6).into()),
            transform: Transform::from_translation(Vec3::new(5.0, 0.0, 0.0)),
            ..Default::default()
        })
        .with(WaveProbe)
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
            material: materials.add(Color::rgb(0.8, 0.5, 0.0).into()),
            transform: Transform::from_translation(Vec3::new(-5.0, 0.0, 5.0)),
            ..Default::default()
        })
        .with(WaveProbe)

        .spawn(palmtree)
        .with(WaterFloor)

        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 50.0, 4.0)),
            ..Default::default()
        })

        .spawn(MeshBundle {
            mesh: asset_server.load("plano.glb#Mesh0/Primitive0"),
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                pipeline_handle,
            )]),
            transform: Transform::from_scale(Vec3::new(100.0, 100.0, 100.0)),
            ..Default::default()
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
            material: materials.add(Color::rgb(0.2, 0.8, 0.6).into()),
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


        .spawn(Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 6.0, -15.0))
                    .looking_at(Vec3::new(0.0, 5.0, 0.0), Vec3::unit_y()),
            ..Default::default()
        })
        .with(Camera {
            bobber: Transform::from_translation(Vec3::new(0.0, 5.0, 0.0)),
        });
}

fn water_update_system(
    time: Res<Time>,
    // weather: Res<Weather>,
    mut water_mats: ResMut<Assets<WaterMaterial>>,
    water_material_query: Query<&Handle<WaterMaterial>>,
    mut water_transform_query: Query<(&mut water::Water, &mut Transform)>,
    camera_query: Query<(&Transform, &Camera)>,
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

                water_transform.translation = boat_transform.translation;
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

const INPUT_ACCEL: f32 = 2.0;
const INPUT_DECAY: f32 = 10.0;
fn keyboard_input_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut boat_query: Query<&mut PlayerBoat>,
) {
    // if let Some(mut boat) = &mut boat_query.iter_mut().next() {
    // }
    for mut boat in &mut boat_query.iter_mut() {
        let mut print = false;

        if keyboard_input.pressed(KeyCode::W) {
            if boat.thrust < 1.0 {
                boat.thrust = (boat.thrust + INPUT_ACCEL * time.delta_seconds).min(1.0);
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

        if print {
            println!("boat {} / {}", boat.thrust, boat.steer);
        }
    }
}

fn boat_physics_system(
    time: Res<Time>,
    mut boat_query: Query<(&mut PlayerBoat, &mut Transform)>,
) {
    if let Some((boat, mut transform)) = boat_query.iter_mut().next() {
        if boat.steer != 0.0 || boat.thrust != 0.0 {
            transform.rotation = transform.rotation.slerp(
                transform.rotation.mul_quat(Quat::from_rotation_y(-boat.steer * 2.)),
                time.delta_seconds
            );

            let thrust_vector = Vec3::new(0., 0., boat.thrust / 4.);
            let jump = transform.rotation.mul_vec3(thrust_vector);

            transform.translation += jump;
        }

    }
}

const CAMERA_ROTATION_FACTOR: f32 = 10.0;
fn camera_system(
    time: Res<Time>,
    boat_query: Query<(&PlayerBoat, &Transform)>,
    mut camera_query: Query<(&mut Transform, &mut Camera)>,
) {
    if let Some((mut transform, mut camera)) = camera_query.iter_mut().next() {
        if let Some((_boat, boat_transform)) = boat_query.iter().next() {
            camera.bobber.translation = boat_transform.translation.clone();
            camera.bobber.rotation = camera.bobber.rotation.slerp(
                boat_transform.rotation,
                time.delta_seconds * CAMERA_ROTATION_FACTOR
            );

            transform.translation = camera.bobber.translation +
                camera.bobber.rotation.mul_vec3(
                    Vec3::new(0.0, 5.0, -15.0)
                );

            transform.rotation = transform.looking_at(
                camera.bobber.translation,
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

