use bevy::prelude::*;
use self::hex_tile::{Hex, get_hex_verts_c};

mod hex_tile;

const ROWS : u32 = 5;
const COLMUNS : u32 = 5;

#[derive(Component)]
struct Hex_Map{
    mesh:Mesh,
    tiles:Vec<Vec<Hex>>,
}

impl Hex_Map{
    pub fn new(t:Vec<Vec<Hex>>,m:Mesh) -> Hex_Map
    {
        Hex_Map { 
            mesh: m, 
            tiles: t 
        }
    }
}

fn create_tiles()
{
    let mut verts = Vec::new();
    let mut map = Vec::new();
    for i in 0..ROWS
    {
        let mut row = Vec::new();
        for j in 0..COLMUNS
        {
            let hex = Hex::new();
            verts.push(get_hex_verts_c(hex.get_center()));
            row.push(hex);
        }
        map.push(row);
    }
}
