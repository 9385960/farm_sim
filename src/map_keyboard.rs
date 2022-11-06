use crate::{
    hex_map::{COLMUNS, ROWS},
    util::get_world_from_hex,
};
use bevy::{input::mouse::MouseScrollUnit, input::mouse::MouseWheel, prelude::*, render::camera};

const Z_OFFSET: f32 = 3.0;

#[derive(Component, Clone, Copy)]
pub struct Position {
    pub x: u32,
    pub z: u32,
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
    let current_position = Position::new();
    commands.spawn_bundle(PositionBundle {
        bundle: current_position,
    });
}

pub fn add_position(
    input: Res<Input<KeyCode>>,
    mut scroll: EventReader<MouseWheel>,
    mut current_position: Query<&mut Position>,
    mut camera: Query<&mut Transform, With<camera::Camera>>,
) {
    let mut current_position = current_position.get_single_mut().expect("dfajlkasd");

    if input.just_pressed(KeyCode::Left) {
        if current_position.x > 0 && current_position.x <= ROWS {
            current_position.x -= 1;
            let mut cam_position = camera.get_single_mut().expect("dfajlkasd");
            let new_position = get_world_from_hex(current_position.x, current_position.z);
            cam_position.translation = Vec3::new(
                new_position[0],
                cam_position.translation.y,
                new_position[2] + Z_OFFSET,
            );
        }
    }
    if input.just_pressed(KeyCode::Right) {
        if current_position.x >= 0 && current_position.x < ROWS - 1 {
            current_position.x += 1;
            let mut cam_position = camera.get_single_mut().expect("dfajlkasd");
            let new_position = get_world_from_hex(current_position.x, current_position.z);
            cam_position.translation = Vec3::new(
                new_position[0],
                cam_position.translation.y,
                new_position[2] + Z_OFFSET,
            );
        }
    }
    if input.just_pressed(KeyCode::Down) {
        if current_position.z >= 0 && current_position.z < COLMUNS - 1 {
            current_position.z += 1;
            let mut cam_position = camera.get_single_mut().expect("dfajlkasd");
            let new_position = get_world_from_hex(current_position.x, current_position.z);
            cam_position.translation = Vec3::new(
                new_position[0],
                cam_position.translation.y,
                new_position[2] + Z_OFFSET,
            );
        }
    }
    if input.just_pressed(KeyCode::Up) {
        if current_position.z > 0 && current_position.z <= COLMUNS {
            current_position.z -= 1;
            let mut cam_position = camera.get_single_mut().expect("dfajlkasd");
            let new_position = get_world_from_hex(current_position.x, current_position.z);
            cam_position.translation = Vec3::new(
                new_position[0],
                cam_position.translation.y,
                new_position[2] + Z_OFFSET,
            );
        }

    }
    for ev in scroll.iter() {
        match ev.unit {
            MouseScrollUnit::Line => {

                let mut cam_position = camera.get_single_mut().expect("dfajlkasd");

                const LOWER_BOUND: f32 = 3.0;
                const UPPER_BOUND: f32 = 6.0;

                if ev.y > 0.0
                    && cam_position.translation.y >= LOWER_BOUND
                    && cam_position.translation.y <= UPPER_BOUND
                {
                    cam_position.translation.y -= 0.1;
                } else if ev.y < 0.0
                    && cam_position.translation.y >= LOWER_BOUND
                    && cam_position.translation.y <= UPPER_BOUND
                {
                    cam_position.translation.y += 0.1;
                } else if cam_position.translation.y < UPPER_BOUND {
                    cam_position.translation.y = LOWER_BOUND
                } else if cam_position.translation.y > UPPER_BOUND {
                    cam_position.translation.y = UPPER_BOUND
                }
            }
            MouseScrollUnit::Pixel => {
            }
        }
    }
}
