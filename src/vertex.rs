use nalgebra_glm::{Mat4, Vec2, Vec3};

pub struct Vertex {
    pub position: Vec3,
    pub normal: Vec3,
    pub texcoord: Vec2,
}

pub struct Uniforms {
    pub model_matrix: Mat4,
    pub view_matrix: Mat4,
    pub projection_matrix: Mat4,
    pub light_position: Vec3,
    pub camera_position: Vec3,
}

pub struct Fragment {
    pub position: Vec3,
    pub color: (u8, u8, u8), 
    pub depth: f32,
}

impl Vertex {
    pub fn new(position: Vec3, normal: Vec3, texcoord: Vec2) -> Self {
        Vertex {
            position,
            normal,
            texcoord,
        }
    }
}

