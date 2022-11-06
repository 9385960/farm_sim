use bevy::prelude::*;
use crate::hex_map::hex_tile::Hex;

#[derive(Component)]
pub struct Machine {
    pub speed: f32,
    machine_type: Model,
    tiles: Vec<([f32;3],[u32;2])>,
    pub hex_location : [u32;2],
    destination: [f32;3],
}

#[derive(Bundle)]
pub struct MachineBundle{
    pub machine : Machine,
}


pub enum Model {
    //Fertilizer,
    Plow,
    //Planter,
    //Harvester,
}

impl Machine{
    pub fn new(vehicle: Model, s : f32, hex_init : [u32;2]) -> Machine
    {
        Machine{
            machine_type : vehicle,
            speed : s,
            destination : [0.0,0.0,0.0],
            hex_location : hex_init,
            tiles : Vec::new(),
        }
    }

    pub fn add_hex(&mut self,tile:([f32;3],[u32;2]))
    {
        self.tiles.push(tile)
    }

    pub fn get_destination(&self) -> [f32;3]
    {
        self.destination
    }

    pub fn get_hex_location(&self) -> [u32;2]
    {
        self.hex_location
    }

    pub fn update_destination(&mut self)
    {
        if self.tiles.len() > 0
        {
            self.destination = self.tiles[0].0;
            self.hex_location = self.tiles[0].1;
            self.tiles.remove(0);
        }
    }
}