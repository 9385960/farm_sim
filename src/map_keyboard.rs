use crate::hex_map::{COLMUNS, ROWS};
use bevy::prelude::*;

#[derive(Component, Clone, Copy)]
pub struct Position {
    x: u32,
    y: u32,
}

impl Position {
    fn new() -> Position {
        Position { x: 0, y: 0 }
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
) {
    let mut currentPosition = currentPosition.get_single_mut().expect("dfajlkasd");

    if input.just_pressed(KeyCode::Left) {
        if (currentPosition.x > 0 && currentPosition.x <= ROWS) {
            currentPosition.x -= 1;
        }
        println!("L");
        println!("{} {}", currentPosition.x, currentPosition.y)
    }
    if input.just_pressed(KeyCode::Right) {
        if (currentPosition.x >= 0 && currentPosition.x < ROWS) {
            currentPosition.x += 1;
        }
        println!("R");
        println!("{} {}", currentPosition.x, currentPosition.y)
    }
    if input.just_pressed(KeyCode::Up) {
        if (currentPosition.y >= 0 && currentPosition.y < COLMUNS) {
            currentPosition.y += 1;
        }
        println!("U");
        println!("{} {}", currentPosition.x, currentPosition.y)
    }
    if input.just_pressed(KeyCode::Down) {
        if (currentPosition.y > 0 && currentPosition.y <= COLMUNS) {
            currentPosition.y -= 1;
        }
        println!("D");
        println!("{} {}", currentPosition.x, currentPosition.y)
    }
}
