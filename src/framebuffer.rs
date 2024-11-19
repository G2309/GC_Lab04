use std::f32::INFINITY;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

use crate::color::Color;

pub struct Framebuffer {
    pub width : usize, 
    pub height: usize,
    pub buffer : Vec<u32>,
    pub zbuffer : Vec<f32>,
    background_color : Color,
    current_color : Color
}

pub trait RenderableToFile {
    fn render_buffer(&self, filename: &str) -> io::Result<()>;
    fn write_bmp_header(&self, file: &mut File) -> io::Result<()>;
    fn write_pixel_data(&self, file: &mut File) -> io::Result<()>;
}

impl Framebuffer {
    pub fn new(width: usize, height: usize, background_color: Color ) -> Self {
        let buffer_size = width * height;
        let buffer = vec![background_color.to_hex(); buffer_size]; 
        let zbuffer = vec![f32::INFINITY; buffer_size];
        Framebuffer {
            width,
            height,
            buffer,
            zbuffer,
            background_color,
            current_color: Color::new(0, 0, 0), 
        }
    }

    pub fn new_default(width: usize, height: usize) -> Self {
        let white_color = Color::new(255, 255, 255);
        Self::new(width, height, white_color)
    }

    pub fn clear(&mut self) {
        let background_hex = self.background_color.to_hex();
        for i in 0..self.buffer.len() {
            self.buffer[i] = background_hex;
            self.zbuffer[i] = INFINITY;
        }
    }

    pub fn draw_point(&mut self, x: usize, y: usize, depth: f32) {
        if  0 < x  
            && x < self.width 
            && 0 < y 
            && y < self.height {
            let index = y * self.width + x;
            if self.zbuffer[index] > depth {
                self.buffer[index] = self.current_color.to_hex();
                self.zbuffer[index] = depth;
            }
        }
    }
    pub fn get_point_color(&mut self, x: usize, y: usize) -> Color{
        if x < self.width && y < self.height {
            let index = y * self.width + x;
            let color = Color::from_hex(self.buffer[index]);
            return  color
        }
        return Color::new(0, 0, 0);
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
        self.clear(); // Clear buffer with the new background color
    }
    
    pub fn set_background_color_hex(&mut self, hex: u32){
        self.background_color = Color::from_hex(hex);
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }
    pub fn set_current_color_hex(&mut self, hex: u32) {
        self.current_color = Color::from_hex(hex);
    }
}

impl RenderableToFile for Framebuffer {
   fn render_buffer(&self, filename: &str) -> io::Result<()> {
        let path = Path::new(filename);
        let mut file = File::create(&path)?;
        self.write_bmp_header(&mut file)?;
        self.write_pixel_data(&mut file)?;
        Ok(())
   } 

   fn write_bmp_header(&self, file: &mut File) -> io::Result<()> {
    let file_size = 14 + 40 + (self.width * self.height * 4) as u32;
    let reserved: u32 = 0;
    let data_offset: u32 = 14 + 40;
    let header_size: u32 = 40;
    let planes: u16 = 1;
    let bits_per_pixel: u16 = 32;
    let compression: u32 = 0;
    let image_size = (self.width * self.height * 4) as u32;
    let x_pixels_per_meter: u32 = 0;
    let y_pixels_per_meter: u32 = 0;
    let total_colors: u32 = 0;
    let important_colors: u32 = 0;

    file.write_all(b"BM")?;
    file.write_all(&file_size.to_le_bytes())?;
    file.write_all(&reserved.to_le_bytes())?;
    file.write_all(&data_offset.to_le_bytes())?;

    file.write_all(&header_size.to_le_bytes())?;
    file.write_all(&(self.width as u32).to_le_bytes())?;
    file.write_all(&(self.height as u32).to_le_bytes())?;
    file.write_all(&planes.to_le_bytes())?;
    file.write_all(&bits_per_pixel.to_le_bytes())?;
    file.write_all(&compression.to_le_bytes())?;
    file.write_all(&image_size.to_le_bytes())?;
    file.write_all(&x_pixels_per_meter.to_le_bytes())?;
    file.write_all(&y_pixels_per_meter.to_le_bytes())?;
    file.write_all(&total_colors.to_le_bytes())?;
    file.write_all(&important_colors.to_le_bytes())?; 

    Ok(())
}

fn write_pixel_data(&self, file: &mut File) -> io::Result<()> {
    for y in 0..self.height { 
        for x in 0..self.width {
            let index = y * self.width + x;
            let color_hex = self.buffer[index];
            let r = (color_hex >> 16) & 0xFF;
            let g = (color_hex >> 8) & 0xFF;
            let b = color_hex & 0xFF;
            let a = 0xFF; 

            file.write_all(&[b as u8, g as u8, r as u8, a as u8])?;
        }
    }
    Ok(())
}
}
