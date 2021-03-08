use crate::boat::PlayerBoat;
use crate::stripe;
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
use bevy_prototype_debug_lines::*;
use std::f32::consts::FRAC_PI_2;

pub struct SkyDomeLayer;

pub struct SkyDome {
    pub rotation: Quat,
}

impl SkyDome {
    pub fn new() -> Self {
        SkyDome {
            rotation: Quat::identity(),
        }
    }
}

pub struct SkyDomeIsland {
    rotation: Quat,
    world_island: Option<u32>,
}
impl SkyDomeIsland {
    fn new(rotation: Quat) -> Self {
        SkyDomeIsland {
            rotation,
            world_island: None,
        }
    }
}

// sky 3d
#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "0320b9b8-dead-beef-8bfa-c94008177b17"]
pub struct SkyMaterial {
    texture: Handle<Texture>,
}
const SKY_VERTEX_SHADER: &str = include_str!("../assets/shaders/sky.vert");
const SKY_FRAGMENT_SHADER: &str = include_str!("../assets/shaders/sky.frag");

pub fn spawn_sky(
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

    let islands = vec![
        // SkyDomeIsland {
        // rotation: Quat::from_rotation_x(FRAC_PI_2 * 1.5),
        // },
        // SkyDomeIsland {
        // rotation: Quat::from_rotation_x(FRAC_PI_2),
        // },
        SkyDomeIsland::new(Quat::from_rotation_x(FRAC_PI_2 * 0.5)),
        SkyDomeIsland::new(
            Quat::from_rotation_x(FRAC_PI_2 * 0.1) * Quat::from_rotation_y(FRAC_PI_2 * 0.2),
        ),
    ];

    let island_stars: Vec<stripe::StarDef> = islands
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

    for island in islands {
        commands.spawn((island,));
    }
}

pub fn skydome_system(
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    skydome: Res<SkyDome>,
    mut skydome_query: Query<(&SkyDomeLayer, &mut Transform)>,
    mut lines: ResMut<DebugLines>,
    mut island_query: Query<&mut SkyDomeIsland>,
    boat_query: Query<(&PlayerBoat, &Transform)>,
    commands: &mut Commands,
) {
    let boat_transform = boat_query.iter().next().map(|t| t.1);
    for (_, mut sky_transform) in skydome_query.iter_mut() {
        sky_transform.rotation = skydome.rotation;
    }

    if let Some(boat_transform) = boat_transform {
        lines.line_colored(
            0,
            boat_transform.translation,
            boat_transform.translation + (Vec3::new(0.0, 100.0, 0.0)),
            0.01,
            Color::GREEN,
        );
    }
    let sky_vec = skydome.rotation.conjugate() * Vec3::unit_z();
    // let sky_inverse = skydome.rotation.conjugate();
    for (i, mut island) in island_query.iter_mut().enumerate() {
        let island_vec = island.rotation * Vec3::unit_z();
        let angle = island_vec.dot(sky_vec);
        if angle > 0.9 && island.world_island.is_none() {
            println!("({}) angle: {:?}", i, angle);
            let mut palmtree_transform = Transform::from_translation(Vec3::new(-5.0, -3.0, 5.0));

            palmtree_transform.scale = Vec3::new(4., 4., 4.);
            let palmtree = PbrBundle {
                mesh: asset_server.load("palmera.glb#Mesh3/Primitive0"),
                material: materials.add(Color::rgb(0.9, 0.9, 0.6).into()),
                transform: palmtree_transform,
                ..Default::default()
            };
            if let Some(entity) = commands.spawn(palmtree).current_entity() {
                island.world_island = Some(entity.id());
            }
        } else if angle < 0.8 {
            if let Some(eid) = island.world_island {
                // commands.despawn_recursive(entity)
            }
        }

        // debug lines
        if let Some(boat_transform) = boat_transform {
            lines.line_colored(
                1 + (i as u32),
                boat_transform.translation,
                boat_transform.translation
                    + (skydome.rotation * island.rotation * Vec3::new(0.0, 10000.0, 0.0)),
                0.1,
                Color::RED,
            );
        }
    }
}
