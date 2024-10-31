use nalgebra_glm::{Vec2, Vec3};

pub struct Vertex {
    pub position: Vec3,
    pub normal: Vec3,
    pub texcoord: Vec2,
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

