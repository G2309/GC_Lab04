use nalgebra_glm::{Vec3, Vec4, dot, normalize};
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
      1 => earth_shader(fragment, uniforms, time),
      6 => urano_shader(fragment, uniforms, time),
      7 => sun_shader(),
      _ => (Color::new(0, 0, 0), 0),
  }
}

pub fn urano_shader(fragment: &Fragment, uniforms: &Uniforms, time: u32) -> (Color, u32) {
  let x = fragment.vertex_position.x;
  let y = fragment.vertex_position.y;
  let z = fragment.vertex_position.z;
  let t = time as f32 * 0.001; 

  let noise_value = uniforms.noise.get_noise_3d(x, y + t, z);

  let base_color = Color::from_float(0.2, 0.5, 0.9);

  let intensity = (noise_value * 0.5 + 0.5).clamp(0.0, 1.0);
  let varied_color = base_color * intensity;

  let light_dir = Vec3::new(1.0, 1.0, 1.0).normalize();
  let normal = fragment.normal.normalize(); 
  let diffuse = normal.dot(&light_dir).max(0.0); 
  let ambient = 0.3; 
  let lit_color = varied_color * (ambient + (1.0 - ambient) * diffuse); 

  (lit_color, 0)
}

pub fn sun_shader() -> (Color, u32) {
  let base_color = Color::from_float(1.0, 0.9, 0.5);
  let emission = 100;

  (base_color, emission)
}

fn earth_shader(fragment: &Fragment, uniforms: &Uniforms, time: u32) -> (Color, u32) {
  let zoom = 100.0;  // to move our values 
  let ox = 100.0; // offset x in the noise map
  let oy = 100.0;
  let x = fragment.vertex_position.x;
  let y = fragment.vertex_position.y;
  let t = time as f32 * 0.1;

  let base_noise_value = uniforms.noise.get_noise_2d(x, y);
  let cloud_noise_value = uniforms.cloud_noise.get_noise_2d(
      x * zoom + ox +t, y * zoom + oy
  );

  let water_color_1 = Color::from_float(0.0, 0.1, 0.6); 
  let water_color_2 = Color::from_float(0.0, 0.3, 0.7);
  let land_color_1 = Color::from_float(0.1, 0.5, 0.0);
  let land_color_2 = Color::from_float(0.2, 0.8, 0.2);
  let cloud_color = Color::from_float(0.9, 0.9, 0.9); 

  let land_threshold = 0.3; // Umbral para tierra

  let base_color = if base_noise_value > land_threshold {
      land_color_1.lerp(&land_color_2, (base_noise_value - land_threshold) / (1.0 - land_threshold))
  } else {
      water_color_1.lerp(&water_color_2, base_noise_value / land_threshold)
  };

  let light_position = Vec3::new(1.0, 1.0, 3.0); 
  let light_dir = normalize(&(light_position - fragment.vertex_position));
  let normal = normalize(&fragment.normal); 
  let diffuse = dot(&normal, &light_dir).max(0.0);

  let lit_color = base_color * (0.1 + 0.9 * diffuse); 

  let cloud_threshold = 0.1; // Umbral para la aparición de nubes
  let cloud_opacity = 0.3 + 0.2 * ((time as f32 / 1000.0) * 0.3).sin().abs(); 
  if cloud_noise_value > cloud_threshold {
      let cloud_intensity = ((cloud_noise_value - cloud_threshold) / (1.0 - cloud_threshold)).clamp(0.0, 1.0);
      (lit_color.blend_add(&(cloud_color * (cloud_intensity * cloud_opacity))), 0)
  } else {
      (lit_color, 0)
  }
}

