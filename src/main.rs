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
// use rand::prelude::*;

/// This example illustrates how to add a custom attribute to a mesh and use it in a custom shader.
fn main() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_asset::<WaterMaterial>()
        .add_startup_system(setup.system())
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .add_system(water_update.system())
        .add_system(keyboard_input_system.system())
        .add_system(boat_physics.system())
        .run();
}

struct Camera {
    bobber: CameraBobber,
}
struct CameraBobber {
    transform: Transform,
}
struct Player;
struct PlayerBoat {
    thrust: f32,
    steer: f32,
}
#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "0320b9b8-b3a3-4baa-8bfa-c94008177b17"]
struct WaterMaterial {
    pub time: f32,
    pub camera: Vec3,
    pub color: Vec4,
}

const VERTEX_SHADER: &str = include_str!("../assets/shaders/water.vert");
const FRAGMENT_SHADER: &str = include_str!("../assets/shaders/water.frag");

fn setup(
    commands: &mut Commands,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut water_materials: ResMut<Assets<WaterMaterial>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut render_graph: ResMut<RenderGraph>,
    asset_server: Res<AssetServer>,
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

    // Setup our world
    commands
        // .spawn_scene(asset_server.load("countach.gltf"))
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_translation(Vec3::new(5.0, 0.0, 0.0)),
            ..Default::default()
        })
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 50.0, 4.0)),
            ..Default::default()
        })

        .spawn(MeshBundle {
            mesh: asset_server.load("water.gltf#Mesh0/Primitive0"),
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                pipeline_handle,
            )]),
            transform: Transform::from_scale(Vec3::new(100.0, 100.0, 100.0)),
            ..Default::default()
        })
        .with(water_materials.add(WaterMaterial {
            time: 0.,
            color: Vec4::new(0.1, 0.8, 0.5, 1.0),
            camera: Vec3::new(0., 0., 0.),
        }))

        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.2, 0.8, 0.6).into()),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..Default::default()
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
            bobber: CameraBobber {
                transform: Transform::from_translation(Vec3::new(0.0, 5.0, 0.0)),
            }
        });
}

fn water_update(
    time: Res<Time>,
    mut water_mats: ResMut<Assets<WaterMaterial>>,
    water_query: Query<&Handle<WaterMaterial>>,
    camera_query: Query<(&Transform, &Camera)>,
) {
    for water in &mut water_query.iter() {
        if let Some(water) = water_mats.get_mut(water) {
            water.time = time.seconds_since_startup as f32;
            for (transform, _) in camera_query.iter() {
                water.camera = transform.translation;
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

fn boat_physics(
    time: Res<Time>,
    mut boat_query: Query<(&mut PlayerBoat, &mut Transform)>,
) {
    for (boat, mut transform) in &mut boat_query.iter_mut() {
        transform.rotation = transform.rotation.slerp(
            transform.rotation.mul_quat(Quat::from_rotation_y(-boat.steer * 2.)),
            time.delta_seconds
        );

        let thrust_vector = Vec3::new(0., 0., boat.thrust / 2.);
        let jump = transform.rotation.mul_vec3(thrust_vector);

        transform.translation += jump;
    }
}
