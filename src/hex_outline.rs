use bevy::prelude::{shape::Quad, *};

use crate::hex_map::{
    hex_tile::{INNER_RADIUS, OUTER_RADIUS},
    Hex_Map,
};

const THICKNESS: f32 = 0.1;

fn create_quad() -> Quad {
    shape::Quad::new(Vec2::new(OUTER_RADIUS, THICKNESS))
}

pub fn add_outline(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    hex_map: &Hex_Map,
) {
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(create_quad())),
        material: materials.add(StandardMaterial {
            base_color: Color::rgb(0.0, 0.0, 0.0),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        }),
        transform: Transform {
            translation: Vec3::new(-INNER_RADIUS, 0.1, 0.0),
            rotation: Quat::from_euler(EulerRot::XYZ, 1.570796327, 3.1415369265, 1.570796327),
            scale: Vec3::new(1.0, 1.0, 1.0),
        },
        ..default()
    });
}
