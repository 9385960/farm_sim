mod camera;
mod hex_map;
mod hex_outline;
mod light;
// mod mouse_to_map;

use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use camera::add_camera;
use hex_map::add_hex_map;
use hex_outline::add_outline;
use light::{add_light, add_light_dir};
// use mouse_to_map::handle_mouse_clicks;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(add_camera)
        .add_startup_system(add_light_dir)
        .add_startup_system(add_hex_map)
        // .add_system(handle_mouse_clicks)
        .add_plugin(WorldInspectorPlugin::new())
        //.add_startup_system(add_outline.after(add_hex_map))
        .run()
}
