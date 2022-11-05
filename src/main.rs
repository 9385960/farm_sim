use bevy::prelude::*;

const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

// Current GameState
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Menu,
    Game,
}

// Graphic Settings
#[derive(Debug, Component, PartialEq, Eq, Clone, Copy)]
enum DisplayQuality {
    Low,
    Medium,
    High,
}

// Volume Settings
#[derive(Debug, Component, PartialEq, Eq, Clone, Copy)]
struct Volume(u32);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Insert as resource the initial value for the settings resources
        .insert_resource(DisplayQuality::Medium)
        .insert_resource(Volume(7))
        .add_startup_system(setup)
        .add_state(GameState::Menu)
        .add_plugin(menu::MenuPlugin)
        .add_plugin(game::GamePlugin)
        .run();
}

// Setup world
fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera3dBundle::default());
}

// Main game loop
mod game {
    use bevy::prelude::*;

    use super::{despawn_screen, DisplayQuality, GameState, Volume, TEXT_COLOR};
    pub struct GamePlugin;

    impl Plugin for GamePlugin {
        fn build(&self, app: &mut App) {
            app.add_system_set(SystemSet::on_enter(GameState::Game).with_system(game_setup))
                .add_system_set(SystemSet::on_update(GameState::Game).with_system(game))
                .add_system_set(
                    SystemSet::on_exit(GameState::Game).with_system(despawn_screen::<OnGameScreen>),
                );
        }
    }

    // Tag component used to tag entities added on the game screen
    #[derive(Component)]
    struct OnGameScreen;

    fn game_setup(
        mut commands: Commands,
        // asset_server: Res<AssetServer>,
        // display_quality: Res<DisplayQuality>,
        // volume: Res<Volume>,
    ) {
        println!("In game loop");
        commands.spawn().insert(OnGameScreen);
    }

    // Change state on esc
    fn game(
        // time: Res<Time>,
        mut game_state: ResMut<State<GameState>>,
        keys: Res<Input<KeyCode>>,
        // mut timer: ResMut<GameTimer>,
    ) {
        if keys.just_pressed(KeyCode::Escape) {
            game_state.set(GameState::Menu).unwrap();
        }
    }
}

mod menu {
    use bevy::{app::AppExit, prelude::*};

    use super::{despawn_screen, DisplayQuality, GameState, Volume, TEXT_COLOR};

    pub struct MenuPlugin;

    impl Plugin for MenuPlugin {
        fn build(&self, app: &mut App) {
            app
                .add_state(MenuState::Main)
                // Main menu screen
                .add_system_set(SystemSet::on_enter(MenuState::Main).with_system(main_menu_setup))
                .add_system_set(
                    SystemSet::on_exit(MenuState::Main)
                        .with_system(despawn_screen::<OnMainMenuScreen>),
                )
                // Settings menu screen
                .add_system_set(
                    SystemSet::on_enter(MenuState::Settings).with_system(settings_menu_setup),
                )
                .add_system_set(
                    SystemSet::on_exit(MenuState::Settings)
                        .with_system(despawn_screen::<OnSettingsMenuScreen>),
                )
                // Display settings screen
                .add_system_set(
                    SystemSet::on_enter(MenuState::SettingsDisplay)
                        .with_system(display_settings_menu_setup),
                )
                .add_system_set(
                    SystemSet::on_update(MenuState::SettingsDisplay)
                        .with_system(setting_button::<DisplayQuality>),
                )
                .add_system_set(
                    SystemSet::on_exit(MenuState::SettingsDisplay)
                        .with_system(despawn_screen::<OnDisplaySettingsMenuScreen>),
                )
                // Sound settings screen
                .add_system_set(
                    SystemSet::on_enter(MenuState::SettingsSound)
                        .with_system(sound_settings_menu_setup),
                )
                .add_system_set(
                    SystemSet::on_update(MenuState::SettingsSound)
                        .with_system(setting_button::<Volume>),
                )
                .add_system_set(
                    SystemSet::on_exit(MenuState::SettingsSound)
                        .with_system(despawn_screen::<OnSoundSettingsMenuScreen>),
                )
                // Handles buttons behaviour
                .add_system_set(
                    SystemSet::on_update(GameState::Menu)
                        .with_system(menu_action)
                        .with_system(button_system),
                );
        }
    }

    #[derive(Clone, Eq, PartialEq, Debug, Hash)]
    enum MenuState {
        Main,
        Settings,
        SettingsDisplay,
        SettingsSound,
        Disabled,
    }

    #[derive(Component)]
    struct OnMainMenuScreen;

    #[derive(Component)]
    struct OnSettingsMenuScreen;

    #[derive(Component)]
    struct OnDisplaySettingsMenuScreen;

    #[derive(Component)]
    struct OnSoundSettingsMenuScreen;

    const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
    const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
    const HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.25, 0.65, 0.25);
    const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

    #[derive(Component)]
    struct SelectedOption;

    #[derive(Component)]
    enum MenuButtonAction {
        Play,
        Settings,
        SettingsDisplay,
        SettingsSound,
        BackToMainMenu,
        BackToSettings,
        Quit,
    }

    // Changes button colors on mouse interaction
    fn button_system(
        mut interaction_query: Query<
            (&Interaction, &mut UiColor, Option<&SelectedOption>),
            (Changed<Interaction>, With<Button>),
        >,
    ) {
        for (interaction, mut color, selected) in &mut interaction_query {
            *color = match (*interaction, selected) {
                (Interaction::Clicked, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
                (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
                (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
                (Interaction::None, None) => NORMAL_BUTTON.into(),
            }
        }
    }

    fn setting_button<T: Component + PartialEq + Copy>(
        interaction_query: Query<(&Interaction, &T, Entity), (Changed<Interaction>, With<Button>)>,
        mut selected_query: Query<(Entity, &mut UiColor), With<SelectedOption>>,
        mut commands: Commands,
        mut setting: ResMut<T>,
    ) {
        for (interaction, button_setting, entity) in &interaction_query {
            if *interaction == Interaction::Clicked && *setting != *button_setting {
                let (previous_button, mut previous_color) = selected_query.single_mut();
                *previous_color = NORMAL_BUTTON.into();
                commands.entity(previous_button).remove::<SelectedOption>();
                commands.entity(entity).insert(SelectedOption);
                *setting = *button_setting;
            }
        }
    }

    fn main_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
        println!("Test");

        let button_style = Style {
            size: Size::new(Val::Px(250.0), Val::Px(65.0)),
            margin: UiRect::all(Val::Px(20.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        };
        let button_icon_style = Style {
            size: Size::new(Val::Px(30.0), Val::Auto),
            // This takes the icons out of the flexbox flow, to be positioned exactly
            position_type: PositionType::Absolute,
            // The icon will be close to the left border of the button
            position: UiRect {
                left: Val::Px(10.0),
                right: Val::Auto,
                top: Val::Auto,
                bottom: Val::Auto,
            },
            ..default()
        };
        let button_text_style = TextStyle {
            font_size: 40.0,
            color: TEXT_COLOR,
            font: asset_server.load("FiraSans-Bold.ttf"),
        };
        commands
            .spawn_bundle(NodeBundle {
                style: Style {
                    margin: UiRect::all(Val::Auto),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                },
                color: Color::CRIMSON.into(),
                ..default()
            })
            .insert(OnMainMenuScreen)
            .with_children(|parent| {
                parent
                    .spawn_bundle(ButtonBundle {
                        style: button_style.clone(),
                        color: NORMAL_BUTTON.into(),
                        ..default()
                    })
                    .insert(MenuButtonAction::Quit)
                    .with_children(|parent| {
                        let icon = asset_server.load("arrow-right-from-bracket-solid.png");
                        parent.spawn_bundle(ImageBundle {
                            style: button_icon_style.clone(),
                            image: UiImage(icon),
                            ..default()
                        });
                        parent.spawn_bundle(TextBundle::from_section(
                            "Quit",
                            button_text_style.clone(),
                        ));
                    });

                parent
                    .spawn_bundle(ButtonBundle {
                        style: button_style.clone(),
                        color: NORMAL_BUTTON.into(),
                        ..default()
                    })
                    .insert(MenuButtonAction::Settings)
                    .with_children(|parent| {
                        let icon = asset_server.load("gear-solid.png");
                        parent.spawn_bundle(ImageBundle {
                            style: button_icon_style.clone(),
                            image: UiImage(icon),
                            ..default()
                        });
                        parent.spawn_bundle(TextBundle::from_section(
                            "Settings",
                            button_text_style.clone(),
                        ));
                    });

                parent
                    .spawn_bundle(ButtonBundle {
                        style: button_style.clone(),
                        color: NORMAL_BUTTON.into(),
                        ..default()
                    })
                    .insert(MenuButtonAction::Play)
                    .with_children(|parent| {
                        let icon = asset_server.load("arrow-right-solid.png");
                        parent.spawn_bundle(ImageBundle {
                            style: button_icon_style.clone(),
                            image: UiImage(icon),
                            ..default()
                        });
                        parent.spawn_bundle(TextBundle::from_section(
                            "New Game",
                            button_text_style.clone(),
                        ));
                    });

                parent.spawn_bundle(
                    TextBundle::from_section(
                        "bevy",
                        TextStyle {
                            font_size: 80.0,
                            color: TEXT_COLOR,
                            font: asset_server.load("FiraSans-Bold.ttf"),
                        },
                    )
                    .with_style(Style {
                        margin: UiRect::all(Val::Px(50.0)),
                        ..default()
                    }),
                );
            });
    }

    fn settings_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
        let button_style = Style {
            size: Size::new(Val::Px(200.0), Val::Px(65.0)),
            margin: UiRect::all(Val::Px(20.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        };

        let button_text_style = TextStyle {
            font: asset_server.load("FiraSans-Bold.ttf"),
            font_size: 40.0,
            color: TEXT_COLOR,
        };

        commands
            .spawn_bundle(NodeBundle {
                style: Style {
                    margin: UiRect::all(Val::Auto),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                },
                color: Color::CRIMSON.into(),
                ..default()
            })
            .insert(OnSettingsMenuScreen)
            .with_children(|parent| {
                for (action, text) in [
                    (MenuButtonAction::SettingsDisplay, "Display"),
                    (MenuButtonAction::SettingsSound, "Sound"),
                    (MenuButtonAction::BackToMainMenu, "Back"),
                ] {
                    parent
                        .spawn_bundle(ButtonBundle {
                            style: button_style.clone(),
                            color: NORMAL_BUTTON.into(),
                            ..default()
                        })
                        .insert(action)
                        .with_children(|parent| {
                            parent.spawn_bundle(TextBundle::from_section(
                                text,
                                button_text_style.clone(),
                            ));
                        });
                }
            });
    }

    fn display_settings_menu_setup(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        display_quality: Res<DisplayQuality>,
    ) {
        let button_style = Style {
            size: Size::new(Val::Px(200.0), Val::Px(65.0)),
            margin: UiRect::all(Val::Px(20.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        };
        let button_text_style = TextStyle {
            font: asset_server.load("FiraSans-Bold.ttf"),
            font_size: 40.0,
            color: TEXT_COLOR,
        };

        commands
            .spawn_bundle(NodeBundle {
                style: Style {
                    margin: UiRect::all(Val::Auto),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                },
                color: Color::CRIMSON.into(),
                ..default()
            })
            .insert(OnDisplaySettingsMenuScreen)
            .with_children(|parent| {
                parent
                    .spawn_bundle(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        color: Color::CRIMSON.into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        // Display a label for the current setting
                        parent.spawn_bundle(TextBundle::from_section(
                            "Display Quality",
                            button_text_style.clone(),
                        ));
                        // Display a button for each possible value
                        for quality_setting in [
                            DisplayQuality::Low,
                            DisplayQuality::Medium,
                            DisplayQuality::High,
                        ] {
                            let mut entity = parent.spawn_bundle(ButtonBundle {
                                style: Style {
                                    size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                                    ..button_style.clone()
                                },
                                color: NORMAL_BUTTON.into(),
                                ..default()
                            });
                            entity.insert(quality_setting).with_children(|parent| {
                                parent.spawn_bundle(TextBundle::from_section(
                                    format!("{quality_setting:?}"),
                                    button_text_style.clone(),
                                ));
                            });
                            if *display_quality == quality_setting {
                                entity.insert(SelectedOption);
                            }
                        }
                    });
                // Display the back button to return to the settings screen
                parent
                    .spawn_bundle(ButtonBundle {
                        style: button_style,
                        color: NORMAL_BUTTON.into(),
                        ..default()
                    })
                    .insert(MenuButtonAction::BackToSettings)
                    .with_children(|parent| {
                        parent.spawn_bundle(TextBundle::from_section("Back", button_text_style));
                    });
            });
    }

    fn sound_settings_menu_setup(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        volume: Res<Volume>,
    ) {
        let button_style = Style {
            size: Size::new(Val::Px(200.0), Val::Px(65.0)),
            margin: UiRect::all(Val::Px(20.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        };
        let button_text_style = TextStyle {
            font: asset_server.load("FiraSans-Bold.ttf"),
            font_size: 40.0,
            color: TEXT_COLOR,
        };

        commands
            .spawn_bundle(NodeBundle {
                style: Style {
                    margin: UiRect::all(Val::Auto),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                },
                color: Color::CRIMSON.into(),
                ..default()
            })
            .insert(OnSoundSettingsMenuScreen)
            .with_children(|parent| {
                parent
                    .spawn_bundle(NodeBundle {
                        style: Style {
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        color: Color::CRIMSON.into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        parent.spawn_bundle(TextBundle::from_section(
                            "Volume",
                            button_text_style.clone(),
                        ));
                        for volume_setting in [0, 1, 2, 3, 4, 5, 6, 7, 8, 9] {
                            let mut entity = parent.spawn_bundle(ButtonBundle {
                                style: Style {
                                    size: Size::new(Val::Px(30.0), Val::Px(65.0)),
                                    ..button_style.clone()
                                },
                                color: NORMAL_BUTTON.into(),
                                ..default()
                            });
                            entity.insert(Volume(volume_setting));
                            if *volume == Volume(volume_setting) {
                                entity.insert(SelectedOption);
                            }
                        }
                    });
                parent
                    .spawn_bundle(ButtonBundle {
                        style: button_style,
                        color: NORMAL_BUTTON.into(),
                        ..default()
                    })
                    .insert(MenuButtonAction::BackToSettings)
                    .with_children(|parent| {
                        parent.spawn_bundle(TextBundle::from_section("Back", button_text_style));
                    });
            });
    }

    fn menu_action(
        interaction_query: Query<
            (&Interaction, &MenuButtonAction),
            (Changed<Interaction>, With<Button>),
        >,
        mut app_exit_events: EventWriter<AppExit>,
        mut menu_state: ResMut<State<MenuState>>,
        mut game_state: ResMut<State<GameState>>,
    ) {
        for (interaction, menu_button_action) in &interaction_query {
            if *interaction == Interaction::Clicked {
                match menu_button_action {
                    MenuButtonAction::Quit => app_exit_events.send(AppExit),
                    MenuButtonAction::Play => {
                        game_state.set(GameState::Game).unwrap();
                        menu_state.set(MenuState::Disabled).unwrap();
                    }
                    MenuButtonAction::Settings => menu_state.set(MenuState::Settings).unwrap(),
                    MenuButtonAction::SettingsDisplay => {
                        menu_state.set(MenuState::SettingsDisplay).unwrap();
                    }
                    MenuButtonAction::SettingsSound => {
                        menu_state.set(MenuState::SettingsSound).unwrap();
                    }
                    MenuButtonAction::BackToMainMenu => menu_state.set(MenuState::Main).unwrap(),
                    MenuButtonAction::BackToSettings => {
                        menu_state.set(MenuState::Settings).unwrap();
                    }
                }
            }
        }
    }
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
