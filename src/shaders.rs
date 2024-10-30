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
    basketball_shader(fragment)
}

/// Shader para simular una pelota de baloncesto
fn basketball_shader(fragment: &Fragment) -> Color {
    let position = fragment.vertex_position;

    // Color base de la pelota (marrón-naranja)
    let base_color = Color::new(186, 101, 49); // Color marrón-naranja que asemeja una pelota de baloncesto

    // Color de las líneas
    let line_color = Color::new(0, 0, 0); // Negro para las líneas

    // Ajustamos el grosor de las líneas
    let line_thickness = 0.02; // Más pequeño para líneas más delgadas

    // Calculamos patrones para simular las líneas de una pelota de baloncesto
    let u = position.x.abs() % 0.3 < line_thickness; // Línea vertical (central)
    let v = position.y.abs() % 0.3 < line_thickness; // Línea horizontal (diagonal)

    // Combinamos ambas condiciones para crear un patrón de líneas
    if u || v {
        line_color // Retornamos el color de la línea si estamos en una zona de línea
    } else {
        base_color // Retornamos el color marrón-naranja si no es una línea
    }
}
