use crate::hex_map::{
    hex_tile::{INNER_RADIUS, OUTER_RADIUS},
    COLMUNS, ROWS,
};
use bevy::{input::mouse::MouseScrollUnit, input::mouse::MouseWheel, prelude::*, render::camera};

const Z_OFFSET: f32 = 3.0;

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
    mut scroll: EventReader<MouseWheel>,
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
            cam_position.translation = Vec3::new(
                new_position[0],
                cam_position.translation.y,
                new_position[2] + Z_OFFSET,
            );
        }
        println!("L");
        println!("{} {}", currentPosition.x, currentPosition.z)
    }
    if input.just_pressed(KeyCode::Right) {
        if (currentPosition.x >= 0 && currentPosition.x < ROWS - 1) {
            currentPosition.x += 1;
            let mut cam_position = camera.get_single_mut().expect("dfajlkasd");
            let new_position = get_world_from_hex(currentPosition.x, currentPosition.z);
            cam_position.translation = Vec3::new(
                new_position[0],
                cam_position.translation.y,
                new_position[2] + Z_OFFSET,
            );
        }
        println!("R");
        println!("{} {}", currentPosition.x, currentPosition.z)
    }
    if input.just_pressed(KeyCode::Down) {
        if (currentPosition.z >= 0 && currentPosition.z < COLMUNS - 1) {
            currentPosition.z += 1;
            let mut cam_position = camera.get_single_mut().expect("dfajlkasd");
            let new_position = get_world_from_hex(currentPosition.x, currentPosition.z);
            cam_position.translation = Vec3::new(
                new_position[0],
                cam_position.translation.y,
                new_position[2] + Z_OFFSET,
            );
        }
        println!("D");
        println!("{} {}", currentPosition.x, currentPosition.z)
    }
    if input.just_pressed(KeyCode::Up) {
        if (currentPosition.z > 0 && currentPosition.z <= COLMUNS) {
            currentPosition.z -= 1;
            let mut cam_position = camera.get_single_mut().expect("dfajlkasd");
            let new_position = get_world_from_hex(currentPosition.x, currentPosition.z);
            cam_position.translation = Vec3::new(
                new_position[0],
                cam_position.translation.y,
                new_position[2] + Z_OFFSET,
            );
        }
        println!("U");
        println!("{} {}", currentPosition.x, currentPosition.z)
    }
    for ev in scroll.iter() {
        match ev.unit {
            MouseScrollUnit::Line => {
                println!(
                    "Scroll (line units): vertical: {}, horizontal: {}",
                    ev.y, ev.x
                );

                let mut cam_position = camera.get_single_mut.expect("dfajlkasd");

                if (ev.y > 0.0
                    && cam_position.translation.y >= 3.0
                    && cam_position.translation.y <= 6.0)
                {
                    cam_position.translation.y -= 0.1;
                } else if (ev.y < 0.0
                    && cam_position.translation.y >= 3.0
                    && cam_position.translation.y <= 6.0)
                {
                    cam_position.translation.y += 0.1;
                } else if (cam_position.translation.y < 6.0) {
                    cam_position.translation.y = 6.0
                } else if (cam_position.translation.y > 6.0) {
                    cam_position.translation.y = 6.0
                }
            }
            MouseScrollUnit::Pixel => {
                println!(
                    "Scroll (pixel units): vertical: {}, horizontal: {}",
                    ev.y, ev.x
                );
            }
        }
    }
}

fn get_world_from_hex(x: u32, z: u32) -> [f32; 3] {
    let mut position = [0.0, 0.0, 0.0];
    let x = x as i32;
    let z = z as i32;
    position[0] = (((x - z / 2) as f32) + (z as f32) * 0.5) * INNER_RADIUS * 2.0;
    position[2] = (z as f32) * OUTER_RADIUS * 1.5;
    position
}
