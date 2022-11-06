use crate::hex_map::hex_tile::{INNER_RADIUS, OUTER_RADIUS};

pub fn get_world_from_hex(x: u32, z: u32) -> [f32; 3] {
    let mut position = [0.0, 0.0, 0.0];
    let x = x as i32;
    let z = z as i32;
    position[0] = (((x - z / 2) as f32) + (z as f32) * 0.5) * INNER_RADIUS * 2.0;
    position[2] = (z as f32) * OUTER_RADIUS * 1.5;
    position
}
