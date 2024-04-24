use bevy::prelude::*;
use crate::AppState;
use crate::ui::*;

pub struct SettingsMenuPlugin;

impl Plugin for SettingsMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Settings), setup)
        .add_systems(Update, click_exit.run_if(in_state(AppState::Settings)))
        .add_systems(OnExit(AppState::Settings), cleanup);
    }

}

#[derive(Component)] struct Menu;

fn setup(mut commands: Commands) {

    let button_colors = ButtonColors::default();
    let general_colors = GeneralUi::default();
                commands.spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(60.0),
                            height: Val::Percent(70.0),
                            align_self: AlignSelf::Center,
                            
                            ..Default::default()
                        },
                        background_color: BackgroundColor::from(general_colors.overlay0),
                        ..default()
                    },
                    Menu
                ))
                .with_children(|c| {
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
                        button_colors,
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
                });
}

fn click_exit(
    mut next_state: ResMut<NextState<AppState>>,
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor, &ButtonColors), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, mut color, button_colors) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => { next_state.set(AppState::MainMenu) },
            Interaction::Hovered => { *color = button_colors.hovered.into() },
            Interaction::None => { *color = button_colors.red.into() },
        }
    }
}

fn cleanup(mut commands: Commands, menu: Query<Entity, With<Menu>>) {
    for e in menu.iter()  {
        commands.entity(e).despawn_recursive();
    }
}
