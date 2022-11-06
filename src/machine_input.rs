use bevy::prelude::*;

use crate::{map_keyboard::Position, machine::Machine, hex_map::Hex_Map};

pub fn add_machine_tiles(input: Res<Input<KeyCode>>, selected_location: Query<&Position>, mut plow : Query<&mut Machine>,tiles: Query<&Hex_Map>, ) {
    let mut plow = plow.get_single_mut().expect("plow");
    let tiles = tiles.get_single().expect("Tile");
    let selected_location = selected_location.get_single().expect("Location");
    if(input.just_pressed(KeyCode::B))
    {
        println!("Tile Added");
        let tile =& tiles.tiles[selected_location.x as usize][selected_location.z as usize];
        plow.add_hex((tile.get_center(),[selected_location.x,selected_location.z]))
    }
}
