use noise::{Perlin, NoiseFn};

#[derive(Clone)]
pub struct Noise {
    perlin: Perlin,
} 

impl Noise {
    pub fn new(seed: u32) -> Noise {
        Noise { perlin: Perlin::new(seed) }
    }
    
    pub fn get_noise_3d(&self, x:f32, y:f32, z:f32) -> f32 {
        self.perlin.get([x as f64, y as f64, z as f64]) as f32
    }
}
