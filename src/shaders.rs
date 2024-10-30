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
    if uniforms.is_mars {
        return mars_shader(fragment, uniforms);  // Shader específico para Marte
    } else if uniforms.is_moon {
        return moon_shader(fragment, uniforms);  // Shader específico para las lunas
    }
    fragment.color  // Devolver color por defecto si no es Marte ni una luna
}

/// Shader para Marte: simula una superficie rojiza con tonos variados
fn mars_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let zoom = 15.0;
    let ox = 0.0;
    let oy = 0.0;

    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;
    let z = fragment.vertex_position.z;

    let noise_value = uniforms.noise.get_noise_3d(
        (x + ox) * zoom,
        (y + oy) * zoom,
        z * zoom
    );

    // Colores ajustados para Marte, con tonos más rojizos
    let deep_red = Color::new(139, 0, 0);       // Rojo profundo
    let reddish_brown = Color::new(165, 42, 42);  // Marrón rojizo
    let lighter_red = Color::new(178, 34, 34);   // Rojo más claro

    // Interpolación para suavizar la transición entre colores
    let final_color = if noise_value < -0.2 {
        deep_red
    } else if noise_value < 0.2 {
        // Interpolación entre rojo profundo y marrón rojizo
        deep_red.lerp(&reddish_brown, (noise_value + 0.2) / 0.4)
    } else {
        // Interpolación entre marrón rojizo y rojo claro
        reddish_brown.lerp(&lighter_red, (noise_value - 0.2) / 0.8)
    };

    final_color * fragment.intensity
}

/// Shader para las lunas: simple gris, sin mucho detalle
fn moon_shader(fragment: &Fragment, _uniforms: &Uniforms) -> Color {
    let base_gray = Color::new(169, 169, 169); // Gris claro para las lunas
    base_gray * fragment.intensity
}
