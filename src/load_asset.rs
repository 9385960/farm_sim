use bevy::prelude::*;

pub fn load_asset(mut commands: Commands, server: Res<AssetServer>) {
    let plow_gltf = server.load("plow.gltf#Scene0");
    let ripe_wheat_gltf = server.load("ripe_wheat.gltf#Scene0");

    commands.spawn_bundle(SceneBundle {
        scene: plow_gltf,
        transform: Transform {
            translation: Vec3 {
                x: 0.0,
                y: 0.05,
                z: 0.0,
            },
            rotation: Quat::IDENTITY,
            scale: Vec3 {
                x: 0.06,
                y: 0.06,
                z: 0.06,
            },
        },
        ..Default::default()
    });

    commands.spawn_bundle(SceneBundle {
        scene: ripe_wheat_gltf,
        transform: Transform {
            translation: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            rotation: Quat::IDENTITY,
            scale: Vec3 {
                x: 0.9,
                y: 0.9,
                z: 0.9,
            },
        },
        ..Default::default()
    });
}
