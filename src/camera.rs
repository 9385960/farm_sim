use std::f32::consts::PI;

use bevy::{prelude::*, render::camera::ScalingMode};

pub fn add_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera3dBundle {
        projection: OrthographicProjection {
            scale: 3.0,
            scaling_mode: ScalingMode::FixedVertical(2.0),
            ..default()
        }
        .into(),
        transform: Transform{
            translation: Vec3 { x: 0.0, y: 4.0, z: 0.0 },
            rotation: Quat::from_euler(EulerRot::XYZ, -PI/2.0,0.0, 0.0),
            scale: Vec3 { x: 2.5, y: 2.5, z: 2.5},
        },
        ..Default::default()
    });
}
