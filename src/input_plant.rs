use bevy::prelude::*;

use crate::{
    hex_map::hex_tile::{INNER_RADIUS, OUTER_RADIUS},
    map_keyboard::Position,
    plant::Plant,
};

pub fn add_plant(
    mut commands: Commands,
    mut input: Res<Input<KeyCode>>,
    mut selected_location: Position,
    mut plant_components: Query<&mut Plant>,
    time: Res<Time>,
    server: Res<AssetServer>,
) {
    let mut canPlant = true;
    for mut plant in plant_components.iter_mut() {
        plant.increase_lifetime(time.delta_seconds());
        if (plant.get_tile()[0] == selected_location.x
            && plant.get_tile()[1] == selected_location.z)
        {
            canPlant = false;
        }
    }

    if input.just_pressed(KeyCode::Space) && canPlant {
        println!("test");
        let seedlings_gltf = server.load("seedlings.gltf#Scene0");
        let spawn_location = get_world_from_hex(selected_location.x, selected_location.z);

        commands.spawn_bundle(SceneBundle {
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
        });
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
