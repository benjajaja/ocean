use bevy::{
    prelude::*,
    render::{
        mesh::{Indices,VertexAttributeValues},
        pipeline::{PrimitiveTopology},
    },
};
use rand::prelude::*;

fn plane(size: u32) -> Mesh {
    const CBRT3: f32 = 1.44224957031; // cubic root of 3
    fn normal() -> [f32; 3] {
        [0.0, 1.0, 0.1]
    }
    let mut rng = rand::thread_rng();

    let mut vertices: Vec<([f32; 3], [f32; 3], [f32; 2])> = vec![];
    let mut tri_indices: Vec<u32> = vec![];
    let mut vertex_colors: Vec<[f32; 3]> = vec![];
    for y in 0..size {
        let offset_y = y as f32 * CBRT3;
        let index_offset_y = y * size;

        if y == 0 {
            for x in 0..(size + 2) {
                let offset_x = x as f32;
                if x % 2 == 0 {
                    vertices.push(([offset_x * 0.5, 0., offset_y + CBRT3], normal(), [1., 0.]));
                    if x > 1 {
                        tri_indices.append(&mut vec![index_offset_y + x, index_offset_y + x - 1, index_offset_y + x - 2]);
                    }
                } else {
                    vertices.push(([offset_x * 0.5, 0., offset_y], normal(), [1., 0.]));
                    if x > 1 {
                        tri_indices.append(&mut vec![index_offset_y + x, index_offset_y + x - 2, index_offset_y + x - 1]);
                    }
                }
                vertex_colors.push([rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>()]);
            }
        }
    }

    println!("{:#?}", tri_indices);
    let indices = Indices::U32(tri_indices);

    let mut positions: Vec<[f32; 3]> = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    for (position, normal, uv) in vertices.iter() {
        positions.push(*position);
        normals.push(*normal);
        uvs.push(*uv);
    }

    println!("{:#?}", positions);

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.set_indices(Some(indices));


    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, VertexAttributeValues::from(positions));
    mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, VertexAttributeValues::from(normals));
    mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, VertexAttributeValues::from(uvs));

    mesh.set_attribute("Vertex_Color", VertexAttributeValues::from(vertex_colors));

    mesh
}

const STAR_DISTANCE: f32 = 1000.;
pub fn stars() -> Mesh {
    const CBRT3: f32 = 1.44224957031; // cubic root of 3
    fn normal() -> [f32; 3] {
        [0.0, 1.0, 0.1]
    }
    let size = 10.;

    let mut vertices: Vec<([f32; 3], [f32; 3], [f32; 2])> = vec![];
    let mut tri_indices: Vec<u32> = vec![];

    let h = size / 2. * CBRT3;
    let offset = h / 3.;

    let mut rng = rand::thread_rng();
    let star_points = vec![
        Vec3::new(-0.5 * size, 0., STAR_DISTANCE),
        Vec3::new(0., h, STAR_DISTANCE),
        Vec3::new(0.5 * size , 0., STAR_DISTANCE),
        Vec3::new(-0.5 * size, h - offset, STAR_DISTANCE),
        Vec3::new(0., -offset, STAR_DISTANCE),
        Vec3::new(0.5 * size , h - offset, STAR_DISTANCE),
    ];

    for i in 0..1000 {
        let quat = Quat::from_rotation_ypr(
            (rng.next_u32() as f64 * std::f64::consts::PI * 2.) as f32,
            (rng.next_u32() as f64 * std::f64::consts::PI * 2.) as f32,
            (rng.next_u32() as f64 * std::f64::consts::PI * 2.) as f32,
        );
        for point in &star_points {
            let rotated = quat.mul_vec3(*point);
            vertices.push(([rotated.x, rotated.y, rotated.z], normal(), [1., 0.]));
        }
        let index_offset = i * 6;
        tri_indices.append(&mut vec![
                           index_offset + 0,
                           index_offset + 1,
                           index_offset + 2,
                           index_offset + 3,
                           index_offset + 5,
                           index_offset + 4,
        ]);
    }

    let indices = Indices::U32(tri_indices);

    let mut positions: Vec<[f32; 3]> = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    for (position, normal, uv) in vertices.iter() {
        positions.push(*position);
        normals.push(*normal);
        uvs.push(*uv);
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.set_indices(Some(indices));


    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, VertexAttributeValues::from(positions));
    mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, VertexAttributeValues::from(normals));
    mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, VertexAttributeValues::from(uvs));

    mesh
}

