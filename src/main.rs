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
use crate::render::{create_model_matrix, create_perspective_matrix, create_view_matrix, create_viewport_matrix, render, Uniforms, gaussian_blur, apply_bloom};
use fastnoise_lite::FastNoiseLite;
use crate::noise::{create_noise, create_cloud_noise};

pub fn start() {
    let window_width = 600;
    let window_height = 600;
    let framebuffer_width = window_width;
    let framebuffer_height = window_height;

    let frame_delay = Duration::from_millis(16);
    let mut framebuffer = Framebuffer::new(window_width, window_height);
    let mut window = Window::new(
        "Planet - Gustavo 22779",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap();

    let mut pov = POV::new(
        Vec3::new(5.0, 5.0, 0.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    framebuffer.set_background_color(120);

    let translation = Vec3::new(0.0, 0.0, 0.0);
    let rotation = Vec3::new(0.0, 0.0, 0.0);
    let scale =1.0f32;

    let obj = Obj::load_custom_obj("src/3D/sphere.obj").expect("Failed to load obj");
    let vertex_array = obj.get_vertex_array();

    let mut time = 0;
    let mut current_shader = 1;
    let mut current_noise = (
        create_noise(1),
        create_noise(2),
        create_noise(6),
        create_noise(7),
    );

    let model_matrix = create_model_matrix(translation, scale, rotation);
    let mut view_matrix = create_view_matrix(pov.eye, pov.center, pov.up);
    let projection_matrix = create_perspective_matrix(window_width as f32, window_height as f32);
    let viewport_matrix = create_viewport_matrix(framebuffer_width as f32, framebuffer_height as f32);

    // RENDER LOOP
    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }

        let keys = window.get_keys_pressed(minifb::KeyRepeat::No);
        for key in keys {
            match key {
                Key::Key1 => {current_shader = 1; current_noise.0 = create_noise(8)},
                Key::Key6 => {current_shader = 6; current_noise.1 = create_noise(6)},
                Key::Key7 => {current_shader = 7; current_noise.2 = create_noise(9)},
                _ => {}
            }
        }

        handle_input(&window, &mut pov);
        if pov.check_if_changed() {
            view_matrix = create_view_matrix(pov.eye, pov.center, pov.up);
        }

        framebuffer.clear();

        let mut uniforms = Uniforms {
            model_matrix,
            view_matrix,
            projection_matrix,
            viewport_matrix,
            time,
            noise: create_noise(1),
            cloud_noise: create_cloud_noise(),
            band_noise: FastNoiseLite::new(),
            current_shader,
        };

        if current_shader == 1 {
            uniforms.current_shader = 1;
            uniforms.model_matrix = create_model_matrix(translation, scale, rotation);
            render(&mut framebuffer, &uniforms, &vertex_array, time);
            let moon_angle = time as f32 * 0.02;
            let moon_translation = Vec3::new(
                3.0 * moon_angle.cos(),
                0.0,
                3.0 * moon_angle.sin(),
            );
            let moon_angle_2 = time as f32 * 0.015;
            let moon_translation_2 = Vec3::new(
                5.0 * moon_angle_2.cos(),
                3.0,
                5.0 * moon_angle_2.sin(),
            );
            uniforms.current_shader = 2;
            uniforms.model_matrix = create_model_matrix(moon_translation_2, 0.3, Vec3::new(0.0, 0.0, 0.0)); 
            render(&mut framebuffer, &uniforms, &vertex_array, time);
            uniforms.model_matrix = create_model_matrix(moon_translation, 0.5, Vec3::new(0.0, 0.0, 0.0));
            render(&mut framebuffer, &uniforms, &vertex_array, time);
        } else if current_shader == 6 {
            uniforms.current_shader = 6;
            uniforms.model_matrix = create_model_matrix(translation, scale, rotation);
            render(&mut framebuffer, &uniforms, &vertex_array, time);
            uniforms.model_matrix = create_model_matrix(translation, scale * 1.5, Vec3::new(0.0, 0.0, 0.0));
            render(&mut framebuffer, &uniforms, &vertex_array, time);
        } else if current_shader == 7 {
            uniforms.current_shader = 7;
            uniforms.model_matrix = create_model_matrix(translation, scale, rotation);
            render(&mut framebuffer, &uniforms, &vertex_array, time);
            gaussian_blur(&mut framebuffer.emissive_buffer, framebuffer.width, framebuffer.height, 30, 4.5);
            apply_bloom(&mut framebuffer.buffer, &framebuffer.emissive_buffer, framebuffer.width, framebuffer.height);
        }

        time += 1;

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}

fn handle_input(window: &Window, pov: &mut POV) {
    const ROTATION_SPEED: f32 = PI / 20.0;
    const ZOOM_SPEED: f32 = 0.75;

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

