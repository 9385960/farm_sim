use bevy::prelude::*;

pub const MAX_TIME: f32 = 30.0;
pub const WHEAT_GROWTH_TIME: [f32; 4] = [5.0, 10.0, 15.0, 20.0];

#[derive(Component)]
pub struct Plant {
    lifetime: f32,
    plant_type: Type,
    nourishment: f32,
    tile: [u32; 2],
    pub update: bool,
    stage: usize,
}

#[derive(Debug)]
pub enum Type {
    Wheat,
    // Carrot,
    // Beet,
    // Potato,
    // Berries,
}

#[derive(Bundle)]
pub struct PlantBundle {
    pub bundle: Plant,
}

impl Plant {
    pub fn new(plant_type: Type, tile: [u32; 2]) -> Plant {
        Plant {
            lifetime: 0.0,
            plant_type,
            nourishment: 0.0,
            tile,
            update: false,
            stage: 0,
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
        if self.stage < 4 {
            if (self.lifetime > WHEAT_GROWTH_TIME[self.stage]) {
                self.update = true;
                self.stage += 1;
            }
        }
    }

    pub fn set_tile(&mut self, t: [u32; 2]) {
        self.tile = t;
    }

    pub fn get_nourishment(&self) -> f32 {
        self.nourishment
    }

    pub fn get_lifetime(&self) -> f32 {
        self.lifetime
    }

    pub fn get_tile(&self) -> [u32; 2] {
        self.tile
    }

    pub fn get_type(&self) -> &Type {
        &self.plant_type
    }

    pub fn updated(&mut self) {
        self.update = false;
    }
}

pub fn getPlantPath(plant: &Type, life: f32) -> &str {
    match Type::Wheat {
        Wheat => {
            if (life > WHEAT_GROWTH_TIME[3]) {
                return "dead_wheat.gltf#Scene0";
            } else if (life > WHEAT_GROWTH_TIME[2]) {
                return "ripe_wheat.gltf#Scene0";
            } else if (life > WHEAT_GROWTH_TIME[1]) {
                return "flowering_wheat.gltf#Scene0";
            } else if (life > WHEAT_GROWTH_TIME[0]) {
                return "sprouts.gltf#Scene0";
            } else {
                return "";
            }
        }
    }
}
