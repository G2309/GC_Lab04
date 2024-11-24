use nalgebra_glm::{Vec3, Vec4};
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

