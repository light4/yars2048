use bevy::prelude::*;

use crate::components::RunState;

pub struct ButtonColors {
    pub normal: Color,
    pub hovered: Color,
    pub pressed: Color,
}

impl FromWorld for ButtonColors {
    fn from_world(_: &mut World) -> Self {
        ButtonColors {
            normal: Color::rgb(0.75, 0.75, 0.9),
            hovered: Color::rgb(0.7, 0.7, 0.9),
            pressed: Color::rgb(0.6, 0.6, 1.0),
        }
    }
}

pub fn button_system(
    button_colors: Res<ButtonColors>,
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut run_state: ResMut<State<RunState>>,
) {
    for (interaction, mut color, children) in interaction_query.iter_mut() {
        let mut text = text_query
            .get_mut(
                *children
                    .first()
                    .expect("expect button to have a first child"),
            )
            .unwrap();
        match *interaction {
            Interaction::Clicked => {
                *color = button_colors.pressed.into();

                match run_state.current() {
                    RunState::Playing => {
                        run_state.set(RunState::GameOver).unwrap();
                    }
                    RunState::GameOver => {
                        run_state.set(RunState::Playing).unwrap();
                    }
                }
            }
            Interaction::Hovered => {
                *color = button_colors.hovered.into();
            }
            Interaction::None => {
                match run_state.current() {
                    RunState::Playing => {
                        text.sections[0].value = "End Game".to_string();
                    }
                    RunState::GameOver => {
                        text.sections[0].value = "New Game".to_string();
                    }
                }

                *color = button_colors.normal.into();
            }
        }
    }
}
