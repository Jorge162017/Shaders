use nalgebra_glm::{Vec3, Vec4, Mat3, mat4_to_mat3};
use crate::vertex::Vertex;
use crate::Uniforms;
use crate::fragment::Fragment;
use crate::color::Color;

pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
    let position = Vec4::new(
        vertex.position.x,
        vertex.position.y,
        vertex.position.z,
        1.0
    );

    let transformed = uniforms.projection_matrix * uniforms.view_matrix * uniforms.model_matrix * position;

    let w = transformed.w;
    let transformed_position = Vec4::new(
        transformed.x / w,
        transformed.y / w,
        transformed.z / w,
        1.0
    );

    let screen_position = uniforms.viewport_matrix * transformed_position;

    let model_mat3 = mat4_to_mat3(&uniforms.model_matrix);
    let normal_matrix = model_mat3.transpose().try_inverse().unwrap_or(Mat3::identity());

    let transformed_normal = normal_matrix * vertex.normal;

    Vertex {
        position: vertex.position,
        normal: vertex.normal,
        tex_coords: vertex.tex_coords,
        color: vertex.color,
        transformed_position: Vec3::new(screen_position.x, screen_position.y, screen_position.z),
        transformed_normal: transformed_normal
    }
}

pub fn fragment_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    if uniforms.is_emissive {
        return emissive_shader(fragment, uniforms);  // Activa el shader emisivo
    }
    fragment.color  // Devolver color normal si no es emisivo
}

/// Shader para material emisivo
fn emissive_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let intensity = fragment.intensity;

    // Color base del material, verde más intenso
    let base_color = Color::new(50, 255, 50); // Verde brillante

    // Color de emisión aún más brillante, verde intenso
    let emission_color = Color::new(0, 255, 0) * (intensity * 2.0); // Mayor intensidad de emisión

    // Mezclamos el color base y el color de emisión
    let final_color = base_color.lerp(&emission_color, 0.9); // Aumentamos el peso del color de emisión

    final_color
}
