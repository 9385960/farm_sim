use bevy::prelude::*;

use super::vector::vec_addition;

pub const OUTER_RADIUS: f32 = 1.0;
pub const SQRT_3_OVER_2: f32 = 0.86602540378443864676;
pub const INNER_RADIUS: f32 = OUTER_RADIUS * SQRT_3_OVER_2;

const VERTS: [[f32; 3]; 7] = [
    [0.0, 0.0, OUTER_RADIUS],
    [INNER_RADIUS, 0.0, 0.5 * OUTER_RADIUS],
    [INNER_RADIUS, 0.0, -0.5 * OUTER_RADIUS],
    [0.0, 0.0, -OUTER_RADIUS],
    [-INNER_RADIUS, 0.0, -0.5 * OUTER_RADIUS],
    [-INNER_RADIUS, 0.0, 0.5 * OUTER_RADIUS],
    [0.0, 0.0, OUTER_RADIUS],
];

#[derive(Component)]
pub struct Hex {
    //nutrients: f32,
    //water: f32,
    center: [f32; 3],
    pub index: [u32; 2],
    pub tilled: bool,
    pub is_planted: bool,
}

impl Hex {
    pub fn new(x: u32, z: u32) -> Hex {
        Hex {
            //nutrients: 1.0,
            //water: 1.0,
            center: [0.0, 0.0, 0.0],
            index: [x, z],
            tilled: false,
            is_planted: false,
        }
    }

    // pub fn set_nutrients(&mut self, n: f32) {
    //     self.nutrients = n;
    // }

    // pub fn set_water(&mut self, w: f32) {
    //     self.water = w;
    // }

    pub fn set_center(&mut self, c: [f32; 3]) {
        self.center = c;
    }

    pub fn get_center(&self) -> [f32; 3] {
        self.center
    }

    pub fn get_verts(&self) -> Vec<[f32; 3]> {
        get_hex_verts_c(self.center)
    }

    // pub fn get_nutrients(&self) -> f32 {
    //     self.nutrients
    // }

    // pub fn get_water(&self) -> f32 {
    //     self.water
    // }

    // pub fn add_nutrients(&mut self, n: f32) {
    //     self.nutrients += n;
    // }

    // pub fn add_water(&mut self, w: f32) {
    //     self.water += w;
    // }

    // pub fn remove_nutrients(&mut self, n: f32) {
    //     self.nutrients -= n;
    // }

    // pub fn remove_water(&mut self, w: f32) {
    //     self.water -= w;
    // }

    // pub fn plantable(&self) -> bool {
    //     self.nutrients > 0.0 && self.water > 0.0
    // }
}

// fn get_hex_verts() -> Vec<[f32; 3]> {
//     let mut verts = Vec::new();
//     for i in 0..6 {
//         verts.push(CENTER);
//         verts.push(VERTS[i]);
//         verts.push(VERTS[i + 1]);
//     }
//     verts
// }

fn get_hex_verts_c(center: [f32; 3]) -> Vec<[f32; 3]> {
    let mut verts = Vec::new();
    for i in 0..6 {
        verts.push(center);
        verts.push(vec_addition(VERTS[i], center));
        verts.push(vec_addition(VERTS[i + 1], center));
    }
    verts
}
