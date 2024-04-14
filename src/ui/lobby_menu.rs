use bevy::prelude::*;
use crate::asset_loading::FontAssets;
use crate::game::Game;
use crate::GameState;
use crate::ui::*;
use bevy_simple_text_input::*;

pub struct LobbyMenuPlugin;

impl Plugin for LobbyMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Lobby), setup)
        .add_systems(Update, click.run_if(in_state(GameState::Lobby)))
        .add_systems(Update, text_input_listener.run_if(in_state(GameState::Lobby)))
        .add_systems(OnExit(GameState::Lobby), cleanup);
    }

}

#[derive(Component)] struct Menu;
#[derive(Component)] struct Lobby;

#[derive(Component)] struct SelectedOption;
#[derive(Component)] enum Button {
    Quit,
    Start,
    Join,
    Leave,
    Remove,
}


fn setup(mut commands: Commands, font_assets: Res<FontAssets>) {

    let general_colors = GeneralUi::default();
    let button_colors = ButtonColors::default();

    commands.spawn((Game::default(), Lobby));

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
        .with_children(|c| { // * quit button
            c.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Vw(6.0),
                        height: Val::Vh(6.0),
                        justify_content: JustifyContent::Center,
                        align_content: AlignContent::Center,
                        ..default()
                    },
                    background_color: BackgroundColor::from(general_colors.red),
                    ..Default::default()
                },
                button_colors, Button::Quit
            ))
            .with_children(|p| {
                p.spawn(
            TextBundle::from_section(
                "X", 
                TextStyle{
                        font_size: 32.0, 
                        color: general_colors.text, 
                        ..default()
                        }
                    )
                );
            });
        })
        .with_children(|c| { // * text input
            c.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Px(200.0),
                        border: UiRect::all(Val::Px(2.0)),
                        padding: UiRect::all(Val::Px(5.0)),
                        ..default()
                    },
                    border_color: BorderColor(general_colors.mantle),
                    background_color: general_colors.base.into(),
                    ..default()
                }, 
                TextInput {
                    text_style: TextStyle {
                        color: general_colors.text,
                        font: font_assets.jbmono_regular.clone(),
                        font_size: 24.0
                    },
                    inactive: true,
                }, general_colors
            ));
        })
        .with_children(|c| { // * start button
            let button_colors = ButtonColors::default();
            let general_colors = GeneralUi::default();

            c.spawn((
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
                button_colors, Button::Start
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Start", 
                    TextStyle {
                        font_size: 32.0, 
                        color: general_colors.text, 
                        ..Default::default()
                    }
                ));
            });
        });

}

fn click(
    interaction_query: Query<
        (&Interaction, &Button),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, button) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match button {
                Button::Quit => { next_state.set(GameState::MainMenu) },
                Button::Start => { next_state.set(GameState::InGame) },
                Button::Join => todo!(),
                Button::Leave => todo!(),
                Button::Remove => todo!(),
            }
        }
    }
}

fn text_input_listener(mut events: EventReader<TextInputSubmitEvent>) {
    for event in events.read() {
        
    }
}

fn button_system(
    mut interaction_query: Query<
    (&Interaction, &mut BackgroundColor, Option<&SelectedOption>), 
    (Changed<Interaction>, With<Button>),
    >,
) {
    let colors = ButtonColors::default();
    for (interaction, mut color, selected) in &mut interaction_query {
        *color = BackgroundColor::from(match (*interaction, selected) {
            (Interaction::Pressed, _) => colors.normal,
            (Interaction::Hovered, None) => colors.hovered,
            (Interaction::Hovered, Some(_)) => colors.clicked,
            (Interaction::None, _) => colors.normal,
        })
    }
}


fn cleanup(mut commands: Commands, menu: Query<Entity, With<Menu>>) {
    for e in menu.iter()  {
        commands.entity(e).despawn_recursive();
    }
}
