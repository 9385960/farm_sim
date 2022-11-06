use crate::hex_map::{COLMUNS, ROWS, hex_tile::{OUTER_RADIUS, INNER_RADIUS}};
use bevy::{prelude::*, render::camera};

#[derive(Component, Clone, Copy)]
pub struct Position {
    x: u32,
    z: u32,
}

impl Position {
    fn new() -> Position {
        Position { x: 0, z: 0 }
    }
}

#[derive(Bundle)]
pub struct PositionBundle {
    bundle: Position,
}

pub fn initalize_position(mut commands: Commands) {
    let mut currentPosition = Position::new();
    commands.spawn_bundle(PositionBundle {
        bundle: currentPosition,
    });
}

pub fn add_position(
    mut input: Res<Input<KeyCode>>,
    mut commands: Commands,
    mut currentPosition: Query<&mut Position>,
    mut camera: Query<&mut Transform, With<camera::Camera>>,
) {
    let mut currentPosition = currentPosition.get_single_mut().expect("dfajlkasd");

    if input.just_pressed(KeyCode::Left) {
        if (currentPosition.x > 0 && currentPosition.x <= ROWS) {
            currentPosition.x -= 1;
            let mut cam_position = camera.get_single_mut().expect("dfajlkasd");
            let new_position = get_world_from_hex(currentPosition.x, currentPosition.z);
            cam_position.translation = Vec3::new(new_position[0],cam_position.translation.y,new_position[2]);
        }
        println!("L");
        println!("{} {}", currentPosition.x, currentPosition.z)
    }
    if input.just_pressed(KeyCode::Right) {
        if (currentPosition.x >= 0 && currentPosition.x < ROWS) {
            currentPosition.x += 1;
            let mut cam_position = camera.get_single_mut().expect("dfajlkasd");
            let new_position = get_world_from_hex(currentPosition.x, currentPosition.z);
            cam_position.translation = Vec3::new(new_position[0],cam_position.translation.y,new_position[2]);
        }
        println!("R");
        println!("{} {}", currentPosition.x, currentPosition.z)
    }
    if input.just_pressed(KeyCode::Up) {
        if (currentPosition.z >= 0 && currentPosition.z < COLMUNS) {
            currentPosition.z += 1;
            let mut cam_position = camera.get_single_mut().expect("dfajlkasd");
            let new_position = get_world_from_hex(currentPosition.x, currentPosition.z);
            cam_position.translation = Vec3::new(new_position[0],cam_position.translation.y,new_position[2]);
        }
        println!("U");
        println!("{} {}", currentPosition.x, currentPosition.z)
    }
    if input.just_pressed(KeyCode::Down) {
        if (currentPosition.z > 0 && currentPosition.z <= COLMUNS) {
            currentPosition.z -= 1;
            let mut cam_position = camera.get_single_mut().expect("dfajlkasd");
            let new_position = get_world_from_hex(currentPosition.x, currentPosition.z);
            cam_position.translation = Vec3::new(new_position[0],cam_position.translation.y,new_position[2]);
        }
        println!("D");
        println!("{} {}", currentPosition.x, currentPosition.z)
    }
}

fn get_world_from_hex(x:u32,z:u32) -> [f32;3]
{
    let mut position = [0.0, 0.0, 0.0];
    let x = x as i32;
    let z = z as i32;
    position[0] = (((x - z / 2) as f32) + (z as f32) * 0.5) * INNER_RADIUS * 2.0;
    position[2] = (z as f32) * OUTER_RADIUS * 1.5;
    position
}