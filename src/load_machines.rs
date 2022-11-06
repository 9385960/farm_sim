use bevy::prelude::*;

use crate::machine::{Machine, MachineBundle, Model};

pub fn load_plow(mut commands: Commands, server: Res<AssetServer>) {
    let plow_gltf = server.load("plow.gltf#Scene0");

    let machine = commands
        .spawn_bundle(MachineBundle {
            machine: Machine::new(Model::Plow, 1.0, [0, 0]),
        })
        .id();

    let machine_location = commands
        .spawn_bundle(SceneBundle {
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
        })
        .id();

    commands.entity(machine_location).push_children(&[machine]);
}

pub fn load_harvester(mut commands: Commands, server: Res<AssetServer>) {
    let plow_gltf = server.load("harvester.gltf#Scene0");

    let machine = commands.spawn_bundle(MachineBundle{
        machine: Machine::new(Model::Harvester,1.0,[0,0]),
    }).id();

    let machine_location = commands
        .spawn_bundle(SceneBundle {
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
        })
        .id();

    commands.entity(machine_location).push_children(&[machine]);
}
