use tobj;
use nalgebra_glm::{Vec2, Vec3};
use crate::vertex::Vertex;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Obj {
    vertices: Vec<Vec3>,
    normals: Vec<Vec3>,
    texcoords: Vec<Vec2>,
    indices: Vec<u32>,
}

pub fn load_obj(file_path: &str) -> (Vec<Vertex>, Vec<u32>) {
    let file = File::open(file_path).expect("Error al abrir el archivo OBJ");
    let reader = BufReader::new(file);

    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    let mut temp_vertices = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.is_empty() {
            continue;
        }

        match parts[0] {
		    "v" => {
		        // Parsear vértices
		        let x: f32 = parts[1].parse().expect("Error al parsear coordenada x del vértice");
		        let y: f32 = parts[2].parse().expect("Error al parsear coordenada y del vértice");
		        let z: f32 = parts[3].parse().expect("Error al parsear coordenada z del vértice");
		        temp_vertices.push(Vec3::new(x, y, z));
		    }
		    "f" => {
		        // Parsear caras, con soporte para formato `v/vt/vn`
		        for i in 1..=3 {
		            let vertex_data: Vec<&str> = parts[i].split('/').collect();
		            let idx: usize = vertex_data[0]
		                .parse::<usize>()
		                .expect("Error al parsear índice del vértice en la cara") - 1;
		            indices.push(idx as u32);
		        }
		    }
		    _ => {}
		}
    }

    for position in temp_vertices {
        vertices.push(Vertex {
            position,
            normal: Vec3::new(0.0, 0.0, 1.0),  // Normal predeterminada
            texcoord: Vec2::new(0.0, 0.0),     // Coordenada de textura
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

