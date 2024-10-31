use image::{DynamicImage, GenericImageView};
use crate::color::Color;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Texture {
    pub image: Rc<DynamicImage>,
}

impl Texture {
    pub fn new(path: &str) -> Self {
        let image = Rc::new(image::open(path).expect("No se ha podido cargar la imagen"));
        Texture { image }
    }

    pub fn get_color(&self, u: f32, v: f32) -> Color {
        let width = self.image.width();
        let height = self.image.height();
        let x = (u * width as f32).clamp(0.0, width as f32 - 1.0) as u32;
        let y = (v * height as f32).clamp(0.0, height as f32 - 1.0) as u32;

        let pixel = self.image.get_pixel(x, y);
        Color::new(pixel[0], pixel[1], pixel[2])
    }

    pub fn sample(&self, uv: (f32, f32)) -> Color {
        self.get_color(uv.0, uv.1)
    }
}
