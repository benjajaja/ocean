use bevy::{
    prelude::*,
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
};
use crate::boat;

struct FpsText;
struct BoatHUDText;

pub fn add_systems(app: &mut bevy::prelude::AppBuilder) -> &mut bevy::prelude::AppBuilder {
    app.add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(spawn_ui.system())
        .add_system(text_update_system.system())
}

fn spawn_ui(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(UiCameraBundle::default())
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                flex_direction: FlexDirection::Column,
                border: Rect::all(Val::Px(2.0)),
                ..Default::default()
            },
            material: materials.add(Color::NONE.into()),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
            .spawn(NodeBundle {
                style: Style {
                    justify_content: JustifyContent::SpaceBetween,
                    border: Rect::all(Val::Px(2.0)),
                    ..Default::default()
                },
                material: materials.add(Color::NONE.into()),
                ..Default::default()
            }).with_children(|parent| {
                parent
                    .spawn(TextBundle {
                        style: Style {
                            align_self: AlignSelf::FlexStart,
                            ..Default::default()
                        },
                        text: Text {
                            value: "♥♥♥".to_string(),
                            font: asset_server.load("fonts/3270/3270-Regular.ttf"),
                            style: TextStyle {
                                font_size: 60.0,
                                color: Color::BLUE,
                                ..Default::default()
                            },
                        },
                        ..Default::default()
                    })
                    .spawn(TextBundle {
                        style: Style {
                            align_self: AlignSelf::FlexEnd,
                            ..Default::default()
                        },
                        text: Text {
                            value: "0.00thr".to_string(),
                            font: asset_server.load("fonts/3270/3270-Regular.ttf"),
                            style: TextStyle {
                                font_size: 60.0,
                                color: Color::GRAY,
                                ..Default::default()
                            },
                        },
                        ..Default::default()
                    })
                    .with(BoatHUDText);
            })
            .spawn(NodeBundle {
                style: Style {
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    border: Rect::all(Val::Px(2.0)),
                    ..Default::default()
                },
                material: materials.add(Color::NONE.into()),
                ..Default::default()
            }).with_children(|parent| {
                parent
                    .spawn(TextBundle {
                        style: Style {
                            ..Default::default()
                        },
                        text: Text {
                            value: "FPS: ...".to_string(),
                            font: asset_server.load("fonts/3270/3270-Regular.ttf"),
                            style: TextStyle {
                                font_size: 60.0,
                                color: Color::WHITE,
                                ..Default::default()
                            },
                        },
                        ..Default::default()
                    })
                    .with(FpsText);
            })
            ;
        });
}

fn text_update_system(
    diagnostics: Res<Diagnostics>,
    mut fps_query: Query<&mut Text, With<FpsText>>,
    mut boat_hud_query: Query<&mut Text, With<BoatHUDText>>,
    boat_query: Query<&boat::PlayerBoat>,
) {
    for mut text in fps_query.iter_mut() {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                text.value = format!("FPS: {:.2}", average);
            }
        }
    }
    for mut text in boat_hud_query.iter_mut() {
        if let Some(boat) = boat_query.iter().next() {
            text.value = format!("{:.2}thr", boat.thrust);
        }
    }
}

