use self::hex_tile::Hex;
use bevy::prelude::*;

#[derive(Component, Debug)]
struct Plant {
    lifetime: u16,
    plant_type: Type,
    nourishment: f32,
    tile: Hex,
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
    fn new(plant_type: Type, tile: Hex) -> Plant {
        Plant {
            lifetime: 0,
            plant_type,
            nourishment: 0.0,
            tile,
        }
    }

    fn set_nourishment(&mut self, n: f32) {
        self.nourishment = n;
    }

    fn set_lifetime(&mut self, l: u16) {
        self.lifetime = l;
    }

    fn set_tile(&mut self, t: Hex) {
        self.tile = t;
    }

    fn get_nourishment(&self) -> f32 {
        self.nourishment
    }

    fn get_lifetime(&self) -> u16 {
        self.lifetime
    }

    fn get_tile(&self) -> Hex {
        self.tile
    }

    fn get_type(&self) -> Type {
        self.plant_type
    }
}
