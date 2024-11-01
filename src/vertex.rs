use nalgebra_glm::{Mat4, Vec2, Vec3, Vec4};
use crate::color::Color;

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

pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> (Vec3, Vec3) {
    let vertex_position_4d = Vec4::new(vertex.position.x, vertex.position.y, vertex.position.z, 1.0);
    let world_position = uniforms.model_matrix * vertex_position_4d;
    let view_position = uniforms.view_matrix * world_position;
    let projected_position = uniforms.projection_matrix * view_position;

    let screen_position = projected_position.xyz() / projected_position.w;

    let normal = (uniforms.model_matrix * Vec4::new(vertex.normal.x, vertex.normal.y, vertex.normal.z, 0.0)).xyz();
    let transformed_normal = normal.normalize();

    (screen_position, transformed_normal)
}


pub fn rasterize_triangle(v0: Vec3, v1: Vec3, v2: Vec3, color: (u8, u8, u8)) -> Vec<Fragment> {
    let mut fragments = Vec::new();
    for x in v0.x.min(v1.x).min(v2.x) as i32..=v0.x.max(v1.x).max(v2.x) as i32 {
        for y in v0.y.min(v1.y).min(v2.y) as i32..=v0.y.max(v1.y).max(v2.y) as i32 {
            let position = Vec3::new(x as f32, y as f32, 0.0);
            let depth = (v0.z + v1.z + v2.z) / 3.0;
            fragments.push(Fragment {
                position,
                color,
                depth,
            });
        }
    }
    fragments
}

fn fragment_shader(fragment: &Fragment, uniforms: &Uniforms) -> (u8, u8, u8) {
    fragment.color
}

pub fn render_pipeline(vertices: Vec<Vertex>, indices: Vec<u32>, uniforms: &Uniforms) -> Vec<Fragment> {
    let mut fragments = Vec::new();
    for triangle in indices.chunks(3) {
        let (v0,_) = vertex_shader(&vertices[triangle[0] as usize], uniforms);
        let (v1, _)= vertex_shader(&vertices[triangle[1] as usize], uniforms);
        let (v2, _)= vertex_shader(&vertices[triangle[2] as usize], uniforms);
        let triangle_fragments = rasterize_triangle(v0, v1, v2, (255, 0, 0));
        for mut fragment in triangle_fragments {
            let color = fragment_shader(&fragment, uniforms);
            fragment.color = color;
            fragments.push(fragment);
        }
    }
    fragments
}

pub fn calculate_bounding_box(v1: &Vec3, v2: &Vec3, v3: &Vec3) -> (i32, i32, i32, i32) {
    let min_x = v1.x.min(v2.x).min(v3.x).floor() as i32;
    let min_y = v1.y.min(v2.y).min(v3.y).floor() as i32;
    let max_x = v1.x.max(v2.x).max(v3.x).ceil() as i32;
    let max_y = v1.y.max(v2.y).max(v3.y).ceil() as i32;
    (min_x, min_y, max_x, max_y)
}

pub fn barycentric_coordinates(p: &Vec3, a: &Vec3, b: &Vec3, c: &Vec3) -> (f32, f32, f32) {
    let v0 = b - a;
    let v1 = c - a;
    let v2 = p - a;

    let d00 = v0.dot(&v0);
    let d01 = v0.dot(&v1);
    let d11 = v1.dot(&v1);
    let d20 = v2.dot(&v0);
    let d21 = v2.dot(&v1);

    let denom = d00 * d11 - d01 * d01;
    let v = (d11 * d20 - d01 * d21) / denom;
    let w = (d00 * d21 - d01 * d20) / denom;
    let u = 1.0 - v - w;
    (u, v, w)
}

pub fn triangle(v1: &Vertex, v2: &Vertex, v3: &Vertex, uniforms: &Uniforms) -> Vec<Fragment> {
    let mut fragments = Vec::new();
    let (a, normal1) = vertex_shader(v1, uniforms);
    let (b, normal2) = vertex_shader(v2, uniforms);
    let (c, normal3) = vertex_shader(v3, uniforms);

    let (min_x, min_y, max_x, max_y) = calculate_bounding_box(&a, &b, &c);
    let light_dir = Vec3::new(0.0, 0.0, -1.0);

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let point = Vec3::new(x as f32, y as f32, 0.0);
            let (w1, w2, w3) = barycentric_coordinates(&point, &a, &b, &c);

            if w1 >= 0.0 && w2 >= 0.0 && w3 >= 0.0 {
                let normal = (normal1 * w1 + normal2 * w2 + normal3 * w3).normalize();
                let intensity = normal.dot(&light_dir).max(0.0);
                let base_color = Color::new(100, 100, 100);
                let lit_color = base_color.multiply(intensity);

                fragments.push(Fragment {
                    position: Vec3::new(x as f32, y as f32, 0.0),
                    color: lit_color.to_rgb(),
                    depth: point.z,
                });
            }
        }
    }
    fragments
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

