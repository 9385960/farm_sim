use bevy::{prelude::*, render::{mesh::Indices, render_resource::PrimitiveTopology}};
use self::{hex_tile::{Hex, INNER_RADIUS, OUTER_RADIUS}, vector::generate_hex_normals};

mod vector;
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
            let mut hex = Hex::new();
            let position = [(i as f32) * INNER_RADIUS * 2.0, 0.0 ,(j as f32) * OUTER_RADIUS * 1.5];
            hex.set_center(position);
            verts.append(&mut hex.get_verts());
            row.push(hex);
        }
        map.push(row);
    }
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    let mut indices = Vec::new();
    for i in 0..verts.len()
    {
        indices.push(i as u32);
    }
    let normals = generate_hex_normals(&verts);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, verts);
    mesh.set_indices(Some(Indices::U32(indices)));
}

