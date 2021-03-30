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
use bevy_prototype_debug_lines::*;
use std::f32::consts::FRAC_PI_2;

pub struct SkyDomeLayer {
    pub daytime: DayTime,
}

pub struct SkyDome {
    pub rotation: Quat,
    pub locked_island: bool,
}

impl SkyDome {
    pub fn new() -> Self {
        SkyDome {
            rotation: Quat::identity(),
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
    app.add_resource(ClearColor(Color::rgb(0., 0., 0.)));
    app.add_resource(SkyDome::new());

    app.add_startup_system(spawn_sky.system());
    app.add_system(skydome_system.system());
    app
}

pub fn spawn_sky(
    commands: &mut Commands,
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
        albedo: Color::WHITE,
        albedo_texture: Some(texture_handle),
        shaded: false,
    });

    let sky_material_islands = materials.add(StandardMaterial {
        albedo: Color::WHITE,
        albedo_texture: Some(texture_handle_islands),
        shaded: false,
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

    // commands
    // .spawn(PbrBundle {
    // mesh: meshes.add(Mesh::from(shape::Icosphere {
    // radius: -110.,
    // subdivisions: 4,
    // })),
    // material: materials.add(Color::rgb(0.5, 0.9, 0.6).into()),
    // ..Default::default()
    // })
    // .with(SkyDomeLayer {
    // daytime: DayTime::Night,
    // });
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
        material_name,
        AssetRenderResourcesNode::<StandardMaterial>::new(true),
    );
    render_graph
        .add_node_edge(material_name, base::node::MAIN_PASS)
        .unwrap();

    RenderPipelines::from_pipelines(vec![RenderPipeline::new(sky_pipeline_handle)])
}

pub fn skydome_system(
    events: Res<Events<super::boat::MoveEvent>>,
    mut event_reader: Local<EventReader<super::boat::MoveEvent>>,
    state: Res<InGameState>,
    mut skydome: ResMut<SkyDome>,
    mut skydome_query: Query<(&SkyDomeLayer, &mut Transform, &mut Visible)>,
    island_query: Query<&SkyDomeIsland>,
    mut ev_approach: ResMut<Events<super::NavigationEvent>>,
    worldisland_query: Query<(&super::WorldIsland, &Transform)>,
    mut clear_color: ResMut<ClearColor>,
    mut weather: ResMut<super::water::Weather>,
    mut lines: ResMut<DebugLines>,
) {
    for ev in event_reader.iter(&events) {
        let translation = ev.translation;

        match state.time {
            DayTime::Night => {
                skydome.rotation = (jump_skydome(ev.jump) * skydome.rotation).normalize();

                let sky_vec = Vec3::unit_y();
                for island in island_query.iter() {
                    let island_vec = (skydome.rotation * island.rotation) * Vec3::unit_y();
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

                        let value = ((distance_frac - 0.1).max(0.) * 10.).min(1.0);
                        println!("approach value {} (dist. {})", value, distance_frac);
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
