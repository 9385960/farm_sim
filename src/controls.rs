use bevy::prelude::*;
use bevy::prelude::{AssetServer, Bundle, Commands, Component, Res};

#[derive(Debug, Component)]
pub struct HelpText {
    commands: String,
}

#[derive(Bundle)]
pub struct TextBundle {
    help_text: HelpText,
}

impl HelpText {
    pub fn new() -> HelpText {
        HelpText {
            commands: "Commands:
    B - Tile tile
    Space - Plant
    H - Harvest"
                .to_string(),
        }
    }
}

pub fn display_help(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(
            TextBundle::from_sections([
                TextSection::new(
                    "Help: ",
                    TextStyle {
                        font: asset_server.load("FiraSans-Bold.ttf"),
                        font_size: 30.0,
                        color: Color::WHITE,
                    },
                ),
                TextSection::from_style(TextStyle {
                    font: asset_server.load("FiraSans-Bold.ttf"),
                    font_size: 30.0,
                    color: Color::GOLD,
                }),
            ])
            .with_style(Style {
                align_self: AlignSelf::FlexEnd,
                ..default()
            }),
        )
        .insert(HelpText::new());
}
