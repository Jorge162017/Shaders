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

pub fn fragment_shader(fragment: &Fragment, uniforms: &Uniforms, is_sun: bool) -> Color {
    if is_sun {
        return sun_shader(fragment, uniforms);  // Aplicar shader del Sol
    }
    fragment.color  // En este caso, no estamos procesando otros cuerpos celestes
}

/// Shader para el Sol: emite un color brillante (amarillo/naranja)
fn sun_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    // Definir colores cálidos para el Sol
    let center_color = Color::new(255, 204, 0);  // Amarillo brillante
    let edge_color = Color::new(255, 69, 0);     // Naranja más oscuro

    // Efecto de emisión: hacemos que el color dependa de la posición en el Sol
    let factor = fragment.vertex_position.y.clamp(-1.0, 1.0) * 0.5 + 0.5;

    // Interpolación de colores entre el centro y los bordes
    let sun_color = center_color.lerp(&edge_color, factor as f32);

    // Aumentar la intensidad para darle un efecto de emisión brillante
    sun_color * (fragment.intensity * 2.0).clamp(0.0, 1.0)
}
