use tobj;
use nalgebra_glm::{Vec2, Vec3};
use crate::vertex::Vertex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub struct Obj {
    vertices: Vec<Vec3>,
    normals: Vec<Vec3>,
    texcoords: Vec<Vec2>,
    indices: Vec<u32>,
}

pub fn load_obj(filename: &str) -> (Vec<Vertex>, Vec<u32>) {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    let mut positions = Vec::new();

    if let Ok(file) = File::open(filename) {
        for line in io::BufReader::new(file).lines().flatten() {
            if line.starts_with("v ") {
                // Parse vertex positions
                let parts: Vec<&str> = line.split_whitespace().collect();
                let x: f32 = parts[1].parse().unwrap();
                let y: f32 = parts[2].parse().unwrap();
                let z: f32 = parts[3].parse().unwrap();
                positions.push(Vec3::new(x, y, z));
            } else if line.starts_with("f ") {
                // Parse face indices
                let parts: Vec<&str> = line.split_whitespace().collect();
                for i in 1..=3 {
                    let index: u32 = parts[i].split('/').next().unwrap().parse().unwrap();
                    indices.push(index - 1); // OBJ indices start from 1
                }
            }
        }
    }

    // Convert positions to Vertex structs
    for pos in positions {
        vertices.push(Vertex {
            position: pos,
            normal: Vec3::zeros(),
            texcoord: Vec2::zeros(),
        });
    }

    (vertices, indices)
}

impl Obj {
    pub fn load(filename: &str) -> Result<Self, tobj::LoadError> {
        let (models, _) = tobj::load_obj(filename, &tobj::LoadOptions {
            single_index: true,
            triangulate: true,
            ..Default::default()
        })?;

        let mesh = &models[0].mesh;

        let vertices: Vec<Vec3> = mesh.positions.chunks(3)
            .map(|v| Vec3::new(v[0], v[1], v[2]))
            .collect();

        let normals: Vec<Vec3> = mesh.normals.chunks(3)
            .map(|n| Vec3::new(n[0], n[1], n[2]))
            .collect();

        let texcoords: Vec<Vec2> = mesh.texcoords.chunks(2)
            .map(|t| Vec2::new(t[0], t[1]))
            .collect();

        let indices = mesh.indices.clone();

        Ok(Obj {
            vertices,
            normals,
            texcoords,
            indices,
        })
    }

    pub fn build_vertex_array(&self) -> Vec<Vertex> {
        let mut vertex_array = Vec::with_capacity(self.indices.len());

        for &index in &self.indices {
            let position = self.vertices[index as usize];
            let normal = if !self.normals.is_empty() {
                self.normals[index as usize]
            } else {
                Vec3::new(0.0, 0.0, 0.0)
            };
            let texcoord = if !self.texcoords.is_empty() {
                self.texcoords[index as usize]
            } else {
                Vec2::new(0.0, 0.0)
            };

            vertex_array.push(Vertex::new(position, normal, texcoord));
        }

        vertex_array
    }
}

