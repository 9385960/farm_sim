use bevy::prelude::*;

const MARGINE: f32 = 0.1;

use crate::{hex_map::Hex_Map, machine::Machine, money::MoneyText};

pub fn machine_update(
    mut machine: Query<(&Parent, &mut Machine)>,
    mut machine_location: Query<&mut Transform>,
    mut tiles: Query<&mut Hex_Map>,
    time: Res<Time>,
) {
    let mut map = tiles.get_single_mut().expect("One map");
    for (parent, mut vehicle) in machine.iter_mut() {
        let vehicle_destination = vehicle.get_destination();
        let vehicle_destination = Vec3::new(
            vehicle_destination[0],
            vehicle_destination[1],
            vehicle_destination[2],
        );
        let mut p = machine_location.get_mut(parent.get()).expect("vehicle");
        if ((p.translation - vehicle_destination).length() < MARGINE) {
            match vehicle.machine_type {
                crate::machine::Model::Plow => {
                    if (!vehicle.action_completed
                        && !map.tiles[vehicle.hex_location[0] as usize]
                            [vehicle.hex_location[1] as usize]
                            .tilled
                        && !map.tiles[vehicle.hex_location[0] as usize]
                            [vehicle.hex_location[1] as usize]
                            .is_planted)
                    {
                        map.tiles[vehicle.hex_location[0] as usize]
                            [vehicle.hex_location[1] as usize]
                            .tilled = true;
                    }
                }
                crate::machine::Model::Harvester => {
                    if (!vehicle.action_completed
                        && map.tiles[vehicle.hex_location[0] as usize]
                            [vehicle.hex_location[1] as usize]
                            .is_planted)
                    {
                        map.tiles[vehicle.hex_location[0] as usize]
                            [vehicle.hex_location[1] as usize]
                            .is_planted = false;
                    }
                }
            }

            vehicle.update_destination();
        } else {
            let direction = (vehicle_destination - p.translation).normalize();
            let direction = Vec3::new(direction.x, 0.0, direction.z).normalize();
            p.translation = p.translation + direction * vehicle.speed * time.delta_seconds();
            // p.rotation = p.rotation.lerp(end, time.delta_seconds())
        }
    }
}
