use nalgebra_glm::{Vec3, Vec4, Mat3, mat4_to_mat3};
use crate::vertex::Vertex;
use crate::Uniforms;
use crate::fragment::Fragment;
use crate::color::Color;
use rand::Rng;

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
    pastel_planet_shader(fragment, uniforms)
}

/// Shader para simular el planeta pastel con nubes difusas y tonos intensos
fn pastel_planet_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let position = fragment.vertex_position;

    // Colores base de la superficie del planeta, usando un degradado de rosa intenso a verde-azulado intenso
    let pink = Color::new(255, 105, 180); // Rosa intenso
    let teal = Color::new(0, 206, 209); // Verde-azulado (celeste) intenso

    // Interpolación para el degradado entre rosa y verde-azulado según la posición en Y
    let base_color = pink.lerp(&teal, (position.y + 1.0) / 2.0); // Normalizamos y a rango [0, 1]

    // Color de las nubes
    let cloud_color = Color::new(255, 245, 238); // Blanco cálido para las nubes

    // Generación de nubes difusas en la franja central
    let cloud_density = (position.y * 3.0).sin() * (position.x * 3.0).cos(); // Densidad de nubes
    let noise_factor = cloud_noise(position.x, position.y, uniforms.time as f32 * 0.1);
    let cloud_opacity = ((cloud_density + noise_factor) * 0.6).clamp(0.0, 1.0); // Aumentamos el valor de opacidad

    // Mezcla de color base y nubes según la opacidad calculada
    let final_color = base_color.lerp(&cloud_color, cloud_opacity);

    final_color
}

/// Función para simular ruido en las nubes
fn cloud_noise(x: f32, y: f32, time: f32) -> f32 {
    let mut rng = rand::thread_rng();
    let noise = (x * 10.0 + rng.gen::<f32>()).sin() * (y * 10.0 + rng.gen::<f32>() + time).cos();
    noise.abs()
}
