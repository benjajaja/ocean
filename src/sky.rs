use crate::boat::PlayerBoat;
use crate::stripe;
use crate::DayTime;
use crate::InGameState;
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

pub struct SkyDomeLayer {
    pub daytime: DayTime,
}

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

#[derive(Debug, Clone)]
pub struct SkyDomeIsland {
    id: super::Island,
    rotation: Quat,
}

impl SkyDomeIsland {
    fn new(id: super::Island, rotation: Quat) -> Self {
        SkyDomeIsland { id, rotation }
    }
}

// sky 3d
#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "0320b9b8-dead-beef-8bfa-c94008177b17"]
pub struct SkySpriteMaterial {
    texture: Handle<Texture>,
}
const SKY_VERTEX_SHADER: &str = include_str!("../assets/shaders/sky.vert");
const SKY_FRAGMENT_SHADER: &str = include_str!("../assets/shaders/sky.frag");

pub fn add_systems(app: &mut bevy::prelude::AppBuilder) -> &mut bevy::prelude::AppBuilder {
    app.add_resource(ClearColor(Color::rgb(0., 0., 0.)));
    app.add_resource(SkyDome::new());

    app.add_asset::<SkySpriteMaterial>();

    app.add_startup_system(spawn_sky.system());
    app.add_system(skydome_system.system());
    app
}

pub fn spawn_sky(
    commands: &mut Commands,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut sky_materials: ResMut<Assets<SkySpriteMaterial>>,
    mut render_graph: ResMut<RenderGraph>,
    asset_server: Res<AssetServer>,
) {
    let mut descriptor = PipelineDescriptor::default_config(ShaderStages {
        vertex: shaders.add(Shader::from_glsl(ShaderStage::Vertex, SKY_VERTEX_SHADER)),
        fragment: Some(shaders.add(Shader::from_glsl(
            ShaderStage::Fragment,
            SKY_FRAGMENT_SHADER,
        ))),
    });
    descriptor.depth_stencil_state =
        descriptor
            .depth_stencil_state
            .map(|mut depth_stencil_state| {
                depth_stencil_state.depth_compare =
                    bevy::render::pipeline::CompareFunction::LessEqual;
                depth_stencil_state.depth_write_enabled = false;
                depth_stencil_state
            });

    let sky_pipeline_handle = pipelines.add(descriptor);
    render_graph.add_system_node(
        "SkySpriteMaterial",
        AssetRenderResourcesNode::<SkySpriteMaterial>::new(true),
    );
    render_graph
        .add_node_edge("SkySpriteMaterial", base::node::MAIN_PASS)
        .unwrap();

    let texture_handle: Handle<Texture> = asset_server.load("star.png");
    let texture_handle_islands: Handle<Texture> = asset_server.load("palmtree_sky.png");

    let render_pipelines =
        RenderPipelines::from_pipelines(vec![RenderPipeline::new(sky_pipeline_handle)]);

    let sky_material = sky_materials.add(SkySpriteMaterial {
        texture: texture_handle,
    });

    let sky_material_islands = sky_materials.add(SkySpriteMaterial {
        texture: texture_handle_islands,
    });

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(stripe::bg_stars()),
            render_pipelines: render_pipelines.clone(),
            transform: Transform::from_scale(Vec3::splat(100.0)),
            ..Default::default()
        })
        .with(sky_material)
        .with(SkyDomeLayer {
            daytime: DayTime::Night,
        });

    let islands = vec![
        // SkyDomeIsland::new(super::Island::IslandA, Quat::from_rotation_x(FRAC_PI_2)),
        // SkyDomeIsland::new(
        // super::Island::IslandA,
        // Quat::from_rotation_x(FRAC_PI_2 * 0.4),
        // ),
        SkyDomeIsland::new(
            super::Island::Home,
            Quat::from_rotation_x(FRAC_PI_2 * 0.5) * Quat::from_rotation_y(FRAC_PI_2 * 0.2),
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
            transform: Transform::from_scale(Vec3::splat(100.0)),
            ..Default::default()
        })
        .with(sky_material_islands)
        .with(SkyDomeLayer {
            daytime: DayTime::Night,
        });

    for island in islands {
        commands.spawn((island,));
    }

    return;
    // let (sky_day_material, day_render_pipeline) = day_shader();
    let mut material = StandardMaterial::from(Color::rgb(0.9, 0.9, 0.6));
    material.shaded = false;
    commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: -100.0 })),
            // render_pipelines: day_render_pipeline.clone(),
            // material: materials.add(material),
            visible: Visible {
                is_visible: false,
                is_transparent: false,
            },
            ..Default::default()
        // })
        // .with(sky_day_material)
        // .with(SkyDomeLayer {
            // daytime: DayTime::Day,
        });
}

fn day_shader() -> () {}

pub fn skydome_system(
    state: Res<InGameState>,
    skydome: Res<SkyDome>,
    mut skydome_query: Query<(&SkyDomeLayer, &mut Transform, &mut Visible)>,
    mut lines: ResMut<DebugLines>,
    island_query: Query<&SkyDomeIsland>,
    boat_query: Query<(&PlayerBoat, &Transform)>,
    camera_query: Query<(&Transform, &super::camera::CameraTracker)>,
    mut ev_approach: ResMut<Events<super::NavigationEvent>>,
) {
    if let Some((camera_transform, _)) = camera_query.iter().next() {
        for (_, mut sky_transform, _) in skydome_query.iter_mut() {
            sky_transform.rotation = skydome.rotation;
            sky_transform.translation = camera_transform.translation;
        }
    }

    match state.time {
        DayTime::Night => {
            let boat_transform = boat_query.iter().next().map(|t| t.1);

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
            for (i, island) in island_query.iter().enumerate() {
                let island_vec = island.rotation * Vec3::unit_z();
                let angle = island_vec.dot(sky_vec);
                println!("angle {}", angle);

                if angle > 0.9 {
                    ev_approach.send(super::NavigationEvent::Approach(island.id));
                    for (layer, _, mut visible) in skydome_query.iter_mut() {
                        visible.is_visible = layer.daytime == DayTime::Day;
                    }
                }

                // debug lines
                if let Some(boat_transform) = boat_transform {
                    lines.line_colored(
                        1 + (i as u32),
                        boat_transform.translation,
                        boat_transform.translation
                            + (skydome.rotation * island.rotation * Vec3::new(0.0, 1000.0, 0.0)),
                        0.1,
                        Color::RED,
                    );
                }
            }
        }
        DayTime::Day => {}
    }
}
