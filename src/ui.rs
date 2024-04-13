mod start_menu;

use bevy::prelude::*;
use catppuccin::Flavour;

use self::start_menu::StartMenuPlugin;

const FLAVOUR: Flavour = Flavour::Mocha;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((StartMenuPlugin));
    }
}

#[derive(Component)]
    pub struct GeneralUi {
        pub text: Color,
        pub surface0: Color,
        pub surface1: Color,
        pub surface2: Color,
        pub overlay0: Color,
        pub overlay1: Color,
        pub overlay2: Color,
        pub base: Color,
        pub mantle: Color,
        pub crust: Color,

        pub sapphire: Color,
}

impl Default for GeneralUi {
    fn default() -> Self {
        Self {
            text: Color::hex(FLAVOUR.text().hex()).unwrap(),
            surface0: Color::hex(FLAVOUR.surface0().hex()).unwrap(),
            surface1: Color::hex(FLAVOUR.surface1().hex()).unwrap(),
            surface2: Color::hex(FLAVOUR.surface2().hex()).unwrap(),
            overlay0: Color::hex(FLAVOUR.overlay0().hex()).unwrap(),
            overlay1: Color::hex(FLAVOUR.overlay1().hex()).unwrap(),
            overlay2: Color::hex(FLAVOUR.overlay2().hex()).unwrap(),
            base: Color::hex(FLAVOUR.base().hex()).unwrap(),
            mantle: Color::hex(FLAVOUR.mantle().hex()).unwrap(),
            crust: Color::hex(FLAVOUR.crust().hex()).unwrap(),

            sapphire: Color::hex(FLAVOUR.sapphire().hex()).unwrap(),
        }
    }
}

#[derive(Component)]
    pub struct ButtonColors {
        pub normal: Color,
        pub hovered: Color,
}

impl Default for ButtonColors {
    fn default() -> Self {
        Self {
            normal: Color::hex(FLAVOUR.surface0().hex()).unwrap(),
            hovered: Color::hex(FLAVOUR.overlay0().hex()).unwrap(),
        }
    }
}
