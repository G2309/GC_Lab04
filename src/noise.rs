use fastnoise_lite::{FastNoiseLite, NoiseType, FractalType};

pub fn create_noise(current_shader: u8) -> FastNoiseLite {
    match current_shader {
        1 => create_kenshi_noise(),
        2 => create_ratchet_t_noise(),
        3 => create_rocky_noise(),
        4 => FastNoiseLite::new(),
        5 => create_ratchet_noise(),
        6 => create_simple_noise(), 
        8 => create_moon_noise(),
        9 => FastNoiseLite::new(),
        _ => create_kenshi_noise(),  
    }
}
pub fn create_cloud_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(805);  
    noise.set_noise_type(Some(NoiseType::Perlin)); 
    noise.set_fractal_type(Some(FractalType::FBm));
    noise.set_fractal_octaves(Some(2)); 
    noise.set_fractal_lacunarity(Some(3.0));
    noise.set_fractal_gain(Some(0.5));
    noise.set_frequency(Some(0.01)); 
    noise
}

fn create_kenshi_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(623);
    noise.set_noise_type(Some(NoiseType::OpenSimplex2S));
    noise.set_fractal_type(Some(FractalType::Ridged));
    noise.set_fractal_octaves(Some(5));
    noise.set_fractal_lacunarity(Some(2.0));
    noise.set_fractal_gain(Some(0.5));
    noise.set_frequency(Some(0.8));
    noise
}

fn create_ratchet_t_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(3344);
    noise.set_noise_type(Some(NoiseType::Perlin));
    noise.set_fractal_type(Some(FractalType::Ridged));
    noise.set_fractal_octaves(Some(8));
    noise.set_fractal_lacunarity(Some(3.0));
    noise.set_fractal_gain(Some(0.7));
    noise.set_frequency(Some(1.8));
    noise
}

pub fn create_moon_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(4321);
    noise.set_noise_type(Some(NoiseType::OpenSimplex2));
    noise.set_fractal_type(Some(FractalType::PingPong));
    noise.set_fractal_octaves(Some(2));
    noise.set_fractal_lacunarity(Some(2.0));
    noise.set_fractal_gain(Some(0.5));
    noise.set_frequency(Some(3.0));
    noise
}

fn create_rocky_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(4321);
    noise.set_noise_type(Some(NoiseType::Perlin));
    noise.set_fractal_type(Some(FractalType::PingPong));
    noise.set_fractal_octaves(Some(5));
    noise.set_fractal_lacunarity(Some(2.0));
    noise.set_fractal_gain(Some(1.0));
    noise.set_frequency(Some(5.0));
    noise
}

fn create_ratchet_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(9876);
    noise.set_noise_type(Some(NoiseType::OpenSimplex2));
    noise.set_fractal_type(Some(FractalType::DomainWarpProgressive));
    noise.set_fractal_octaves(Some(6));
    noise.set_fractal_lacunarity(Some(2.0));
    noise.set_fractal_gain(Some(0.5));
    noise.set_frequency(Some(2.0));
    noise
}

fn create_simple_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(2021);
    noise.set_noise_type(Some(NoiseType::OpenSimplex2));
    noise.set_fractal_type(Some(FractalType::Ridged));
    noise.set_fractal_octaves(Some(4));
    noise.set_fractal_lacunarity(Some(2.0));
    noise.set_fractal_gain(Some(0.4));
    noise.set_frequency(Some(0.2));
    noise
}

