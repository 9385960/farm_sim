mod camera;
mod hex_map;
mod input_plant;
mod light;
mod load_machines;
mod machine;
mod machine_input;
mod machine_update;
mod map_keyboard;
mod money;
mod plant;
mod util;

use crate::machine_update::machine_update;
use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_inspector_egui::egui::Order;
use camera::add_camera;
use hex_map::add_hex_map;
use input_plant::add_plant;
use input_plant::despawn_plant;
use input_plant::grow_plant;
use light::add_light_dir;
use load_machines::load_harvester;
use load_machines::load_plow;
use machine_input::add_machine_tiles;
use map_keyboard::add_position;
use map_keyboard::initalize_position;
use money::show_money;
use money::text_update_system;

#[derive(SystemLabel)]
enum Ordering {
    Input,
    Update,
    Delete,
}


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(add_camera)
        .add_startup_system(add_light_dir)
        .add_startup_system(add_hex_map)
        .add_startup_system(initalize_position)
        .add_startup_system(load_plow)
        .add_startup_system(show_money)
        .add_startup_system(load_harvester)
        .add_system(add_position.label(Ordering::Input).before(Ordering::Update))
        .add_system(add_plant.label(Ordering::Input).before(Ordering::Update))
        .add_system(despawn_plant.label(Ordering::Delete).after(Ordering::Update))
        .add_system(grow_plant.label(Ordering::Update).before(Ordering::Delete).after(Ordering::Input))
        .add_system(machine_update.label(Ordering::Update).before(Ordering::Delete).after(Ordering::Input))
        .add_system(add_machine_tiles.label(Ordering::Input).before(Ordering::Update))
        .add_system(text_update_system.label(Ordering::Update).before(Ordering::Delete).after(Ordering::Input))
        .add_plugin(WorldInspectorPlugin::new())
        .run()
}
