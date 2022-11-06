use crate::hex_map::{COLMUNS, ROWS};
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
            camera.get_single_mut().expect("dfajlkasd").translation.x -= 1.0;
        }
        println!("L");
        println!("{} {}", currentPosition.x, currentPosition.z)
    }
    if input.just_pressed(KeyCode::Right) {
        if (currentPosition.x >= 0 && currentPosition.x < ROWS) {
            currentPosition.x += 1;
            camera.get_single_mut().expect("dfajlkasd").translation.x += 1.0;
        }
        println!("R");
        println!("{} {}", currentPosition.x, currentPosition.z)
    }
    if input.just_pressed(KeyCode::Up) {
        if (currentPosition.z >= 0 && currentPosition.z < COLMUNS) {
            currentPosition.z += 1;
            camera.get_single_mut().expect("dfajlkasd").translation.z -= 1.0;
        }
        println!("U");
        println!("{} {}", currentPosition.x, currentPosition.z)
    }
    if input.just_pressed(KeyCode::Down) {
        if (currentPosition.z > 0 && currentPosition.z <= COLMUNS) {
            currentPosition.z -= 1;
            camera.get_single_mut().expect("dfajlkasd").translation.z += 1.0;
        }
        println!("D");
        println!("{} {}", currentPosition.x, currentPosition.z)
    }
}
