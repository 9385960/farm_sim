mod camera;
mod hex_map;
mod hex_outline;
mod light;

use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use camera::add_camera;
use hex_map::add_hex_map;
use hex_outline::add_outline;
use light::add_light;

#[derive(SystemLabel)]
enum Order {
    Map,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(add_camera)
        .add_startup_system(add_light)
        .add_startup_system(add_hex_map.label(Order::Map))
        .add_system(add_outline.after(Order::Map))
        .add_plugin(WorldInspectorPlugin::new())
        .run()
}
