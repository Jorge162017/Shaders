use nalgebra_glm::{Vec3, Vec4, Mat3, dot, mat4_to_mat3};
use crate::vertex::Vertex;
use crate::Uniforms;
use crate::fragment::Fragment;
use crate::color::Color;
use rand::Rng;
use rand::SeedableRng;
use rand::rngs::StdRng;

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
    uranus_shader(fragment, uniforms)
}

/// Shader para Urano: simula su característico color azul
fn uranus_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    // Definir los colores de Urano
    let dark_blue = Color::new(0, 34, 102);  // Azul oscuro
    let light_blue = Color::new(173, 216, 230); // Azul claro

    // Interpolación para crear un efecto de gradiente basado en la posición Y del fragmento
    let factor = fragment.vertex_position.y.clamp(-1.0, 1.0) * 0.5 + 0.5;
    let uranus_color = dark_blue.lerp(&light_blue, factor as f32);

    uranus_color * fragment.intensity
}

/// Shader opcional para anillos de Urano
fn rings_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let ring_radius = 1.2; // Ajustar el radio de los anillos
    let thickness = 0.05;  // Ajustar el grosor de los anillos

    let distance = fragment.vertex_position.x.hypot(fragment.vertex_position.z);

    if distance > ring_radius - thickness && distance < ring_radius + thickness {
        // Color de los anillos
        return Color::new(200, 200, 200);  // Gris claro
    }

    // Si no es parte de los anillos, devolver el color original
    fragment.color
}

fn black_and_white(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let seed = uniforms.time as f32 * fragment.vertex_position.y * fragment.vertex_position.x;
  
    let mut rng = StdRng::seed_from_u64(seed.abs() as u64);
  
    let random_number = rng.gen_range(0..=100);
  
    let black_or_white = if random_number < 50 {
        Color::new(0, 0, 0)
    } else {
        Color::new(255, 255, 255)
    };
  
    black_or_white * fragment.intensity
}

fn cloud_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let zoom = 100.0;  // Ajusta el zoom para controlar la escala de las nubes
    let ox = 100.0;    // Offset en el eje x del mapa de ruido
    let oy = 100.0;
    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;
    let t = uniforms.time as f32 * 0.5;

    let noise_value = uniforms.noise.get_noise_2d(x * zoom + ox + t, y * zoom + oy);

    // Umbral de nubes y colores
    let cloud_threshold = 0.5; // Ajusta este valor para cambiar la densidad de las nubes
    let cloud_color = Color::new(255, 255, 255); // Blanco para las nubes
    let sky_color = Color::new(30, 97, 145);     // Azul del cielo

    // Determinar si el píxel es parte de una nube o del cielo
    let noise_color = if noise_value > cloud_threshold {
        cloud_color
    } else {
        sky_color
    };

    noise_color * fragment.intensity
}

fn lava_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    // Colores base para el efecto de lava
    let bright_color = Color::new(255, 240, 0); // Naranja brillante (similar a la lava)
    let dark_color = Color::new(130, 20, 0);    // Rojo oscuro

    // Obtener la posición del fragmento
    let position = Vec3::new(
        fragment.vertex_position.x,
        fragment.vertex_position.y,
        fragment.depth
    );

    // Frecuencia base y amplitud para el efecto pulsante
    let base_frequency = 0.2;
    let pulsate_amplitude = 0.5;
    let t = uniforms.time as f32 * 0.01;

    // Efecto pulsante en el eje Z para cambiar el tamaño de las manchas
    let pulsate = (t * base_frequency).sin() * pulsate_amplitude;

    // Aplicar ruido a las coordenadas con pulsación en el eje Z
    let zoom = 1000.0; // Factor de zoom constante
    let noise_value1 = uniforms.noise.get_noise_3d(
        position.x * zoom,
        position.y * zoom,
        (position.z + pulsate) * zoom
    );
    let noise_value2 = uniforms.noise.get_noise_3d(
        (position.x + 1000.0) * zoom,
        (position.y + 1000.0) * zoom,
        (position.z + 1000.0 + pulsate) * zoom
    );
    let noise_value = (noise_value1 + noise_value2) * 0.5;  // Promedio del ruido para transiciones suaves

    // Mezcla de colores según el valor del ruido
    let color = dark_color.lerp(&bright_color, noise_value);

    color * fragment.intensity
}
