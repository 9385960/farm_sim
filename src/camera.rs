use std::f32::consts::PI;

use bevy::{prelude::*, render::camera::ScalingMode};

pub fn add_camera(mut commands: Commands) {
    let proj = PerspectiveProjection {
        //scale: 7.0,
        //scaling_mode: ScalingMode::FixedVertical(2.0),
        ..default()
    };

    let bundle = Camera3dBundle {
        projection: proj.into(),
        transform: Transform {
            translation: Vec3 {
                x: 0.0,
                y: 4.0,
                z: 2.0,
            },
            rotation: Quat::from_euler(EulerRot::XYZ, -PI / 2.0, 0.0, 0.0),
            scale: Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
        },
        ..Default::default()
    };
    commands.spawn_bundle(bundle);
}
