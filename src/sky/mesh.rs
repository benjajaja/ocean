use bevy::{
    prelude::*,
    render::mesh::{Indices, VertexAttributeValues},
};
use rand::distributions::{Distribution, Uniform};
use std::f32::consts::{FRAC_PI_2, PI};
use wgpu::PrimitiveTopology;

pub const STAR_DISTANCE: f32 = 1000.; // work around broken depth_stencil trick

pub struct StarDef {
    pub quat: Quat,
    pub size: f32,
}

pub fn bg_stars() -> Mesh {
    let mut quat_rng = RandomRotation::new();
    let mut rng = rand::thread_rng();
    let dist = Uniform::from(0.001..0.005);

    let mut simple_stars = Vec::new();
    for _ in 0..1000 {
        if let Some(quat) = quat_rng.next() {
            simple_stars.push(StarDef {
                quat,
                size: dist.sample(&mut rng) * STAR_DISTANCE,
            });
        }
    }
    stars(simple_stars)
}
pub fn island_stars(defs: Vec<StarDef>) -> Mesh {
    stars(defs)
}

const NORMAL: [f32; 3] = [0.0, 1.0, 0.1];

fn stars(defs: Vec<StarDef>) -> Mesh {
    let mut vertices: Vec<([f32; 3], [f32; 3], [f32; 2])> = vec![];
    let mut tri_indices: Vec<u32> = vec![];

    for (i, def) in defs.iter().enumerate() {
        let star_points: [Vec3; 4] = [
            Vec3::new(-0.5 * def.size, -0.5 * def.size, STAR_DISTANCE),
            Vec3::new(-0.5 * def.size, 0.5 * def.size, STAR_DISTANCE),
            Vec3::new(0.5 * def.size, -0.5 * def.size, STAR_DISTANCE),
            Vec3::new(0.5 * def.size, 0.5 * def.size, STAR_DISTANCE),
        ];
        for point in &star_points {
            let rotated = def.quat
                * (Quat::from_rotation_x(-FRAC_PI_2) * (Quat::from_rotation_z(PI) * *point));
            vertices.push((
                [rotated.x, rotated.y, rotated.z],
                NORMAL,
                [point.x / def.size + 0.5, point.y / def.size + 0.5],
            ));
        }
        let index_offset = i as u32 * 4;
        tri_indices.append(&mut vec![
            index_offset + 0,
            index_offset + 1,
            index_offset + 2,
            index_offset + 1,
            index_offset + 3,
            index_offset + 2,
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

    mesh.set_attribute(
        Mesh::ATTRIBUTE_POSITION,
        VertexAttributeValues::from(positions),
    );
    mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, VertexAttributeValues::from(normals));
    mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, VertexAttributeValues::from(uvs));

    mesh
}

pub struct RandomRotation {
    rng: rand::prelude::ThreadRng,
    dist: rand::distributions::Uniform<f32>,
}

impl RandomRotation {
    pub fn new() -> Self {
        let rng = rand::thread_rng();
        RandomRotation {
            rng,
            dist: Uniform::from(0.0..1.0),
        }
    }
}

impl Iterator for RandomRotation {
    type Item = Quat;
    fn next(&mut self) -> Option<Quat> {
        let u = self.dist.sample(&mut self.rng);
        let v = self.dist.sample(&mut self.rng);
        let w = self.dist.sample(&mut self.rng);
        Some(Quat::from_xyzw(
            (1. - u).sqrt() * (2. * PI * v).sin(),
            (1. - u).sqrt() * (2. * PI * v).cos(),
            u.sqrt() * (2. * PI * w).sin(),
            u.sqrt() * (2. * PI * w).cos(),
        ))
    }
}
