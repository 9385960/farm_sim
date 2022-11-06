use bevy::prelude::*;

#[derive(Component)]
struct Plant {
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
    fn new(plant_type: Type, tile: Vec2) -> Plant {
        Plant {
            lifetime: 0.0,
            plant_type,
            nourishment: 0.0,
            tile,
        }
    }

    fn set_nourishment(&mut self, n: f32) {
        self.nourishment = n;
    }

    fn set_lifetime(&mut self, l: f32) {
        self.lifetime = l;
    }

    fn increase_lifetime(&mut self, time: f32)
    {
        self.lifetime += time;
    }

    fn set_tile(&mut self, t: Vec2) {
        self.tile = t;
    }

    fn get_nourishment(&self) -> f32 {
        self.nourishment
    }

    fn get_lifetime(&self) -> f32 {
        self.lifetime
    }

    fn get_tile(&self) -> Vec2 {
        self.tile
    }

    fn get_type(&self) -> &Type {
        &self.plant_type
    }
}
