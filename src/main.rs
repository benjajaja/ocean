use bevy::{
    prelude::*,
    render::{
        mesh::{VertexAttributeValues, Indices},
        pipeline::{PipelineDescriptor, RenderPipeline, PrimitiveTopology},
        render_graph::{base, AssetRenderResourcesNode, RenderGraph},
        renderer::RenderResources,
        shader::{ShaderStage, ShaderStages},
    },
    type_registry::TypeUuid,
};
use rand::prelude::*;

/// This example illustrates how to add a custom attribute to a mesh and use it in a custom shader.
fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_asset::<WaterMaterial>()
        .add_startup_system(setup.system())
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .add_system(water_update.system())
        .run();
}

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "0320b9b8-b3a3-4baa-8bfa-c94008177b17"]
struct WaterMaterial {
    pub time: f32,
}

const VERTEX_SHADER: &str = include_str!("../media/shaders/water.vert");
const FRAGMENT_SHADER: &str = include_str!("../media/shaders/water.frag");

fn setup(
    commands: &mut Commands,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<WaterMaterial>>,
    mut render_graph: ResMut<RenderGraph>,
) {
    // Create a new shader pipeline
    let pipeline_handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        vertex: shaders.add(Shader::from_glsl(ShaderStage::Vertex, VERTEX_SHADER)),
        fragment: Some(shaders.add(Shader::from_glsl(ShaderStage::Fragment, FRAGMENT_SHADER))),
    }));

    // Add an AssetRenderResourcesNode to our Render Graph. This will bind WaterMaterial resources to our shader
    render_graph.add_system_node(
        "my_material_with_vertex_color_support",
        AssetRenderResourcesNode::<WaterMaterial>::new(true),
    );

    // Add a Render Graph edge connecting our new "my_material" node to the main pass node. This ensures "my_material" runs before the main pass
    render_graph
        .add_node_edge(
            "my_material_with_vertex_color_support",
            base::node::MAIN_PASS,
        )
        .unwrap();

    // Create a new material
    let material = materials.add(WaterMaterial {
        time: 0.,
    });

    // create a generic cube
    let cube_with_vertex_colors = Mesh::from(plane(5));

    // Setup our world
    commands.spawn(MeshBundle {
            mesh: meshes.add(cube_with_vertex_colors), // use our cube with vertex colors
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                pipeline_handle,
            )]),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..Default::default()
        })
        .with(material)
        // camera
        .spawn(Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 15.0, 15.0))
                .looking_at(Vec3::default(), Vec3::unit_y()),
            ..Default::default()
        });
}

fn plane(size: u32) -> Mesh {
    const CBRT3: f32 = 1.44224957031; // cubic root of 3
    fn normal() -> [f32; 3] {
        [0.0, 1.0, 0.1]
    }
    let mut rng = rand::thread_rng();

    let mut vertices: Vec<([f32; 3], [f32; 3], [f32; 2])> = vec![];
    let mut tri_indices: Vec<u32> = vec![];
    let mut vertex_colors: Vec<[f32; 3]> = vec![];
    for y in 0..size {
        let offset_y = y as f32 * CBRT3;
        let index_offset_y = y * size;

        // if y == 0 {
            // vertices.push(([0., 0., CBRT3], normal(), [1., 0.]));
            // vertices.push(([0.5, 0., 0.], normal(), [1., 0.]));
            // vertex_colors.push([rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>()]);
            // vertex_colors.push([rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>()]);
        // }

        if y == 0 {
            for x in 0..(size + 2) {
                let offset_x = x as f32;
                if x % 2 == 0 {
                    vertices.push(([offset_x * 0.5, 0., offset_y + CBRT3], normal(), [1., 0.]));
                    if x > 1 {
                        tri_indices.append(&mut vec![index_offset_y + x, index_offset_y + x - 1, index_offset_y + x - 2]);
                    }
                } else {
                    vertices.push(([offset_x * 0.5, 0., offset_y], normal(), [1., 0.]));
                    if x > 1 {
                        tri_indices.append(&mut vec![index_offset_y + x, index_offset_y + x - 2, index_offset_y + x - 1]);
                    }
                }
                vertex_colors.push([rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>()]);
            }
        } else if y % 2 != 0 {
            for x in 0..(size / 2) {
            }
        }
    }

    println!("{:#?}", tri_indices);
    let indices = Indices::U32(tri_indices);

    let mut positions: Vec<[f32; 3]> = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    for (position, normal, uv) in vertices.iter() {
        positions.push(*position);
        normals.push(*normal);
        uvs.push(*uv);
    }

    println!("{:#?}", positions);

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.set_indices(Some(indices));


    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, VertexAttributeValues::from(positions));
    mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, VertexAttributeValues::from(normals));
    mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, VertexAttributeValues::from(uvs));

    mesh.set_attribute("Vertex_Color", VertexAttributeValues::from(vertex_colors));

    mesh
}

fn water_update(
    time: Res<Time>,
    mut water_mats: ResMut<Assets<WaterMaterial>>,
    water_query: Query<&Handle<WaterMaterial>>,
) {
    for water in &mut water_query.iter() {
        if let Some(water) = water_mats.get_mut(water) {
            water.time = time.seconds_since_startup as f32;
        }
    }
}

