use std::f32::consts::PI;

use bevy::prelude::*;

pub fn add_light_dir(mut commands: Commands) {
    commands.spawn_bundle(DirectionalLightBundle {
        transform: Transform {
            translation: Vec3 {
                x: 0.0,
                y: 8.0,
                z: 0.0,
            },
            rotation: Quat::from_euler(EulerRot::XYZ, -PI / 1.5, 0.0, 0.0),
            scale: Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
        },
        ..default()
    });
}
