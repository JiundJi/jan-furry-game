use bevy::prelude::*;
use crate::GameState;
use crate::ui::*;

pub struct StartMenuPlugin;

impl Plugin for StartMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), setup)
        .add_systems(Update, click_settings.run_if(in_state(GameState::Settings)))
        .add_systems(Update, click_play.run_if(in_state(GameState::MainMenu)))
        .add_systems(OnExit(GameState::MainMenu), cleanup);
    }

}

#[derive(Component)] struct Menu;
#[derive(Component)] struct Settings;


fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                ..Default::default()
            },
            Menu,
        ))
        .with_children(|children| { // * play button
            let button_colors = ButtonColors::default();
            let general_colors = GeneralUi::default();

            children.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Vw(12.0),
                        height: Val::Vh(6.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,

                        ..Default::default()
                    },
                    background_color: BackgroundColor::from(button_colors.normal),
                    ..Default::default()
                },
                button_colors,
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Play", 
                    TextStyle {
                        font_size: 32.0, 
                        color: general_colors.text, 
                        ..Default::default()
                    }
                ));
            });
        })
        .with_children(|children| { // * settings button
            let button_colors = ButtonColors::default();
            let general_colors = GeneralUi::default();

            children.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Vw(12.0),
                        height: Val::Vh(6.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    ..default()
                },
                button_colors,
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Settings",
                    TextStyle {
                        font_size: 32.0,
                        color: general_colors.text,
                        ..Default::default()
                    }
                ));
            });
        });

}

fn click_play(
    mut next_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor, &ButtonColors), (Changed<Interaction>, With<Button>)>
) {
    for (interaction, mut color, button_colors) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => { next_state.set(GameState::Lobby) },
            Interaction::Hovered => { *color = button_colors.hovered.into() },
            Interaction::None => { *color = button_colors.normal.into() },
        }
    }
}

fn click_settings(
    mut next_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor, &ButtonColors), (Changed<Interaction>, With<Button>)>,
    mut commands: Commands,
) {
    for (interaction, mut color, button_colors) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => { 
                commands.spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(50.0),
                            height: Val::Percent(50.0),

                            ..Default::default()
                        },

                        ..default()
                    },
                    Settings
                ));
            },
            Interaction::Hovered => { *color = button_colors.hovered.into() },
            Interaction::None => { *color = button_colors.normal.into() },
        }
    }
}

fn cleanup(mut commands: Commands, menu: Query<Entity, With<Menu>>) {
    for e in menu.iter()  {
        commands.entity(e).despawn_recursive();
    }
}
