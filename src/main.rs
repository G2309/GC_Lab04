mod framebuffer;
mod texture;
mod color;
mod obj;
mod vertex;

use nalgebra_glm::{Mat4, Vec2, Vec3};
use minifb::{Key, Window, WindowOptions};
use vertex::{Vertex, Uniforms, render_pipeline};

fn main() {
    let width = 800;
    let height = 600;
    let mut window = Window::new("3D Renderer", width, height, WindowOptions::default()).unwrap();
    
    let mut buffer: Vec<u32> = vec![0; width * height];

    let uniforms = Uniforms {
        model_matrix: Mat4::identity(),
        view_matrix: Mat4::identity(),
        projection_matrix: Mat4::identity(),
        light_position: Vec3::new(0.0, 0.0, -1.0),
        camera_position: Vec3::new(0.0, 0.0, 5.0),
    };

    let vertices: Vec<Vertex> = vec![];
    let indices: Vec<u32> = vec![];

    let fragments = render_pipeline(vertices, indices, &uniforms);

    for fragment in fragments {
        let x = fragment.position.x as usize;
        let y = fragment.position.y as usize;

        if x < width && y < height {
            buffer[y * width + x] = ((fragment.color.0 as u32) << 16)
                                  | ((fragment.color.1 as u32) << 8)
                                  | (fragment.color.2 as u32);
        }
    }

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer, width, height).unwrap();
    }
}

