use bevy::prelude::*;

use crate::{hex_map::HexMap, machine::Machine, map_keyboard::Position};

pub fn add_machine_tiles(
    input: Res<Input<KeyCode>>,
    selected_location: Query<&Position>,
    mut vehicles: Query<&mut Machine>,
    tiles: Query<&HexMap>,
) {
    for mut vehicle in vehicles.iter_mut() {
        match vehicle.machine_type {
            crate::machine::Model::Plow => {
                let tiles = tiles.get_single().expect("Tile");
                let selected_location = selected_location.get_single().expect("Location");
                if input.just_pressed(KeyCode::B) {
                    let tile =
                        &tiles.tiles[selected_location.x as usize][selected_location.z as usize];
                    vehicle.add_hex((
                        tile.get_center(),
                        [selected_location.x, selected_location.z],
                    ))
                }
            }
            crate::machine::Model::Harvester => {
                let tiles = tiles.get_single().expect("Tile");
                let selected_location = selected_location.get_single().expect("Location");
                if input.just_pressed(KeyCode::H) {
                    let tile =
                        &tiles.tiles[selected_location.x as usize][selected_location.z as usize];
                    vehicle.add_hex((
                        tile.get_center(),
                        [selected_location.x, selected_location.z],
                    ))
                }
            }
        }
    }
}
