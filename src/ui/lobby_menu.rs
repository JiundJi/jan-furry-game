use bevy::prelude::*;
use crate::asset_loading::FontAssets;
use crate::{ui::*, AppState};
use bevy_simple_text_input::*;

pub struct LobbyMenuPlugin;

impl Plugin for LobbyMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Lobby), setup)
        .add_systems(Update, click.run_if(in_state(AppState::Lobby)))
        .add_systems(Update, button_system.run_if(in_state(AppState::Lobby)))
        .add_systems(Update, text_input_listener.run_if(in_state(AppState::Lobby)))
        .add_systems(OnExit(AppState::Lobby), cleanup);
    }

}

#[derive(Component)] struct Menu;
#[derive(Component)] struct Lobby;

#[derive(Component)] struct SelectedOption;
#[derive(Component)] struct NameInput;
#[derive(Component, PartialEq)] enum Button {
    Quit,
    Start,
    Join,
    Leave,
    Remove,
}


fn setup(mut commands: Commands, font_assets: Res<FontAssets>) {    
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
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
            let meow = ButtonStyle::x();
            c.spawn((
                meow.bundle, meow.colors, Button::Quit))
            .with_children(|p| {
                p.spawn(TextBundle::from_section("X", meow.text));});
        })
        .with_children(|c| { // * text input
            let general_colors = GeneralUi::default();
            c.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Px(200.),
                        border: UiRect::all(Val::Px(2.)),
                        padding: UiRect::all(Val::Px(5.)),
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
                        font_size: 24.
                    },
                    inactive: true,
                }, general_colors, NameInput,
            ));
        })
        .with_children(|c| { // * join button
            let meow = ButtonStyle::long();
            c.spawn((
                meow.bundle, meow.colors, Button::Join))
            .with_children(|p| {
                p.spawn(TextBundle::from_section("Join Game", meow.text));});

        })
        .with_children(|c| { // * leave button
            let meow = ButtonStyle::long();
            c.spawn((
                meow.bundle, meow.colors, Button::Leave
                ))
                .with_children(|p|{
                    p.spawn(TextBundle::from_section("Leave", meow.text));
                });
        })
        .with_children(|c| { // * start button
            let meow = ButtonStyle::long();
            c.spawn((meow.bundle, meow.colors, Button::Start))
            .with_children(|p| {p.spawn(TextBundle::from_section("Start", meow.text));});}
    );
}

fn click(
    interaction_query: Query<
        (&Interaction, &Button),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (interaction, button) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match button {
                Button::Quit => { next_state.set(AppState::MainMenu) },
                Button::Start => { next_state.set(AppState::InGame) },
                Button::Join => todo!(),
                Button::Leave => todo!(),
                Button::Remove => todo!(),
            }
        }
    }
}

fn text_input_listener(mut events: EventReader<TextInputSubmitEvent>) {
    for event in events.read() {
        info!("{:?} submitted: {}", event.entity, event.value);
        todo!();
    }
}

fn button_system(
    mut interaction_query: Query<
    (&Interaction, &mut BackgroundColor, Option<&SelectedOption>, &Button), 
    (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, selected, button) in &mut interaction_query {
        let button_type = match button {
                    Button::Quit => ButtonStyle::x(),
                    Button::Start => ButtonStyle::long(),
                    Button::Join => ButtonStyle::long(),
                    Button::Leave => ButtonStyle::long(),
                    Button::Remove => ButtonStyle::smol_quadratic(),
                };
        
        *color = BackgroundColor::from(match (*interaction, selected) {
            (Interaction::Pressed, _) => button_type.colors.normal,
            (Interaction::Hovered, None) => button_type.colors.hovered,
            (Interaction::Hovered, Some(_)) => button_type.colors.clicked,
            (Interaction::None, _) => button_type.colors.normal,
        })
    }
}


fn cleanup(mut commands: Commands, menu: Query<Entity, With<Menu>>) {
    for e in menu.iter()  {
        commands.entity(e).despawn_recursive();
    }
}
