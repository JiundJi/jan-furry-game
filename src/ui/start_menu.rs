use bevy::app::AppExit;
use bevy::prelude::*;
use crate::GameState;
use crate::ui::*;

pub struct StartMenuPlugin;

impl Plugin for StartMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), setup)
        .add_systems(Update, button_system.run_if(in_state(GameState::MainMenu)))
        .add_systems(Update, menu_action.run_if(in_state(GameState::MainMenu)))
        .add_systems(OnExit(GameState::MainMenu), cleanup);
    }
}

#[derive(Component)] enum MenuButtonAction {
    Play,
    Settings,
    Quit,
}

#[derive(Component)] struct SelectedOption;
#[derive(Component)] struct Menu;

fn setup(mut commands: Commands) {

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

            children.spawn(( // * button
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
                button_colors, MenuButtonAction::Play
            ))
            .with_children(|parent| { // * text
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

            children.spawn(( // * button
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
                button_colors, MenuButtonAction::Settings
            ))
            .with_children(|parent| { // * text
                parent.spawn(TextBundle::from_section(
                    "Settings",
                    TextStyle {
                        font_size: 32.0,
                        color: general_colors.text,
                        ..Default::default()
                    }
                ));
            });
        })
        .with_children(|children| { // * quit button
            let button_colors = ButtonColors::default();
            let general_colors = GeneralUi::default();

            children.spawn(( // * button
                ButtonBundle {
                    style: Style {
                        width: Val::Vw(12.0),
                        height: Val::Vh(6.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    ..default()
                }, button_colors, MenuButtonAction::Quit
            ))
            .with_children(|parent| { // * text
                parent.spawn(TextBundle::from_section(
                    "Quit", 
                    TextStyle {
                        font_size: 32.0,
                        color: general_colors.text,
                        ..Default::default()
                    }
                ));
            });
        });

}

fn button_system(
    mut interaction_query: Query<
    (&Interaction, &mut BackgroundColor, Option<&SelectedOption>), 
    (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, selected) in &mut interaction_query {
        *color = BackgroundColor::from(match (*interaction, selected) {
            (Interaction::Pressed, _) => ButtonColors::default().normal,
            (Interaction::Hovered, None) => ButtonColors::default().hovered,
            (Interaction::Hovered, Some(_)) => ButtonColors::default().clicked,
            (Interaction::None, _) => ButtonColors::default().normal,
        })
    }
}

fn menu_action(
    interaction_query: Query<
    (&Interaction, &MenuButtonAction),
    (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit: EventWriter<AppExit>,
) {

    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Play => {
                    next_state.set(GameState::Lobby)
                },
                MenuButtonAction::Settings => {
                    next_state.set(GameState::Settings)
                },
                MenuButtonAction::Quit => {
                    exit.send(AppExit);
                },
            }
        }
    }

}

fn cleanup(mut commands: Commands, menu: Query<Entity, With<Menu>>) {
    for e in menu.iter()  {
        commands.entity(e).despawn_recursive();
    }
}
