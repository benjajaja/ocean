use crate::boat;
use crate::AppState;
use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use bevy_egui::egui::Align2;
use bevy_egui::{egui, EguiContext};

struct FpsText;
struct BoatHUDText;

pub fn add_systems(app: &mut bevy::prelude::AppBuilder) -> &mut bevy::prelude::AppBuilder {
    app.add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(spawn_ui.system())
        .add_system(text_update_fps_system.system())
        .add_system(text_update_hud_system.system());

    app.add_system_set(SystemSet::on_update(AppState::Menu).with_system(ui_example.system()));
    app
}

fn spawn_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let font = asset_server.load("fonts/VCR_OSD_MONO_1.001.ttf");
    let font_size = 10.;

    commands.spawn_bundle(UiCameraBundle::default());

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::SpaceBetween,
                ..Default::default()
            },
            material: materials.add(Color::NONE.into()),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    style: Style {
                        align_self: AlignSelf::FlexEnd,
                        ..Default::default()
                    },
                    // Use `Text` directly
                    text: Text {
                        // Construct a `Vec` of `TextSection`s
                        sections: vec![
                            TextSection {
                                value: "FPS: ".to_string(),
                                style: TextStyle {
                                    font: font.clone(),
                                    font_size,
                                    color: Color::WHITE,
                                },
                            },
                            TextSection {
                                value: "".to_string(),
                                style: TextStyle {
                                    font: font.clone(),
                                    font_size,
                                    color: Color::GOLD,
                                },
                            },
                        ],
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(FpsText);
        });

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::SpaceBetween,
                ..Default::default()
            },
            material: materials.add(Color::NONE.into()),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    style: Style {
                        align_self: AlignSelf::FlexStart, // bottom
                        ..Default::default()
                    },
                    // Use `Text` directly
                    text: Text {
                        // Construct a `Vec` of `TextSection`s
                        sections: vec![
                            TextSection {
                                value: "Throttle: ".to_string(),
                                style: TextStyle {
                                    font: font.clone(),
                                    font_size,
                                    color: Color::WHITE,
                                },
                            },
                            TextSection {
                                value: "".to_string(),
                                style: TextStyle {
                                    font: font.clone(),
                                    font_size,
                                    color: Color::GOLD,
                                },
                            },
                            TextSection {
                                value: " Speed: ".to_string(),
                                style: TextStyle {
                                    font: font.clone(),
                                    font_size,
                                    color: Color::WHITE,
                                },
                            },
                            TextSection {
                                value: "".to_string(),
                                style: TextStyle {
                                    font: font.clone(),
                                    font_size,
                                    color: Color::RED,
                                },
                            },
                        ],
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(BoatHUDText);
        });
}

fn text_update_fps_system(
    diagnostics: Res<Diagnostics>,
    mut fps_query: Query<&mut Text, With<FpsText>>,
) {
    for mut text in fps_query.iter_mut() {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                text.sections[1].value = format!("{:.2}", average);
            }
        }
    }
}

fn text_update_hud_system(
    mut hud_query: Query<&mut Text, With<BoatHUDText>>,
    boat_query: Query<&boat::PlayerBoat>,
) {
    for mut text in hud_query.iter_mut() {
        if let Ok(boat) = boat_query.single() {
            text.sections[1].value = format!("{:.2}", boat.throttle);
            text.sections[3].value = format!("{:.2}", boat.speed);
        }
    }
}

// Note the usage of `ResMut`. Even though `ctx` method doesn't require
// mutability, accessing the context from different threads will result
// into panic if you don't enable `egui/multi_threaded` feature.
fn ui_example(egui_context: ResMut<EguiContext>) {
    egui::Window::new("Storage")
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .show(egui_context.ctx(), |ui| {
            ui.label("Pharaoh");
        });
}
