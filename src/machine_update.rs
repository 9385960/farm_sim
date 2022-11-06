use std::f32::consts::PI;

use bevy::{prelude::*, ecs::system::Command};

#[derive(Component)]
pub struct DirtFlag{
    pub location : [u32;2]
}

#[derive(Bundle)]
pub struct DirtFlagBundle
{
    pub flag : DirtFlag,
}

const MARGINE: f32 = 0.1;
const ROATATION_SPEED : f32 = 3.0;

use crate::{hex_map::HexMap, machine::Machine};

pub fn machine_update(
    mut machine: Query<(&Parent, &mut Machine)>,
    mut machine_location: Query<&mut Transform>,
    mut commands : Commands,
    mut tiles: Query<&mut HexMap>,
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
        if (p.translation - vehicle_destination).length() < MARGINE {
            match vehicle.machine_type {
                crate::machine::Model::Plow => {
                    if !vehicle.action_completed
                        && !map.tiles[vehicle.hex_location[0] as usize]
                            [vehicle.hex_location[1] as usize]
                            .tilled
                        && !map.tiles[vehicle.hex_location[0] as usize]
                            [vehicle.hex_location[1] as usize]
                            .is_planted
                    {
                        map.tiles[vehicle.hex_location[0] as usize]
                            [vehicle.hex_location[1] as usize]
                            .tilled = true;
                        commands.spawn_bundle(DirtFlagBundle{
                            flag : DirtFlag { location: [vehicle.hex_location[0],vehicle.hex_location[1]]
                             }
                        });
                    }
                }
                crate::machine::Model::Harvester => {
                    if !vehicle.action_completed
                        && map.tiles[vehicle.hex_location[0] as usize]
                            [vehicle.hex_location[1] as usize]
                            .is_planted
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
            let forward = p.forward();
            let forward = Vec3::new(forward.x,0.0,forward.z).normalize();
            let end = match vehicle.machine_type {
                crate::machine::Model::Plow => Quat::from_rotation_arc(Vec3::NEG_Z,direction),
                crate::machine::Model::Harvester => Quat::from_rotation_arc(Vec3::Z,direction),
            };
            match vehicle.machine_type{
                crate::machine::Model::Plow => p.translation = p.translation + forward * vehicle.speed * time.delta_seconds(),
                crate::machine::Model::Harvester => p.translation = p.translation + -forward * vehicle.speed * time.delta_seconds(),
            };
            p.rotation = p.rotation.slerp(end, time.delta_seconds()*ROATATION_SPEED);

        }
    }
}
