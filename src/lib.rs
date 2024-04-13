mod asset_loading;
mod logic;
mod game;
mod ui;


use bevy::prelude::*;
#[cfg(debug_assertions)] use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use crate::ui::UiPlugin;
use crate::asset_loading::AssetLoadingPlugin;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
        #[default] Loading,
        MainMenu,
        CreationMenu,
        Lobby,
        InGame,
}


pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_plugins((AssetLoadingPlugin, UiPlugin));

        #[cfg(debug_assertions)]
        {
            app.add_plugins((FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin::default()));
        }
    }
}