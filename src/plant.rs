use bevy::prelude::*;

#[derive(Component)]
pub struct Plant {
    lifetime: f32,
    plant_type: Type,
    nourishment: f32,
    tile: Vec2,
}

#[derive(Debug)]
enum Type {
    Wheat,
    Carrot,
    Beet,
    Potato,
    Berries,
}

impl Plant {
    pub fn new(plant_type: Type, tile: Vec2) -> Plant {
        Plant {
            lifetime: 0.0,
            plant_type,
            nourishment: 0.0,
            tile,
        }
    }

    pub fn set_nourishment(&mut self, n: f32) {
        self.nourishment = n;
    }

    pub fn set_lifetime(&mut self, l: f32) {
        self.lifetime = l;
    }

    pub fn increase_lifetime(&mut self, time: f32) {
        self.lifetime += time;
    }

    pub fn set_tile(&mut self, t: Vec2) {
        self.tile = t;
    }

    pub fn get_nourishment(&self) -> f32 {
        self.nourishment
    }

    pub fn get_lifetime(&self) -> f32 {
        self.lifetime
    }

    pub fn get_tile(&self) -> Vec2 {
        self.tile
    }

    pub fn get_type(&self) -> &Type {
        &self.plant_type
    }
}
