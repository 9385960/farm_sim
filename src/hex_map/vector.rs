
pub fn generate_hex_normals(verts: &Vec<[f32;3]>) -> Vec<[f32;3]>
{
    if verts.len()%3 != 0
    {
        println!("Verts needs to be a multiple of three");
        return Vec::new()
    }
    let mut normals = Vec::new();
    for i in (0..verts.len()).step_by(3)
    {
        let v1 = vec_difference(verts[i+1], verts[i]);
        let v2 = vec_difference(verts[i+2], verts[i]);
        let v1xv2 = normalize(cross_product(v1, v2));
        normals.push(v1xv2);
        normals.push(v1xv2);
        normals.push(v1xv2);
        println!("Normal for triangle {}, x: {}, y: {} , z: {}.",i/3,v1xv2[0],v1xv2[1],v1xv2[2])
    }
    normals
}

fn vec_difference(v1 : [f32; 3], v2: [f32;3]) -> [f32;3]
{
    [v1[0]-v2[0],v1[1]-v2[1],v1[2]-v2[2]]
}

fn normalize(v : [f32;3]) -> [f32;3]
{
    let magnitude = (v[0].powi(2) + v[1].powi(2) + v[2].powi(2)).sqrt();
    [v[0]/magnitude,v[1]/magnitude,v[2]/magnitude]
}

fn cross_product(v1 : [f32; 3], v2: [f32;3]) -> [f32;3]
{
    let x = [v1[1]*v2[2]-v1[2]*v2[1],v1[2]*v2[0]-v1[0]*v2[2],v1[0]*v2[1]-v1[1]*v2[0]];
    x
}

pub fn vec_addition(v1 : [f32; 3], v2: [f32;3]) -> [f32;3]
{
    [v1[0]+v2[0],v1[1]+v2[1],v1[2]+v2[2]]
}

fn vector_multiplication(v : [f32;3], c : f32) -> [f32;3]
{
    [v[0]*c,v[1]*c,v[2]*c]
}

fn vector_division(v : [f32;3], c : f32) -> [f32;3]
{
    [v[0]/c,v[1]/c,v[2]/c]
}