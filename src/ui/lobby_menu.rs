use bevy::prelude::*;
use crate::GameState;
use crate::ui::*;

pub struct LobbyMenuPlugin;

impl Plugin for LobbyMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Lobby), setup)
        .add_systems(Update, click.run_if(in_state(GameState::Lobby)))
        .add_systems(OnExit(GameState::Lobby), cleanup);
    }

}

#[derive(Component)] struct Menu;



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
        .with_children(|children| {
            
        })
        .with_children(|children| { // start button
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

fn click() {

}

fn cleanup(mut commands: Commands, menu: Query<Entity, With<Menu>>) {
    for e in menu.iter()  {
        commands.entity(e).despawn_recursive();
    }
}
