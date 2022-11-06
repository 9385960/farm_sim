use std::fmt::{self, Debug, Display, Formatter};

use bevy::prelude::*;

const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

#[derive(Debug, Component)]
pub struct MoneyText {
    amount: f32,
}

impl MoneyText {
    pub fn new() -> MoneyText {
        MoneyText { amount: 100.0 }
    }

    pub fn add_money(&mut self, value: f32) {
        self.amount += value;
    }

    pub fn remove_money(&mut self, value: f32) {
        self.amount -= value;
    }
}

impl ToString for MoneyText {
    fn to_string(&self) -> String {
        format!("{:#?}", self.amount)
    }
}

#[derive(Bundle)]
pub struct MoneyTextBundle {
    money_text: MoneyText,
}

pub fn show_money(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Text with multiple sections
    commands
        .spawn_bundle(
            // Create a TextBundle that has a Text with a list of sections.
            TextBundle::from_sections([
                TextSection::new(
                    "Money: ",
                    TextStyle {
                        font: asset_server.load("FiraSans-Bold.ttf"),
                        font_size: 60.0,
                        color: Color::WHITE,
                    },
                ),
                TextSection::from_style(TextStyle {
                    font: asset_server.load("FiraSans-Bold.ttf"),
                    font_size: 60.0,
                    color: Color::GOLD,
                }),
            ])
            .with_style(Style {
                align_self: AlignSelf::FlexEnd,
                ..default()
            }),
        )
        .insert(MoneyText::new());
}

pub fn text_update_system(
    mut query: Query<&mut Text, With<MoneyText>>,
    mut amount: Query<&mut MoneyText>,
) {
    let mut value = amount.get_single_mut().expect("fdsaf");
    for mut text in &mut query {
        // Update the value of the second section
        text.sections[1].value = format!("{}", value.to_string());
    }
}
