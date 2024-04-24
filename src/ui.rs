mod start_menu;
mod settings_menu;
mod lobby_menu;

use bevy::prelude::*;
use catppuccin::Flavour;

use self::start_menu::StartMenuPlugin;
use self::settings_menu::SettingsMenuPlugin;
use self::lobby_menu::LobbyMenuPlugin;

const FLAVOUR: Flavour = Flavour::Mocha;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((StartMenuPlugin, SettingsMenuPlugin, LobbyMenuPlugin));
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
    pub important: Color,
    pub green: Color,
    pub red: Color,
    pub hovered: Color,
    pub clicked: Color,
}

impl Default for ButtonColors {
    fn default() -> Self {
        Self {
            normal: Color::hex(FLAVOUR.surface0().hex()).unwrap(),
            hovered: Color::hex(FLAVOUR.overlay0().hex()).unwrap(),
            important: Color::hex(FLAVOUR.yellow().hex()).unwrap(),
            green: Color::hex(FLAVOUR.green().hex()).unwrap(),
            red: Color::hex(FLAVOUR.red().hex()).unwrap(),
            clicked: Color::hex(FLAVOUR.overlay2().hex()).unwrap(),
        }
    }
}




#[derive(Resource)]
pub struct ButtonStyle {
    bundle: ButtonBundle,
    colors: ButtonColors,
    text: TextStyle,
}

impl ButtonStyle {

    fn long() -> ButtonStyle {
        ButtonStyle {
            bundle: ButtonBundle {
                style: Style {
                    width: Val::Vw(12.),
                    height: Val::Vh(6.),
                    ..Default::default()
                },
                background_color: BackgroundColor::from(ButtonColors::default().normal),
                ..Default::default()
            },
            colors: ButtonColors::default(),
            text: TextStyle {
                font_size: 32.,
                color: GeneralUi::default().text,
                ..Default::default()
            }
        }
    }
    fn smol_quadratic() -> ButtonStyle {
        ButtonStyle {
            bundle: ButtonBundle {
                style: Style {
                    width: Val::Vw(4.),
                    height: Val::Vh(4.),
                    ..Default::default()
                },
                ..Default::default()
            },
            colors: ButtonColors::default(),
            text: TextStyle {

                ..Default::default()
            }
        }
    }
}