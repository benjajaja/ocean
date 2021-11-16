use crate::atmosphere;
use crate::stripe;
use crate::DayTime;
use crate::InGameState;
use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::{
        pipeline::{PipelineDescriptor, RenderPipeline},
        render_graph::{base, AssetRenderResourcesNode, RenderGraph},
        shader::ShaderStages,
    },
};
use std::f32::consts::FRAC_PI_2;

pub struct SkyDomeLayer;
pub struct SkyDomeLayerBg;

pub struct SkyDome {
    pub rotation: Quat,
    pub locked_island: bool,
}

impl SkyDome {
    pub fn new() -> Self {
        SkyDome {
            rotation: Quat::IDENTITY,
            locked_island: false,
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

pub const FORWARD_PIPELINE_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(PipelineDescriptor::TYPE_UUID, 13148362314012771389);

pub fn add_systems(app: &mut bevy::prelude::AppBuilder) -> &mut bevy::prelude::AppBuilder {
    app.insert_resource(ClearColor(Color::rgb(0., 0., 0.)));
    app.insert_resource(SkyDome::new());

    app.add_startup_system(spawn_sky.system());

    app.insert_resource(atmosphere::AtmosphereMat {
        sun_intensity: 1.0,
        ..Default::default()
    });
    app.add_asset::<atmosphere::AtmosphereMat>();
    app.add_startup_system(atmosphere_add_sky_sphere.system());

    app.add_system(skydome_system.system());

    app
}

pub fn spawn_sky(
    mut commands: Commands,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut render_graph: ResMut<RenderGraph>,
    asset_server: Res<AssetServer>,
) {
    let render_pipelines = sky_pipelines(&mut pipelines, &mut render_graph, "StandardMaterial");

    let texture_handle: Handle<Texture> = asset_server.load("star.png");
    let texture_handle_islands: Handle<Texture> = asset_server.load("palmtree_sky.png");

    let sky_material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        base_color_texture: Some(texture_handle),
        unlit: true,
        ..Default::default()
    });

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(stripe::bg_stars()),
            render_pipelines: render_pipelines.clone(),
            transform: Transform::from_scale(Vec3::splat(100.0)),
            ..Default::default()
        })
        .insert(sky_material)
        .insert(SkyDomeLayer {})
        .insert(SkyDomeLayerBg);

    let islands = vec![
        // SkyDomeIsland::new(
        // super::Island::IslandA,
        // Quat::from_rotation_z(FRAC_PI_2 * 0.9),
        // ),
        // SkyDomeIsland::new(
        // super::Island::IslandA,
        // Quat::from_rotation_x(FRAC_PI_2 * 0.9),
        // ),
        SkyDomeIsland::new(
            super::Island::Home,
            Quat::from_rotation_x(FRAC_PI_2 * 0.1) * Quat::from_rotation_y(FRAC_PI_2 * 0.2),
        ),
        SkyDomeIsland::new(
            super::Island::IslandA,
            Quat::from_rotation_x(FRAC_PI_2 * -0.15) * Quat::from_rotation_z(FRAC_PI_2 * 0.12),
        ),
    ];

    let sky_material_islands = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle_islands),
        unlit: true,
        ..Default::default()
    });

    let island_stars: Vec<stripe::StarDef> = islands
        .iter()
        .map(|island| stripe::StarDef {
            quat: island.rotation,
            size: 0.025,
        })
        .collect();
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(stripe::island_stars(island_stars)),
            render_pipelines: render_pipelines.clone(),
            transform: Transform::from_scale(Vec3::splat(100.0)),
            ..Default::default()
        })
        .insert(sky_material_islands)
        .insert(SkyDomeLayer {})
        .insert(SkyDomeLayerBg {});

    for island in islands {
        commands.spawn_bundle((island,));
    }
}

fn sky_pipelines(
    pipelines: &mut ResMut<Assets<PipelineDescriptor>>,
    render_graph: &mut ResMut<RenderGraph>,
    material_name: &'static str,
) -> RenderPipelines {
    let forward_pipeline_handle = pipelines.get(FORWARD_PIPELINE_HANDLE).unwrap();

    let mut descriptor = PipelineDescriptor::default_config(ShaderStages {
        vertex: forward_pipeline_handle.shader_stages.vertex.clone(),
        fragment: forward_pipeline_handle.shader_stages.fragment.clone(),
    });

    descriptor.depth_stencil = descriptor.depth_stencil.map(|mut depth_stencil| {
        depth_stencil.depth_compare = bevy::render::pipeline::CompareFunction::LessEqual;
        depth_stencil.depth_write_enabled = false;
        depth_stencil
    });

    let sky_pipeline_handle = pipelines.add(descriptor);
    render_graph.add_system_node(
        material_name,
        AssetRenderResourcesNode::<StandardMaterial>::new(true),
    );
    render_graph
        .add_node_edge(material_name, base::node::MAIN_PASS)
        .unwrap();

    RenderPipelines::from_pipelines(vec![RenderPipeline::new(sky_pipeline_handle)])
}

pub fn skydome_system(
    mut events: EventReader<super::boat::MoveEvent>,
    state: Res<InGameState>,
    mut skydome: ResMut<SkyDome>,
    mut skydome_query: Query<
        (&SkyDomeLayer, &mut Transform, &mut Visible),
        Without<super::WorldIsland>,
    >,
    island_query: Query<&SkyDomeIsland>,
    mut ev_approach: EventWriter<super::NavigationEvent>,
    worldisland_query: Query<(&super::WorldIsland, &Transform), Without<SkyDomeLayer>>,
    mut clear_color: ResMut<ClearColor>,
    mut weather: ResMut<super::water::Weather>,
) {
    for ev in events.iter() {
        let translation = ev.translation;

        match state.time {
            DayTime::Night => {
                skydome.rotation = (jump_skydome(ev.jump) * skydome.rotation).normalize();

                let sky_vec = Vec3::Y;
                for island in island_query.iter() {
                    let island_vec = (skydome.rotation * island.rotation) * Vec3::Y;
                    let angle = island_vec.dot(sky_vec);

                    if angle > 0.99 {
                        let mut vec: Vec3 = translation + (island_vec * 5000.);
                        vec.y = 0.;

                        ev_approach.send(super::NavigationEvent::Enter(
                            island.id,
                            island.rotation,
                            vec,
                        ));
                    }
                }
            }
            DayTime::Day => {
                if let Some((island, island_transform)) = worldisland_query.iter().next() {
                    let distance = translation.distance(island_transform.translation);
                    if distance > 700. {
                        ev_approach.send(super::NavigationEvent::Leave);
                    } else {
                        let distance_frac = 1. - (distance / 700.);
                        ev_approach.send(super::NavigationEvent::Approach(distance_frac));

                        let value = ((distance_frac - 0.1).max(0.) * 10.).min(0.75);
                        // println!("approach value {} (dist. {})", value, distance_frac);
                        clear_color.0 = Color::rgb(value - 0.3, value - 0.2, value);
                        weather.wave_intensity = 1. - value;

                        const LOCK_DISTANCE: f32 = 0.2;
                        if distance_frac > LOCK_DISTANCE && !skydome.locked_island {
                            let axis_angle =
                                (skydome.rotation * island.sky_rotation).to_axis_angle();
                            skydome.rotation = Quat::from_axis_angle(axis_angle.0, -axis_angle.1)
                                * skydome.rotation;
                            for (_, _, mut visible) in skydome_query.iter_mut() {
                                visible.is_visible = false;
                            }
                            skydome.locked_island = true;
                        } else if distance_frac < LOCK_DISTANCE {
                            if skydome.locked_island {
                                let mut from = translation;
                                from.y = 0.;
                                let mut to = island_transform.translation;
                                to.y = 0.;
                                let angle = FRAC_PI_2 * 0.05;
                                let axis = (from - to).normalize();
                                let rotation = (Quat::from_axis_angle(
                                    Quat::from_rotation_y(FRAC_PI_2) * axis,
                                    -angle,
                                ))
                                .normalize();
                                skydome.rotation = rotation * skydome.rotation;
                                skydome.locked_island = false;
                                for (_, _, mut visible) in skydome_query.iter_mut() {
                                    visible.is_visible = true;
                                }
                            } else {
                                skydome.rotation =
                                    (jump_skydome(ev.jump) * skydome.rotation).normalize();
                            }
                        }
                    }
                }
            }
        }

        for (_, mut sky_transform, _) in skydome_query.iter_mut() {
            sky_transform.rotation = skydome.rotation;
        }

        // lines.line_colored(
        // translation,
        // translation + (Vec3::new(0.0, 100000.0, 0.0)),
        // 0.1,
        // Color::WHITE,
        // );
        // lines.line_colored(
        // translation,
        // translation + (skydome.rotation * Vec3::new(0.0, 100000.0, 0.0)),
        // 0.1,
        // Color::BLUE,
        // );
        // for island in island_query.iter() {
        // lines.line_colored(
        // translation,
        // translation + ((skydome.rotation * island.rotation) * Vec3::new(0.0, 1000.0, 0.0)),
        // 0.1,
        // Color::RED,
        // );
        // }
    }
}

fn jump_skydome(jump: Vec3) -> Quat {
    let right_angle = Quat::from_rotation_y(FRAC_PI_2);
    let rotation_axis = right_angle * jump;
    let rotation = Quat::from_axis_angle(rotation_axis, -jump.length() / 1000.);
    rotation
}

fn atmosphere_add_sky_sphere(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut sky_materials: ResMut<Assets<atmosphere::AtmosphereMat>>,
    pipelines: ResMut<Assets<PipelineDescriptor>>,
    shaders: ResMut<Assets<Shader>>,
    render_graph: ResMut<RenderGraph>,
) {
    let render_pipelines = atmosphere::AtmosphereMat::pipeline(pipelines, shaders, render_graph);

    let sky_material = atmosphere::AtmosphereMat::default();

    let sky_material = sky_materials.add(sky_material);

    commands
        .spawn_bundle(MeshBundle {
            mesh: meshes.add(Mesh::from(shape::Cube {
                size: -10.0,
                // radius: -10.0,
                // subdivisions: 2,
            })),
            render_pipelines,
            ..Default::default()
        })
        .insert(sky_material)
        .insert(SkyDomeLayerBg {});
}
