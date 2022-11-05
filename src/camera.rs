use bevy::{prelude::*, render::camera::ScalingMode};

pub fn add_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera3dBundle {
        projection: OrthographicProjection {
            scale: 3.0,
            scaling_mode: ScalingMode::FixedVertical(2.0),
            ..default()
        }
        .into(),
        transform: Transform::from_xyz(1.0, 4.0, 0.0).looking_at(Vec3::ZERO, -Vec3::Y),
        ..Default::default()
    });
}
