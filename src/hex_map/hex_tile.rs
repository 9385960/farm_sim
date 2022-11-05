use bevy::prelude::*;

const OUTER_RADIUS: f32 = 1.0;
const SQRT_3_OVER_2: f32 = 0.86602540378443864676;
const INNER_RADIUS: f32 = OUTER_RADIUS * SQRT_3_OVER_2;

const CENTER: [f32; 3] = [0.0, 0.0, 0.0];

#[derive(Component)]
struct Hex {
    nutrients: f32,
    water: f32,
}

const VERTS: [[f32; 3]; 7] = [
    [0.0, 0.0, OUTER_RADIUS],
    [INNER_RADIUS, 0.0, 0.5 * OUTER_RADIUS],
    [INNER_RADIUS, 0.0, -0.5 * OUTER_RADIUS],
    [0.0, 0.0, -OUTER_RADIUS],
    [-INNER_RADIUS, 0.0, -0.5 * OUTER_RADIUS],
    [-INNER_RADIUS, 0.0, 0.5 * OUTER_RADIUS],
    [0.0, 0.0, OUTER_RADIUS],
];

fn get_hex_verts() -> Vec<[f32; 3]> {
    let mut verts = Vec::new();
    for i in 0..6 {
        verts.push(CENTER);
        verts.push(VERTS[i]);
        verts.push(VERTS[i + 1]);
    }
    verts
}

fn get_hex_verts_c(center: [f32; 3]) -> Vec<[f32; 3]> {
    let mut verts = Vec::new();
    for i in 0..6 {
        verts.push(center);
        verts.push(VERTS[i]);
        verts.push(VERTS[i + 1]);
    }
    verts
}
