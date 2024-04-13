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
                .load_collection::<TextureAssets>()
        );
    }
}

#[derive(AssetCollection, Resource)]
    pub struct FontAssets {
        #[asset(path = "fonts/FiraSans-Bold.ttf")]
            pub fira_sans: Handle<Font>,
}

#[derive(AssetCollection, Resource)]
    pub struct AudioAssets {

}

#[derive(AssetCollection, Resource)]
    pub struct TextureAssets {
        #[asset(path = "textures/pointy_hex_tiles.png")]
            pub grassland: Handle<Image>,
}
