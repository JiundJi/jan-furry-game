mod start_menu;
mod settings_menu;

use bevy::prelude::*;
use catppuccin::Flavour;

use self::start_menu::StartMenuPlugin;
use self::settings_menu::SettingsMenuPlugin;

const FLAVOUR: Flavour = Flavour::Mocha;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((StartMenuPlugin, SettingsMenuPlugin));
    }
}

#[derive(Component)]
    pub struct GeneralUi {
        pub text: Color,
        pub subtext1: Color,
        pub subtext0: Color,
        pub overlay2: Color,
        pub overlay1: Color,
        pub overlay0: Color,
        pub surface2: Color,
        pub surface1: Color,
        pub surface0: Color,
        pub base: Color,
        pub mantle: Color,
        pub crust: Color,

        pub rosewater: Color,
        pub flamingo: Color,
        pub pink: Color,
        pub mauve: Color,
        pub red: Color,
        pub maroon: Color,
        pub peach: Color,
        pub yellow: Color,
        pub green: Color,
        pub teal: Color,
        pub sky: Color,
        pub sapphire: Color,
        pub blue: Color,
        pub lavender: Color,
}

impl Default for GeneralUi {
    fn default() -> Self {
        Self {
            text: Color::hex(FLAVOUR.text().hex()).unwrap(),
            subtext1: Color::hex(FLAVOUR.subtext1().hex()).unwrap(),
            subtext0: Color::hex(FLAVOUR.subtext0().hex()).unwrap(),
            surface2: Color::hex(FLAVOUR.surface2().hex()).unwrap(),
            surface1: Color::hex(FLAVOUR.surface1().hex()).unwrap(),
            surface0: Color::hex(FLAVOUR.surface0().hex()).unwrap(),
            overlay2: Color::hex(FLAVOUR.overlay2().hex()).unwrap(),
            overlay1: Color::hex(FLAVOUR.overlay1().hex()).unwrap(),
            overlay0: Color::hex(FLAVOUR.overlay0().hex()).unwrap(),
            base: Color::hex(FLAVOUR.base().hex()).unwrap(),
            mantle: Color::hex(FLAVOUR.mantle().hex()).unwrap(),
            crust: Color::hex(FLAVOUR.crust().hex()).unwrap(),

            
            rosewater: Color::hex(FLAVOUR.rosewater().hex()).unwrap(),
            flamingo: Color::hex(FLAVOUR.flamingo().hex()).unwrap(),
            pink: Color::hex(FLAVOUR.pink().hex()).unwrap(),
            mauve: Color::hex(FLAVOUR.mauve().hex()).unwrap(),
            red: Color::hex(FLAVOUR.red().hex()).unwrap(),
            maroon: Color::hex(FLAVOUR.maroon().hex()).unwrap(),
            peach: Color::hex(FLAVOUR.peach().hex()).unwrap(),
            yellow: Color::hex(FLAVOUR.yellow().hex()).unwrap(),
            green: Color::hex(FLAVOUR.green().hex()).unwrap(),
            teal: Color::hex(FLAVOUR.teal().hex()).unwrap(),
            sky: Color::hex(FLAVOUR.sky().hex()).unwrap(),
            sapphire: Color::hex(FLAVOUR.sapphire().hex()).unwrap(),
            blue: Color::hex(FLAVOUR.blue().hex()).unwrap(),
            lavender: Color::hex(FLAVOUR.lavender().hex()).unwrap(),
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
