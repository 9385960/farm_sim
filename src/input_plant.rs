use bevy::{ecs::entity, prelude::*};

use crate::{
    hex_map::{
        hex_tile::{INNER_RADIUS, OUTER_RADIUS},
        Hex_Map,
    },
    map_keyboard::Position,
    plant::{getPlantPath, Plant, PlantBundle, MAX_TIME},
};

pub fn add_plant(
    mut commands: Commands,
    mut input: Res<Input<KeyCode>>,
    mut selected_location: Query<&Position>,
    mut plant_components: Query<&mut Plant>,
    mut tiles: Query<&mut Hex_Map>,
    time: Res<Time>,
    server: Res<AssetServer>,
) {
    let mut canPlant = true;
    let selected_location = selected_location.get_single().expect("dfad");
    for mut plant in plant_components.iter_mut() {
        plant.increase_lifetime(time.delta_seconds());

        if (plant.get_tile()[0] == selected_location.x
            && plant.get_tile()[1] == selected_location.z)
        {
            canPlant = false;
        }
    }

    let mut tiles = tiles.get_single_mut().expect("dfa");
    if !(tiles.tiles[selected_location.x as usize][selected_location.z as usize]).tilled
        && (tiles.tiles[selected_location.x as usize][selected_location.z as usize]).is_planted
    {
        canPlant = false;
    }

    if input.just_pressed(KeyCode::Space) && canPlant {
        tiles.tiles[selected_location.x as usize][selected_location.z as usize].is_planted = true;
        let seedlings_gltf = server.load("seedlings.gltf#Scene0");
        let spawn_location = get_world_from_hex(selected_location.x, selected_location.z);
        let plant_bundle = commands
            .spawn_bundle(PlantBundle {
                bundle: Plant::new(
                    crate::plant::Type::Wheat,
                    [selected_location.x, selected_location.z],
                ),
            })
            .id();

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
            })
            .id();

        commands.entity(mesh).push_children(&[plant_bundle]);
    }

    fn get_world_from_hex(x: u32, z: u32) -> [f32; 3] {
        let mut position = [0.0, 0.0, 0.0];
        let x = x as i32;
        let z = z as i32;
        position[0] = (((x - z / 2) as f32) + (z as f32) * 0.5) * INNER_RADIUS * 2.0;
        position[2] = (z as f32) * OUTER_RADIUS * 1.5;
        position
    }
}

pub fn despawn_plant(to_despawn: Query<(&Plant, &Parent)>, mut commands: Commands,tiles: Query<& Hex_Map>,) {
    let hex_map = tiles.get_single().expect("despawn map");
    for (plant, parent) in &to_despawn {
        if plant.get_lifetime() > MAX_TIME {
            commands.entity(parent.get()).despawn_recursive();
        }
        if(!hex_map.tiles[plant.get_tile()[0] as usize][plant.get_tile()[1] as usize].is_planted)
        {
            commands.entity(parent.get()).despawn_recursive();
        }
    }
}

pub fn grow_plant(
    mut commands: Commands,
    mut scene_plant: Query<(&mut Handle<Scene>, &Children)>,
    mut plant_components: Query<&mut Plant>,
    time: Res<Time>,
    server: Res<AssetServer>,
) {
    for (mut mesh, plant) in scene_plant.iter_mut() {
        for (plant_id) in plant.iter() {
            let plant_component = plant_components.get_mut(*plant_id);
            match plant_component {
                Ok(mut p) => {
                    if p.update {
                        *mesh = server.load(getPlantPath(p.get_type(), p.get_lifetime()));
                        p.update = false;
                    }
                }
                Err(_) => (),
            }
        }
    }
}
