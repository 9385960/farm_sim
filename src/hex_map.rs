use std::f32::consts::PI;

use crate::hex_outline::{self, add_outline};
use bevy::{
    prelude::{shape::Quad, *},
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};

use self::{
    hex_tile::{Hex, INNER_RADIUS, OUTER_RADIUS, SQRT_3_OVER_2},
    vector::{generate_hex_normals, vec_addition},
};

pub mod hex_tile;
pub mod vector;

pub const ROWS: u32 = 5;
pub const COLMUNS: u32 = 10;
const THICKNESS: f32 = 0.1;

const EDGES: [[f32; 3]; 6] = [
    [-INNER_RADIUS, 0.0, 0.0],
    [-INNER_RADIUS * 0.5, 0.0, INNER_RADIUS * SQRT_3_OVER_2],
    [INNER_RADIUS * 0.5, 0.0, INNER_RADIUS * SQRT_3_OVER_2],
    [INNER_RADIUS, 0.0, 0.0],
    [INNER_RADIUS * 0.5, 0.0, -INNER_RADIUS * SQRT_3_OVER_2],
    [-INNER_RADIUS * 0.5, 0.0, -INNER_RADIUS * SQRT_3_OVER_2],
];

#[derive(Component)]
pub struct Hex_Map {
    mesh: Mesh,
    pub tiles: Vec<Vec<Hex>>,
}

#[derive(Bundle)]
struct Map_Bundle {
    hex_map: Hex_Map,
}

impl Hex_Map {
    pub fn new(t: Vec<Vec<Hex>>, m: Mesh) -> Hex_Map {
        Hex_Map { mesh: m, tiles: t }
    }
}

fn create_tiles() -> Hex_Map {
    let mut verts = Vec::new();
    let mut map = Vec::new();
    for i in 0..ROWS {
        let mut row = Vec::new();
        for j in 0..COLMUNS {
            let mut hex = Hex::new();
            let mut position = [0.0, 0.0, 0.0];
            let x = i as i32;
            let z = j as i32;
            position[0] = (((x - z / 2) as f32) + (z as f32) * 0.5) * INNER_RADIUS * 2.0;
            position[2] = (j as f32) * OUTER_RADIUS * 1.5;
            hex.set_center(position);
            verts.append(&mut hex.get_verts());
            row.push(hex);
        }
        map.push(row);
    }
    let mut hex_mesh = Mesh::new(PrimitiveTopology::TriangleList);
    let mut indices = Vec::new();
    for i in 0..verts.len() {
        indices.push(i as u32);
    }
    let normals = generate_hex_normals(&verts);
    hex_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, verts);
    hex_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    hex_mesh.set_indices(Some(Indices::U32(indices)));

    let hex_map = Hex_Map {
        mesh: hex_mesh,
        tiles: map,
    };
    hex_map
}

pub fn add_hex_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let hex_map = create_tiles();

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(hex_map.mesh),
        material: materials.add(Color::rgb(0.398, 0.2, 0.0).into()),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });

    commands.spawn_bundle(Map_Bundle {
        hex_map: create_tiles(),
    });

    let map = create_tiles();

    for i in 0..map.tiles.len() {
        for j in 0..map.tiles[0].len() {
            for k in 0..6 {
                let position = vec_addition(map.tiles[i][j].get_center(), EDGES[k]);
                commands.spawn_bundle(PbrBundle {
                    mesh: meshes.add(Mesh::from(create_quad())),
                    material: materials.add(StandardMaterial {
                        base_color: Color::rgb(0.0, 0.0, 0.0),
                        alpha_mode: AlphaMode::Blend,
                        unlit: true,
                        ..default()
                    }),
                    transform: Transform {
                        translation: Vec3::new(position[0], position[1] + 0.01, position[2]),
                        rotation: Quat::from_euler(
                            EulerRot::XYZ,
                            (PI / 2.0),
                            PI,
                            PI / 2.0 + (PI / 3.0) * (k as f32),
                        ),
                        scale: Vec3::new(1.0, 1.0, 1.0),
                    },
                    ..default()
                });
            }
        }
    }
}

fn create_quad() -> Quad {
    shape::Quad::new(Vec2::new(OUTER_RADIUS, THICKNESS))
}
