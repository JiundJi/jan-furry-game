use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub struct AssetLoadingPlugin;

impl Plugin for AssetLoadingPlugin {
    fn build(&self, app: &mut App) {

        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::MainMenu)
                .load_collection::<FontAssets>()
                .load_collection::<AudioAssets>()
                .load_collection::<ImageAssets>()
        );
    }
}

#[derive(AssetCollection, Resource)]
    pub struct FontAssets {
        #[asset(path = "fonts/JetBrainsMono-Regular.ttf")]
            pub jbmono_regular: Handle<Font>,
}

#[derive(AssetCollection, Resource)]
    pub struct AudioAssets {

}

#[derive(AssetCollection, Resource)]
    pub struct ImageAssets {
        #[asset(path = "images/jan bw.png")]
            pub icon: Handle<Image>,
}