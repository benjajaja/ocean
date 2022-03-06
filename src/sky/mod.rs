use crate::DayTime;
use crate::InGameState;
use bevy::prelude::*;
use std::f32::consts::FRAC_PI_2;

// use self::sphere_material::SkySphereMaterial;
use self::star_material::SkyStarMaterial;
mod mesh;
// mod sphere_material;
mod star_material;

#[derive(Component)]
pub struct SkyDomeLayer;
#[derive(Component)]
pub struct SkyDomeLayerBg;

#[derive(Component)]
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

#[derive(Component, Debug, Clone)]
pub struct SkyDomeIsland {
    id: super::Island,
    rotation: Quat,
}

impl SkyDomeIsland {
    fn new(id: super::Island, rotation: Quat) -> Self {
        SkyDomeIsland { id, rotation }
    }
}

pub fn add_systems(app: &mut bevy::prelude::App) -> &mut bevy::prelude::App {
    app.insert_resource(ClearColor(Color::rgb(0., 0., 0.)));
    app.insert_resource(SkyDome::new());

    app.add_plugin(MaterialPlugin::<SkyStarMaterial>::default());
    // app.add_plugin(MaterialPlugin::<SkySphereMaterial>::default());
    app.add_startup_system(spawn_sky.system());

    app.add_system(skydome_system.system());

    app
}

pub fn spawn_sky(
    mut commands: Commands,
    // mut pipelines: ResMut<Assets<RenderPipelineDescriptor>>,
    // mut pipelines: ResMut<SpecializedPipelines<IsRedPipeline>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut sky_star_materials: ResMut<Assets<SkyStarMaterial>>,
    // mut sky_sphere_materials: ResMut<Assets<SkySphereMaterial>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    // mut render_graph: ResMut<RenderGraph>,
    asset_server: Res<AssetServer>,
) {
    // let texture_handle: Handle<Image> = asset_server.load("star.png");
    let sky_sphere_material_handle = sky_star_materials.add(SkyStarMaterial {
        color: Color::MIDNIGHT_BLUE,
    });
    commands
        .spawn()
        .insert_bundle(MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: -1.0,
                subdivisions: 4,
            })),
            material: sky_sphere_material_handle,
            ..Default::default()
        })
        .insert(Name::new("SkySphere"))
        .insert(SkyDomeLayer)
        .insert(SkyDomeLayerBg);

    let sky_material_handle = sky_star_materials.add(SkyStarMaterial {
        color: Color::WHITE,
    });

    commands
        .spawn()
        .insert_bundle(MaterialMeshBundle {
            mesh: meshes.add(mesh::bg_stars()),
            material: sky_material_handle,
            ..Default::default()
        })
        // .insert(sky_material_handle)
        .insert(Name::new("SkyStars"))
        .insert(SkyDomeLayer)
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

    let texture_handle_islands: Handle<Image> = asset_server.load("palmtree_sky.png");
    let sky_material_islands = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        base_color_texture: Some(texture_handle_islands),
        unlit: true,
        alpha_mode: AlphaMode::Blend,
        ..Default::default()
    });

    let island_stars: Vec<mesh::StarDef> = islands
        .iter()
        .map(|island| mesh::StarDef {
            quat: island.rotation,
            size: 0.025 * mesh::STAR_DISTANCE,
        })
        .collect();
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(mesh::island_stars(island_stars)),
            material: sky_material_islands,
            ..Default::default()
        })
        .insert(Name::new("SkyIslands"))
        .insert(SkyDomeLayer)
        .insert(SkyDomeLayerBg);

    for island in islands {
        let name = format!("SkyDomeIsland-{:?}", island.id);
        commands.spawn_bundle((island,)).insert(Name::new(name));
    }
}

pub fn skydome_system(
    mut events: EventReader<super::boat::MoveEvent>,
    state: Res<InGameState>,
    mut skydome: ResMut<SkyDome>,
    mut skydome_query: Query<(&SkyDomeLayer, &mut Transform), Without<super::WorldIsland>>,
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
                            // for (_, _, mut visible) in skydome_query.iter_mut() {
                            // visible.is_visible = false;
                            // }
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
                                // for (_, _, mut visible) in skydome_query.iter_mut() {
                                // visible.is_visible = true;
                                // }
                            } else {
                                skydome.rotation =
                                    (jump_skydome(ev.jump) * skydome.rotation).normalize();
                            }
                        }
                    }
                }
            }
        }

        for (_, mut sky_transform) in skydome_query.iter_mut() {
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
