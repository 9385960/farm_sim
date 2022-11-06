use bevy::{ prelude::*};

#[derive(Component)]
pub struct Dirt{
    pub location : [u32;2],
}

use crate::{
    hex_map::HexMap,
    map_keyboard::Position,
    plant::{get_plant_path, Plant, PlantBundle, MAX_TIME, WHEAT_COST},
    util::get_world_from_hex, money::{ MoneyText}, machine_update::DirtFlag,
};

pub fn add_dirt(
    mut commands: Commands,
    mut dirt_locations: Query<(Entity,&mut DirtFlag)>,
    server: Res<AssetServer>,
) {
    for (mut entity ,mut dirt) in dirt_locations.iter_mut() {
        let seedlings_gltf = server.load("dirt.gltf#Scene0");
        let spawn_location = get_world_from_hex(dirt.location[0], dirt.location[1]);
        let mesh = commands
            .spawn_bundle(SceneBundle {
                scene: seedlings_gltf,
                transform: Transform {
                    translation: Vec3 {
                        x: spawn_location[0],
                        y: spawn_location[1],
                        z: spawn_location[2],
                    },
                    rotation: Quat::IDENTITY,
                    scale: Vec3 {
                        x: 0.9,
                        y: 0.9,
                        z: 0.9,
                    },
                },
                ..Default::default()
            }).insert(Dirt{
                location: dirt.location,
            });
            commands.entity(entity).despawn();
    }
}

pub fn despawn_dirt(to_despawn: Query<(Entity,&mut Dirt)>, mut commands: Commands,tiles: Query<& HexMap>) {
    let hex_map = tiles.get_single().expect("despawn map");
    for (entity, dirt) in &to_despawn {
        if !hex_map.tiles[dirt.location[0] as usize][dirt.location[1] as usize].tilled
        {
            commands.entity(entity).despawn_recursive();
        }
    }
}