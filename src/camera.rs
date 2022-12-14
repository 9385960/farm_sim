

use bevy::{prelude::*};

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
                y: 3.0,
                z: 3.0,
            },
            rotation: Quat::from_euler(EulerRot::XYZ, -0.7, 0.0, 0.0),
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
