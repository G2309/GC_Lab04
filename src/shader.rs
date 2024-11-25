use nalgebra_glm::{Vec2,Vec3, Vec4, dot, normalize, smoothstep};
use crate::vertex::Vertex;
use crate::render::Uniforms;
use crate::color::Color;
use crate::fragment::Fragment;

pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
    let position = Vec4::new(
        vertex.position.x,
        vertex.position.y,
        vertex.position.z,
        1.0
    );
        let transformed = uniforms.viewport_matrix * uniforms.projection_matrix * uniforms.view_matrix * uniforms.model_matrix * position;
        let w = transformed.w;
    let transformed_position = Vec3::new(
        transformed.x / w,
        transformed.y / w,
        transformed.z / w
    );
    Vertex {
        position: vertex.position,
        normal: vertex.normal,
        tex_coords: vertex.tex_coords,
        color: vertex.color,
        transformed_position,
        transformed_normal: vertex.normal,
    }
}

pub fn fragment_shader(fragment: &Fragment, uniforms: &Uniforms, time: u32) -> (Color, u32) {
  match uniforms.current_shader {
      1 => kenshi_shader(fragment, uniforms, time),
      2 => moon_shader(fragment, uniforms),
      3 => ratchet_toxic_shader(fragment, uniforms, time),
      4 => rocky_planet_shader(fragment, uniforms, time),
      5 => ratchet_shader(fragment, uniforms, time as f32),
      6 => ratchet1_shader(fragment, uniforms, time),
      7 => sun_shader(time),
      8 => simple_planet_shader(fragment, uniforms),
      _ => (Color::new(0, 0, 0), 0),
  }
}

pub fn simple_planet_shader(fragment: &Fragment, uniforms: &Uniforms) -> (Color, u32) {
    let (base_color, detail_color) = match uniforms.current_shader {
        1 => (Color::from_float(0.6, 0.4, 0.2), Color::from_float(0.4, 0.3, 0.1)), 
        2 => (Color::from_float(0.2, 0.6, 0.3), Color::from_float(0.1, 0.4, 0.2)),
        3 => (Color::from_float(0.2, 0.3, 0.6), Color::from_float(0.1, 0.2, 0.5)),
        _ => (Color::from_float(0.8, 0.8, 0.8), Color::from_float(0.5, 0.5, 0.5)), 
    };

    let noise_value = uniforms.noise.get_noise_2d(
        fragment.vertex_position.x * 50.0, 
        fragment.vertex_position.y * 50.0
    );
    let normalized_noise = (noise_value + 1.0) * 0.5; 

    let surface_color = base_color.lerp(&detail_color, normalized_noise.clamp(0.0, 1.0));

    let light_position = Vec3::new(5.0, 5.0, 5.0); 
    let light_direction = (light_position - fragment.vertex_position).normalize(); 
    let normal = fragment.normal.normalize(); // Normal del fragmento
    let diffuse = normal.dot(&light_direction).max(0.0); // Cálculo de iluminación difusa

    (surface_color * (0.3 + 0.7 * diffuse), 0)
}

fn rocky_planet_shader(fragment: &Fragment, uniforms: &Uniforms, time: u32) -> (Color, u32) {
    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;
    let t = time as f32 * 0.1;

    let base_noise_value = uniforms.noise.get_noise_2d(x, y);
    let rock_noise_value = uniforms.noise.get_noise_3d(x - t, x+y -t, y );

    let rocky_color_1 = Color::from_float(0.6, 0.5, 0.4); // Color principal rocoso
    let rocky_color_2 = Color::from_float(0.4, 0.3, 0.2); // Color de zonas más oscuras y rocosas
    let rocky_color_3 = Color::from_float(0.2, 0.1, 0.05); // Color de las zonas más áridas y secas

    let land_threshold = 0.4;

    let base_color = if base_noise_value > land_threshold {
        let rock_intensity = (base_noise_value - land_threshold) / (1.0 - land_threshold);
        if rock_intensity < 0.5 {
            rocky_color_1.lerp(&rocky_color_2, rock_intensity * 2.0)
        } else {
            rocky_color_2.lerp(&rocky_color_3, (rock_intensity - 0.5) * 2.0)
        }
    } else {
        rocky_color_1.lerp(&rocky_color_2, base_noise_value / land_threshold)
    };

    // Calculamos la iluminación basada en la posición de la luz
    let light_position = Vec3::new(1.0, 1.0, 3.0); // Posición de la fuente de luz
    let light_dir = normalize(&(light_position - fragment.vertex_position)); // Dirección de la luz
    let normal = normalize(&fragment.normal); // Normal del fragmento para iluminación
    let diffuse = dot(&normal, &light_dir).max(0.0); // Cálculo de la iluminación difusa

    let lit_color = base_color * (0.1 + 0.9 * diffuse);

    let dust_threshold = 0.3; 
    let dust_opacity = 0.2 + 0.1 * ((time as f32 / 500.0) * 0.5).sin().abs(); 
    if rock_noise_value > dust_threshold {
        let dust_intensity = ((rock_noise_value - dust_threshold) / (1.0 - dust_threshold)).clamp(0.0, 1.0);
        (lit_color.blend_add(&(rocky_color_3 * (dust_intensity * dust_opacity))), 0)
    } else {
        (lit_color, 0)
    }
}


pub fn ratchet_toxic_shader(fragment: &Fragment, uniforms: &Uniforms, time: u32) -> (Color, u32) {
    let zoom = 100.0;  
    let ox = 100.0; 
    let oy = 100.0;
    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;
    let t = time as f32 * 0.1;

    // Base noise para la tierra
    let base_noise_value = uniforms.noise.get_noise_2d(x * x, y);
    
    // Movimiento en la atmósfera
    let offset_x = t * 0.1; 
    let offset_y = t * 0.05; 
    let cloud_noise_value = uniforms.cloud_noise.get_noise_2d(
        (x * zoom + ox + t + offset_x), 
        (y * zoom + oy + offset_y + t)
    );

    let land_color_1 = Color::from_float(0.1, 0.3, 0.0); // Verde tóxico 1
    let land_color_2 = Color::from_float(0.2, 0.5, 0.1); // Verde tóxico 2

    let cloud_color = Color::from_float(0.9, 0.6, 0.2); // Naranja para la atmósfera
    let atmosphere_color = Color::from_float(0.1, 0.4, 0.3); // Verde azulado para otra capa de atmósfera

    let land_threshold = 0.3;

    let base_color = if base_noise_value > land_threshold {
        let land_intensity = (base_noise_value - land_threshold) / (1.0 - land_threshold);
        if land_intensity < 0.5 {
            land_color_1.lerp(&land_color_2, land_intensity * 2.0)
        } else {
            land_color_2.lerp(&land_color_1, (land_intensity - 0.5) * 2.0)
        }
    } else {
        land_color_1.lerp(&land_color_2, base_noise_value / land_threshold)
    };

    let light_position = Vec3::new(1.0, 1.0, 3.0); 
    let light_dir = normalize(&(light_position - fragment.vertex_position));
    let normal = normalize(&fragment.normal); 
    let diffuse = dot(&normal, &light_dir).max(0.0);

    let lit_color = base_color * (0.1 + 0.9 * diffuse); 

    let cloud_threshold = 0.25; 
    let cloud_opacity = 0.3 + 0.2 * ((time as f32 / 1000.0) * 0.3).sin().abs(); 
    
    if cloud_noise_value > cloud_threshold {
        let cloud_intensity = ((cloud_noise_value - cloud_threshold) / (1.0 - cloud_threshold)).clamp(0.0, 1.0);
        let cloud_layer = cloud_color * (cloud_intensity * cloud_opacity);
        let atmosphere_intensity = (cloud_noise_value * 0.5).clamp(0.0, 1.0);
        let atmosphere_layer = atmosphere_color * atmosphere_intensity;
        (lit_color.blend_add(&cloud_layer).blend_add(&atmosphere_layer), 0)
    } else {
        (lit_color, 0)
    }
}


pub fn ratchet_shader(fragment: &Fragment, uniforms: &Uniforms, time: f32) -> (Color, u32) {
    // Capa 1: Bandas horizontales difuminadas
    let latitude = fragment.vertex_position.y;
    let band_frequency = 8.0;

    // Ruido para distorsionar las bandas
    let band_noise = uniforms.band_noise.get_noise_2d(
        fragment.vertex_position.x * 2.5,
        fragment.vertex_position.y * 2.5,
    );
    let band_noise_intensity = 0.25;

    let band_speed = 0.02; // Velocidad de desplazamiento de las bandas
    let time_offset = time * band_speed; // Desplazamiento en función del tiempo
    let distorted_latitude = latitude + band_noise * band_noise_intensity + time_offset;
    let band_pattern = (distorted_latitude * band_frequency).sin();

    // Paleta de colores morados
    let band_colors = [
        Color::from_hex(0x5a189a), // Morado intenso
        Color::from_hex(0x9d4edd), // Morado claro
        Color::from_hex(0xc77dff), // Lila brillante
        Color::from_hex(0xe0aaff), // Lila pastel
    ];

    // Interpolación suave entre colores
    let normalized_band = (band_pattern + 1.0) / 2.0 * (band_colors.len() as f32 - 1.0);
    let index = normalized_band.floor() as usize;
    let t = normalized_band.fract();
    let color1 = band_colors[index % band_colors.len()];
    let color2 = band_colors[(index + 1) % band_colors.len()];
    let base_color = color1.lerp(&color2, t);

    // Capa 2: Turbulencia con ruido
    let noise_value = uniforms.noise.get_noise_3d(
        fragment.vertex_position.x * 3.0,
        fragment.vertex_position.y * 3.0,
        fragment.vertex_position.z * 3.0,
    );

    let turbulence_intensity = 0.35;
    let turbulence_color = base_color.lerp(&Color::from_hex(0xffffff), noise_value * turbulence_intensity);

    // Variación adicional con tonos morados
    let variation_noise = uniforms.noise.get_noise_3d(
        fragment.vertex_position.x * 2.0,
        fragment.vertex_position.y * 2.0,
        fragment.vertex_position.z * 2.0,
    );

    let deep_purple = Color::from_hex(0x240046); // Morado profundo
    let light_lavender = Color::from_hex(0xdee2ff); // Lavanda ligera
    let variation_color = turbulence_color
        .lerp(&deep_purple, (variation_noise * 0.5).clamp(0.0, 1.0))
        .lerp(&light_lavender, (variation_noise.abs() * 0.3).clamp(0.0, 1.0));

    // Capa 3: Mancha Morada Difuminada
    // No usamos uv, en lugar de eso calculamos la "distancia" desde un centro con las coordenadas del vértice.
    let spot_center = Vec2::new(0.6, 0.4);
    let distance_to_spot = (fragment.vertex_position.xy() - spot_center).norm();

    let spot_noise_value = uniforms.noise.get_noise_2d(
        fragment.vertex_position.x * 18.0,
        fragment.vertex_position.y * 18.0,
    );
    let spot_noise_intensity = spot_noise_value * 0.25;

    let spot_radius = 0.15;
    let spot_edge = 0.1;
    let spot_intensity = smoothstep(
        spot_radius + spot_edge,
        spot_radius - spot_edge,
        distance_to_spot,
    );

    let spot_intensity = (spot_intensity + spot_noise_intensity).clamp(0.0, 1.0);
    let spot_color = Color::from_hex(0x8e44ad); // Morado vibrante
    let final_color = variation_color.lerp(&spot_color, spot_intensity * 0.85);

    // Iluminación
    let light_position = Vec3::new(10.0, 10.0, 20.0);
    let light_direction = (light_position - fragment.vertex_position).normalize();
    let normal = fragment.normal.normalize();
    let diffuse = normal.dot(&light_direction).max(0.0);
    let ambient_intensity = 0.2;
    let ambient_color = final_color * ambient_intensity;
    let lit_color = final_color * diffuse;
    let color_with_lighting = ambient_color + lit_color;

    (color_with_lighting, 0)
}


pub fn ratchet1_shader(fragment: &Fragment, uniforms: &Uniforms, time: u32) -> (Color, u32) {
  let x = fragment.vertex_position.x;
  let y = fragment.vertex_position.y;
  let z = fragment.vertex_position.z;
  let t = time as f32 * 0.001; 

  let noise_value = uniforms.noise.get_noise_3d(x, y + t, z);

  let base_color = Color::from_float(0.9, 0.3, 0.9);

  let intensity = (noise_value * 0.5 + 0.5).clamp(0.0, 1.0);
  let varied_color = base_color * intensity;

  let light_dir = Vec3::new(1.0, 1.0, 1.0).normalize();
  let normal = fragment.normal.normalize(); 
  let diffuse = normal.dot(&light_dir).max(0.0); 
  let ambient = 0.3; 
  let lit_color = varied_color * (ambient + (1.0 - ambient) * diffuse); 

  (lit_color, 0)
}

pub fn sun_shader(time: u32) -> (Color, u32) {
    // Base color del Sol
    let base_color = Color::from_float(1.0, 0.8, 0.3);

    // Factores de ruido para las tormentas solares
    let frequency = 0.03; // Frecuencia de las variaciones
    let noise_r = ((time as f32 * frequency).sin() * 0.5 + 0.5) * 0.3; // Variación en rojo
    let noise_g = ((time as f32 * frequency * 1.3).cos() * 0.5 + 0.5) * 0.2; // Variación en verde
    let noise_b = ((time as f32 * frequency * 1.7).sin() * 0.5 + 0.5) * 0.1; // Variación en azul

    // Patrón dinámico: alterna entre tonos cálidos (amarillos y naranjas)
    let r = (base_color.r as f32 / 255.0 + noise_r).min(1.0);
    let g = (base_color.g as f32 / 255.0 + noise_g).min(1.0);
    let b = (base_color.b as f32 / 255.0 + noise_b).min(1.0);

    // Intensidad de emisión ajustada para simular destellos de tormentas solares
    let emission_base = 150;
    let emission_variation = (50.0 * ((time as f32 * 0.02).cos() * 0.5 + 0.5)) as u32;
    let emission = emission_base + emission_variation;

    // Crear el color final
    let dynamic_color = Color::from_float(r, g, b);

    (dynamic_color, emission)
}

pub fn moon_shader(fragment: &Fragment, uniforms: &Uniforms) -> (Color, u32) {
    let (base_color, detail_color) = match uniforms.current_shader {
        1 => (Color::from_float(0.1, 0.2, 0.4), Color::from_float(0.05, 0.05, 0.1)), // Azul oscuro
        2 => (Color::from_float(0.4, 0.4, 0.4), Color::from_float(0.2, 0.2, 0.2)), // Gris medio
        3 => (Color::from_float(0.2, 0.2, 0.2), Color::from_float(0.05, 0.05, 0.05)), // Gris oscuro
        _ => (Color::from_float(0.8, 0.8, 0.8), Color::from_float(0.3, 0.3, 0.3)),    // Por defecto: tonos claros
    };
    let noise_value = uniforms.noise.get_noise_2d(fragment.vertex_position.x, fragment.vertex_position.y);
    let normalized_noise = (noise_value + 1.0) * 0.5; 
    let surface_variation = base_color.lerp(&detail_color, normalized_noise.clamp(0.0, 1.0));
    let light_position = Vec3::new(10.0, 10.0, 10.0); // Fuente de luz
    let light_direction = (light_position - fragment.vertex_position).normalize();
    let normal = fragment.normal.normalize();
    let diffuse = normal.dot(&light_direction).max(0.0);
    (surface_variation * (0.2 + 0.6 * diffuse), 0)
}


fn kenshi_shader(fragment: &Fragment, uniforms: &Uniforms, time: u32) -> (Color, u32) {
  let zoom = 100.0;  
  let ox = 100.0; 
  let oy = 100.0;
  let x = fragment.vertex_position.x;
  let y = fragment.vertex_position.y;
  let t = time as f32 * 0.1;

  let base_noise_value = uniforms.noise.get_noise_2d(x, y);
  let offset_x = t * 0.1; 
  let offset_y = t * 0.05; 
  let cloud_noise_value = uniforms.cloud_noise.get_noise_2d(
      (x * zoom + ox +t + offset_x), (y * zoom + oy + offset_y + t)
  );


  let water_color_1 = Color::from_float(0.0, 0.1, 0.6); 
  let water_color_2 = Color::from_float(0.0, 0.3, 0.7);
  let land_color_1 = Color::from_float(0.2, 0.4, 0.0); 
  let land_color_2 = Color::from_float(0.6, 0.5, 0.2);
  let land_color_3 = Color::from_float(0.4, 0.3, 0.1);
  let cloud_color = Color::from_float(0.9, 0.9, 0.9); 

  let land_threshold = 0.3; 

  let base_color = if base_noise_value > land_threshold {
    let land_intensity = (base_noise_value - land_threshold) / (1.0 - land_threshold);
    if land_intensity < 0.5 {
        land_color_1.lerp(&land_color_2, land_intensity * 2.0)
    } else {
        land_color_2.lerp(&land_color_3, (land_intensity - 0.5) * 2.0)
    }
    } else {
    water_color_1.lerp(&water_color_2, base_noise_value / land_threshold)
    };

  let light_position = Vec3::new(1.0, 1.0, 3.0); 
  let light_dir = normalize(&(light_position - fragment.vertex_position));
  let normal = normalize(&fragment.normal); 
  let diffuse = dot(&normal, &light_dir).max(0.0);

  let lit_color = base_color * (0.1 + 0.9 * diffuse); 

  let cloud_threshold = 0.25; 
  let cloud_opacity = 0.3 + 0.2 * ((time as f32 / 1000.0) * 0.3).sin().abs(); 
  if cloud_noise_value > cloud_threshold {
      let cloud_intensity = ((cloud_noise_value - cloud_threshold) / (1.0 - cloud_threshold)).clamp(0.0, 1.0);
      (lit_color.blend_add(&(cloud_color * (cloud_intensity * cloud_opacity))), 0)
  } else {
      (lit_color, 0)
  }
}
