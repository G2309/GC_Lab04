mod pov;
mod color;
mod fragment;
mod framebuffer;
mod line;
mod obj;
mod render;
mod shader;
mod vertex;
mod noise;

use crate::pov::POV;
use crate::obj::Obj;
use minifb::{Window, WindowOptions, Key};
use nalgebra_glm::Vec3;
use std::time::Duration;
use std::f32::consts::PI;
use crate::framebuffer::Framebuffer;
use crate::render::{create_model_matrix, create_perspective_matrix, create_view_matrix, create_viewport_matrix, render, Uniforms};
use crate::color::Color;
use crate::shader::{sun_shader, urano_shader};
use fastnoise_lite::FastNoiseLite;

pub fn start() {
    let window_width = 800;
    let window_height = 800;
    let framebuffer_width =  window_width;
    let framebuffer_height = window_height;
    
    let frame_delay = Duration::from_millis(16);
    let mut framebuffer = Framebuffer::new(window_width, window_height);
    let mut window = Window::new(
      "Planet - Gustavo 22779",
      window_width,
      window_height,
      WindowOptions::default()
    ).unwrap();

    let mut pov = POV::new(
        Vec3::new(5.0, 5.0, 0.0), 
        Vec3::new(0.0, 0.0, 0.0), 
        Vec3::new(0.0, 1.0, 0.0)
    );
    
    framebuffer.set_background_color(32);
    
    let translation = Vec3::new(0.0, 0.0, 0.0);
    let rotation = Vec3::new(0.0, 0.0, 0.0);
    let scale = 2.0f32;
    
    let obj = Obj::load_custom_obj("src/3D/sphere.obj").expect("Failed to load obj");
    let vertex_array = obj.get_vertex_array();

    let model_matrix = create_model_matrix(translation, scale, rotation);
    let mut view_matrix = create_view_matrix(pov.eye, pov.center, pov.up);
    let projection_matrix = create_perspective_matrix(window_width as f32, window_height as f32);
    let viewport_matrix = create_viewport_matrix(framebuffer_width as f32, framebuffer_height as f32);

    
    // RENDER LOOP
    while window.is_open() {
        let time = 0;
        if window.is_key_down(Key::Escape) {
            break;
        }

        handle_input(&window, &mut pov);
        if pov.check_if_changed() {
            view_matrix = create_view_matrix(pov.eye, pov.center, pov.up);
        }
        let uniforms = Uniforms {
    model_matrix,
    view_matrix,
    projection_matrix,
    viewport_matrix,
    time: 0, // Tiempo inicial
    noise: FastNoiseLite::new(),
    cloud_noise: FastNoiseLite::new(),
    band_noise: FastNoiseLite::new(),
    current_shader: 0, // Por ejemplo, 0 para sun_shader
};

framebuffer.clear();
render(&mut framebuffer, &uniforms, &vertex_array, time);

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}

fn handle_input(window: &Window, pov: &mut POV) {

    const ROTATION_SPEED : f32 = PI /20.0;
    const ZOOM_SPEED : f32 = 0.75;

    if window.is_key_down(Key::Right) {
        pov.orbit(ROTATION_SPEED, 0.0);
    }
    if window.is_key_down(Key::Left) {
        pov.orbit(-ROTATION_SPEED, 0.0);
    }
    if window.is_key_down(Key::Down) {
        pov.orbit(0.0, -ROTATION_SPEED);
    }
    if window.is_key_down(Key::Up) {
        pov.orbit(0.0, ROTATION_SPEED);
    }

    // pov zoom
    if window.is_key_down(Key::W) {
        pov.zoom(ZOOM_SPEED);
    }
    if window.is_key_down(Key::S) {
        pov.zoom(-ZOOM_SPEED);
    }
    
}

fn main() {
    start();
}
