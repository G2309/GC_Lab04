use nalgebra_glm::{Vec3, Vec4};
use crate::vertex::Vertex;
use crate::render::Uniforms;
use crate::color::Color;
use crate::fragment::Fragment;

pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
    let position = Vec4::new(
        vertex.position.x,
        vertex.position.y,
        vertex.position.z,
        1.0
    );
        let transformed = uniforms.viewport_matrix * uniforms.perspective_matrix * uniforms.view_matrix * uniforms.model_matrix * position;
        let w = transformed.w;
    let transformed_position = Vec3::new(
        transformed.x / w,
        transformed.y / w,
        transformed.z / w
    );
    Vertex {
        position: vertex.position,
        normal: vertex.normal,
        tex_coords: vertex.tex_coords,
        color: vertex.color,
        transformed_position,
        transformed_normal: vertex.normal,
    }
}

pub fn sun_shader(vertex: &Vertex, uniforms: &Uniforms, time: f32) -> Vertex {
    let mut transformed_vertex = vertex_shader(vertex, uniforms);
    let mut color = Vec3::new(1.0, 5.0, 1.0);
    let noise = (time * 5.0 + transformed_vertex.position.x * 10.0).sin() * 0.5 + 0.5;
    if noise > 0.8 { 
        color += Vec3::new(1.0, 0.5, 0.1); 
    }
    transformed_vertex.color = crate::color::Color::new(
        (color.x * 255.0).clamp(0.0, 255.0) as u8,
        (color.y * 255.0).clamp(0.0, 255.0) as u8,
        (color.z * 255.0).clamp(0.0, 255.0) as u8,
    );
    transformed_vertex
}

pub fn urano_shader(vertex: &Vertex, uniforms: &Uniforms, time: f32) -> Vertex {
    let mut transformed_vertex = vertex_shader(vertex, uniforms);
    let base_color = Vec3::new(0.2, 0.5, 1.0); // Azul claro
    let noise = (time * 0.5 + transformed_vertex.position.x * 3.0).sin() * 0.2 + 0.8;  // Añade variación de gas
    let color_variation = base_color * noise;
    transformed_vertex.color = Color::new(
        (color_variation.x * 255.0).clamp(0.0, 255.0) as u8,
        (color_variation.y * 255.0).clamp(0.0, 255.0) as u8,
        (color_variation.z * 255.0).clamp(0.0, 255.0) as u8,
    );
    transformed_vertex
}
