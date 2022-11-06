mod camera;
mod hex_map;
mod hex_outline;
mod light;
mod load_asset;
mod map_keyboard;

use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use camera::add_camera;
use hex_map::add_hex_map;
use hex_outline::add_outline;
use light::{add_light, add_light_dir};
use map_keyboard::add_position;
use map_keyboard::initalize_position;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(load_asset::load_asset)
        .add_startup_system(add_camera)
        .add_startup_system(add_light_dir)
        .add_startup_system(add_hex_map)
        .add_startup_system(initalize_position)
        .add_system(add_position)
        .add_plugin(WorldInspectorPlugin::new())
        //.add_startup_system(add_outline.after(add_hex_map))
        .run()
}
