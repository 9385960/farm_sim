mod camera;
mod hex_map;
mod hex_outline;
mod input_plant;
mod light;
mod load_asset;
mod map_keyboard;
mod plant;
mod machine;
mod load_machines;
mod machine_update;
mod machine_input;

use bevy::prelude::*;
use machine_input::add_machine_tiles;
use crate::machine_update::machine_update;
use bevy_inspector_egui::WorldInspectorPlugin;
use camera::add_camera;
use hex_map::add_hex_map;
use hex_outline::add_outline;
use input_plant::add_plant;
use input_plant::despawn_plant;
use input_plant::grow_plant;
use light::{add_light, add_light_dir};
use load_machines::load_plow;
use map_keyboard::add_position;
use map_keyboard::initalize_position;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // .add_startup_system(load_asset::load_asset)
        .add_startup_system(add_camera)
        .add_startup_system(add_light_dir)
        .add_startup_system(add_hex_map)
        .add_startup_system(initalize_position)
        .add_startup_system(load_plow)
        .add_system(add_position)
        .add_system(add_plant)
        .add_system(despawn_plant)
        .add_system(grow_plant)
        .add_system(machine_update)
        .add_system(add_machine_tiles)
        .add_plugin(WorldInspectorPlugin::new())
        //.add_startup_system(add_outline.after(add_hex_map))
        .run()
}
