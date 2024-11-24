use nalgebra_glm::{dot, Vec2, Vec3};
use crate::color::Color;
use crate::fragment::Fragment;
use crate::vertex::Vertex;

pub fn line(v1: &Vertex, v2: &Vertex) -> Vec<Fragment> {
    let mut fragments = Vec::new();

    let a = Vec2::new(v1.transformed_position.x, v1.transformed_position.y);
    let b = Vec2::new(v2.transformed_position.x, v2.transformed_position.y);

    let diff = b - a;
    let steps = diff.magnitude().ceil() as usize;
    let step = diff / steps as f32;

    let z_diff = v2.transformed_position.z - v1.transformed_position.z;
    let color_start = v1.color;
    let color_end = v2.color;

    let normal_start = v1.transformed_normal;
    let normal_end = v2.transformed_normal;

    let tex_coords_start = v1.tex_coords;
    let tex_coords_end = v2.tex_coords;

    for i in 0..=steps {
        let t = i as f32 / steps as f32;

        let position = a + step * i as f32;
        let depth = v1.transformed_position.z + z_diff * t;
        let color = color_start.lerp(&color_end, t);
        let normal = (normal_start * (1.0 - t) + normal_end * t).normalize();
        let tex_coords = tex_coords_start * (1.0 - t) + tex_coords_end * t;

        fragments.push(Fragment::new(
            position,
            color,
            depth,
            normal,
            0.0, 
            Vec3::new(position.x, position.y, depth),
            Some(tex_coords),
        ));
    }

    fragments
}

pub fn triangle_wireframe(v1: &Vertex, v2: &Vertex, v3: &Vertex) -> Vec<Fragment> {
    let mut fragments = Vec::new();

    fragments.extend(line(v1, v2));
    fragments.extend(line(v2, v3));
    fragments.extend(line(v1, v3));

    fragments
}

pub fn triangle_flat_shade(v1: &Vertex, v2: &Vertex, v3: &Vertex) -> Vec<Fragment> {
    let mut fragments = Vec::new();

    let (a, b, c) = (
        v1.transformed_position,
        v2.transformed_position,
        v3.transformed_position,
    );

    let light_dir = Vec3::new(0.0, 1.0, -1.0);

    let (min_x, min_y, max_x, max_y) = calculate_bounding_box(&a, &b, &c);
    let triangle_area = edge_function(&a, &b, &c);

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let point = Vec3::new(x as f32 + 0.5, y as f32 + 0.5, 0.0);
            let (w1, w2, w3) = barycentric_coordinates(&point, &a, &b, &c, triangle_area);

            if w1 >= 0.0 && w2 >= 0.0 && w3 >= 0.0 {
                let normal = (v1.transformed_normal * w1
                    + v2.transformed_normal * w2
                    + v3.transformed_normal * w3)
                    .normalize();
                let intensity = dot(&normal, &light_dir).max(0.0);
                let base_color = Color::new(255, 255, 255);
                let lit_color = base_color * intensity;

                let depth = a.z * w1 + b.z * w2 + c.z * w3;

                let tex_coords = v1.tex_coords * w1 + v2.tex_coords * w2 + v3.tex_coords * w3;

                fragments.push(Fragment::new(
                    Vec2::new(point.x, point.y),
                    lit_color,
                    depth,
                    normal,
                    intensity,
                    point,
                    Some(tex_coords),
                ));
            }
        }
    }

    fragments
}

fn calculate_bounding_box(v1: &Vec3, v2: &Vec3, v3: &Vec3) -> (i32, i32, i32, i32) {
    let min_x = v1.x.min(v2.x).min(v3.x).floor() as i32;
    let min_y = v1.y.min(v2.y).min(v3.y).floor() as i32;
    let max_x = v1.x.max(v2.x).max(v3.x).ceil() as i32;
    let max_y = v1.y.max(v2.y).max(v3.y).ceil() as i32;

    (min_x, min_y, max_x, max_y)
}

fn barycentric_coordinates(
    p: &Vec3,
    a: &Vec3,
    b: &Vec3,
    c: &Vec3,
    area: f32,
) -> (f32, f32, f32) {
    let w1 = edge_function(b, c, p) / area;
    let w2 = edge_function(c, a, p) / area;
    let w3 = edge_function(a, b, p) / area;

    (w1, w2, w3)
}
fn edge_function(a: &Vec3, b: &Vec3, c: &Vec3) -> f32 {
    (c.x - a.x) * (b.y - a.y) - (c.y - a.y) * (b.x - a.x)
}
