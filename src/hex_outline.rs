use std::f32::consts::PI;

use bevy::prelude::{shape::Quad, *};

use crate::hex_map::{
    hex_tile::{INNER_RADIUS, OUTER_RADIUS, SQRT_3_OVER_2},
    Hex_Map, vector::vec_addition,
};

const EDGES: [[f32; 3]; 6] = [
    [-INNER_RADIUS, 0.0, 0.0],
    [-INNER_RADIUS*0.5, 0.0, INNER_RADIUS*SQRT_3_OVER_2],
    [INNER_RADIUS*0.5, 0.0, INNER_RADIUS*SQRT_3_OVER_2],
    [INNER_RADIUS, 0.0, 0.0],
    [INNER_RADIUS*0.5, 0.0, -INNER_RADIUS*SQRT_3_OVER_2],
    [-INNER_RADIUS*0.5, 0.0, -INNER_RADIUS*SQRT_3_OVER_2],
];

const THICKNESS: f32 = 0.1;

fn create_quad() -> Quad {
    shape::Quad::new(Vec2::new(OUTER_RADIUS, THICKNESS))
}

pub fn add_outline(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    hex_map: Query<&Hex_Map>,
) {
    let map = hex_map.get_single().expect("Help");
    for i in 0..map.tiles.len()
    {
        for j in 0..map.tiles[0].len()
        {
            for k in 0..6
            {
                let position = vec_addition(map.tiles[i][j].get_center(),EDGES[k]);
                commands.spawn_bundle(PbrBundle {
                    mesh: meshes.add(Mesh::from(create_quad())),
                    material: materials.add(StandardMaterial {
                        base_color: Color::rgb(0.0, 0.0, 0.0),
                        alpha_mode: AlphaMode::Blend,
                        unlit: true,
                        ..default()
                    }),
                    transform: Transform {
                        translation: Vec3::new(position[0],position[1],position[2]),
                        rotation: Quat::from_euler(EulerRot::XYZ, (PI/2.0)-(PI*(k as f32)/3.0), 0.0, PI/2.0),
                        scale: Vec3::new(1.0, 1.0, 1.0),
                    },
                    ..default()
                });
            }
        }
    }
}
