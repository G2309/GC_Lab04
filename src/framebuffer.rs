//                          Framebuffer class
//                          Gustavo Cruz
//                          # 22779
use crate::color::Color;

pub struct FrameBuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<Color>,
    pub background_color: Color,
    pub current_color: Color
}

impl FrameBuffer {
    pub fn new(width: usize, height: usize) -> FrameBuffer {
        let default_color = Color::new(255, 255, 255);
        let buffer = vec![default_color; width * height];
        FrameBuffer {
            width,
            height,
            buffer,
            background_color: default_color,
            current_color: default_color
        }
    }
    // Limpia el buffer con el color de fondo
    pub fn clear(&mut self) {
        self.buffer.fill(self.background_color);
    }
    // Dibuja un punto en las coordenadas (x, y) con el color actual
    pub fn point(&mut self, x: usize, y: usize) {
        let adjusted_y = self.height - 1 - y;
        self.buffer[self.width * y + x] = self.current_color;
    }
    // Establece el color de fondo
    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }
    // Obtiene el color en las coordenadas (x, y)
    pub fn get_color(&self, x: usize, y: usize) -> Color {
        self.buffer[self.width * y + x]
    }
    // Establece el color actual para dibujar
    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }
}

