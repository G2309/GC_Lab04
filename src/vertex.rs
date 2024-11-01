use nalgebra_glm::{Mat4, Vec2, Vec3, Vec4};

pub struct Vertex {
    pub position: Vec3,
    pub normal: Vec3,
    pub texcoord: Vec2,
}

pub struct Uniforms {
    pub model_matrix: Mat4,
    pub view_matrix: Mat4,
    pub projection_matrix: Mat4,
    pub light_position: Vec3,
    pub camera_position: Vec3,
}

pub struct Fragment {
    pub position: Vec3,
    pub color: (u8, u8, u8), 
    pub depth: f32,
}

pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vec3 {
    let vertex_position_4d = Vec4::new(vertex.position.x, vertex.position.y, vertex.position.z, 1.0);

    let world_position = uniforms.model_matrix * vertex_position_4d;
    let view_position = uniforms.view_matrix * world_position;
    let projected_position = uniforms.projection_matrix * view_position;

    // Convertir el resultado de vuelta a Vec3 dividiendo por el componente w
    let screen_position = projected_position.xyz() / projected_position.w;

    screen_position
}

pub fn rasterize_triangle(v0: Vec3, v1: Vec3, v2: Vec3, color: (u8, u8, u8)) -> Vec<Fragment> {
    let mut fragments = Vec::new();
    for x in v0.x.min(v1.x).min(v2.x) as i32..=v0.x.max(v1.x).max(v2.x) as i32 {
        for y in v0.y.min(v1.y).min(v2.y) as i32..=v0.y.max(v1.y).max(v2.y) as i32 {
            let position = Vec3::new(x as f32, y as f32, 0.0);
            let depth = (v0.z + v1.z + v2.z) / 3.0;
            fragments.push(Fragment {
                position,
                color,
                depth,
            });
        }
    }
    fragments
}

fn fragment_shader(fragment: &Fragment, uniforms: &Uniforms) -> (u8, u8, u8) {
    fragment.color
}

pub fn render_pipeline(vertices: Vec<Vertex>, indices: Vec<u32>, uniforms: &Uniforms) -> Vec<Fragment> {
    let mut fragments = Vec::new();
    for triangle in indices.chunks(3) {
        let v0 = vertex_shader(&vertices[triangle[0] as usize], uniforms);
        let v1 = vertex_shader(&vertices[triangle[1] as usize], uniforms);
        let v2 = vertex_shader(&vertices[triangle[2] as usize], uniforms);
        let triangle_fragments = rasterize_triangle(v0, v1, v2, (255, 0, 0));
        for mut fragment in triangle_fragments {
            let color = fragment_shader(&fragment, uniforms);
            fragment.color = color;
            fragments.push(fragment);
        }
    }
    fragments
}

impl Vertex {
    pub fn new(position: Vec3, normal: Vec3, texcoord: Vec2) -> Self {
        Vertex {
            position,
            normal,
            texcoord,
        }
    }
}

