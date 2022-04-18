mod buttons;

use bevy::prelude::*;

use crate::components::{BestScoreDisplay, Game, ScoreDisplay};
use buttons::{button_system, ButtonColors};

const BACKGROUND_COLOR: Color = Color::rgb(0.73, 0.68, 0.63);

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_ui)
            .init_resource::<ButtonColors>()
            .add_system(button_system)
            .add_system(scoreboard);
    }
}

fn setup_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    button_colors: Res<ButtonColors>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                display: Display::Flex,
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexEnd,
                border: Rect::all(Val::Px(50.0)),
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Yars 2048",
                    TextStyle {
                        font: font.clone(),
                        font_size: 40.0,
                        color: Color::WHITE,
                    },
                    TextAlignment {
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Center,
                    },
                ),
                ..default()
            });

            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        display: Display::Flex,
                        align_items: AlignItems::FlexEnd,
                        justify_content: JustifyContent::Center,
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        ..default()
                    },
                    color: Color::NONE.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // scorebox
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                display: Display::Flex,
                                flex_direction: FlexDirection::ColumnReverse,
                                align_items: AlignItems::Center,
                                size: Size::new(Val::Px(100.0), Val::Px(60.0)),
                                margin: Rect {
                                    left: Val::Px(20.0),
                                    right: Val::Px(20.0),
                                    top: Val::Px(0.0),
                                    bottom: Val::Px(0.0),
                                },
                                border: Rect::all(Val::Px(10.0)),
                                ..default()
                            },
                            color: BACKGROUND_COLOR.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn_bundle(TextBundle {
                                text: Text::with_section(
                                    "Score",
                                    TextStyle {
                                        font: font.clone(),
                                        font_size: 20.0,
                                        color: Color::WHITE,
                                    },
                                    TextAlignment {
                                        vertical: VerticalAlign::Center,
                                        horizontal: HorizontalAlign::Center,
                                    },
                                ),
                                ..default()
                            });
                            parent
                                .spawn_bundle(TextBundle {
                                    text: Text::with_section(
                                        "<score>",
                                        TextStyle {
                                            font: font.clone(),
                                            font_size: 30.0,
                                            color: Color::WHITE,
                                        },
                                        TextAlignment {
                                            vertical: VerticalAlign::Center,
                                            horizontal: HorizontalAlign::Center,
                                        },
                                    ),
                                    ..default()
                                })
                                .insert(ScoreDisplay);
                        });
                    // end scorebox
                    // best scorebox
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                display: Display::Flex,
                                flex_direction: FlexDirection::ColumnReverse,
                                align_items: AlignItems::Center,
                                size: Size::new(Val::Px(100.0), Val::Px(60.0)),
                                border: Rect::all(Val::Px(10.0)),
                                ..default()
                            },
                            color: BACKGROUND_COLOR.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn_bundle(TextBundle {
                                text: Text::with_section(
                                    "Best",
                                    TextStyle {
                                        font: font.clone(),
                                        font_size: 20.0,
                                        color: Color::WHITE,
                                    },
                                    TextAlignment {
                                        vertical: VerticalAlign::Center,
                                        horizontal: HorizontalAlign::Center,
                                    },
                                ),
                                ..default()
                            });
                            parent
                                .spawn_bundle(TextBundle {
                                    text: Text::with_section(
                                        "<score>",
                                        TextStyle {
                                            font: font.clone(),
                                            font_size: 30.0,
                                            color: Color::WHITE,
                                        },
                                        TextAlignment {
                                            vertical: VerticalAlign::Center,
                                            horizontal: HorizontalAlign::Center,
                                        },
                                    ),
                                    ..default()
                                })
                                .insert(BestScoreDisplay);
                        });
                    // end best scorebox
                });
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(100.0), Val::Px(60.0)),
                        // center button
                        // margin: Rect::all(Val::Auto),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        margin: Rect {
                            left: Val::Px(20.0),
                            right: Val::Px(20.0),
                            top: Val::Px(20.0),
                            bottom: Val::Px(20.0),
                        },
                        ..default()
                    },
                    color: button_colors.normal.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Button",
                            TextStyle {
                                font: font.clone(),
                                font_size: 20.0,
                                color: Color::WHITE,
                            },
                            default(),
                        ),
                        ..default()
                    });
                });
        });
}

// update the score displayed during the game
fn scoreboard(
    game: Res<Game>,
    mut query_scores: ParamSet<(
        Query<&mut Text, With<ScoreDisplay>>,
        Query<&mut Text, With<BestScoreDisplay>>,
    )>,
) {
    let mut p0 = query_scores.p0();
    let mut text = p0.single_mut();
    text.sections[0].value = game.score.to_string();

    let mut p1 = query_scores.p1();
    let mut best_text = p1.single_mut();
    best_text.sections[0].value = game.score_best.to_string();
}
