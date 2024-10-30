pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
    pub zbuffer: Vec<f32>,
    pub emission_buffer: Vec<u32>, // Nuevo buffer para la emisión
    background_color: u32,
    current_color: u32,
    current_emission_color: u32, // Color de emisión actual
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Framebuffer {
            width,
            height,
            buffer: vec![0; width * height],
            zbuffer: vec![f32::INFINITY; width * height],
            emission_buffer: vec![0; width * height], // Inicializamos el buffer de emisión
            background_color: 0x000000,
            current_color: 0xFFFFFF,
            current_emission_color: 0x000000, // Emisión predeterminada (apagado)
        }
    }

    pub fn clear(&mut self) {
        for pixel in self.buffer.iter_mut() {
            *pixel = self.background_color;
        }
        for depth in self.zbuffer.iter_mut() {
            *depth = f32::INFINITY;
        }
        for emission in self.emission_buffer.iter_mut() {
            *emission = 0; // Limpiamos el buffer de emisión
        }
    }

    pub fn point(&mut self, x: usize, y: usize, depth: f32) {
        if x < self.width && y < self.height {
            let index = y * self.width + x;

            if self.zbuffer[index] > depth {
                self.buffer[index] = self.current_color;
                self.emission_buffer[index] = self.current_emission_color; // Escribimos en el buffer de emisión
                self.zbuffer[index] = depth;
            }
        }
    }

    pub fn set_background_color(&mut self, color: u32) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: u32) {
        self.current_color = color;
    }

    pub fn set_emission_color(&mut self, color: u32) { // Método para configurar el color de emisión
        self.current_emission_color = color;
    }

    pub fn apply_emission(&mut self) { // Método de postprocesamiento
        for i in 0..self.buffer.len() {
            let base_color = self.buffer[i];
            let emission_color = self.emission_buffer[i];

            // Mezclamos el color base y el color de emisión
            self.buffer[i] = self.mix_colors(base_color, emission_color);
        }
    }

    fn mix_colors(&self, color1: u32, color2: u32) -> u32 {
        // Método simple para mezclar dos colores (simulación de brillo)
        let r1 = (color1 >> 16) & 0xFF;
        let g1 = (color1 >> 8) & 0xFF;
        let b1 = color1 & 0xFF;

        let r2 = (color2 >> 16) & 0xFF;
        let g2 = (color2 >> 8) & 0xFF;
        let b2 = color2 & 0xFF;

        let r = (r1 + r2).min(255);
        let g = (g1 + g2).min(255);
        let b = (b1 + b2).min(255);

        (r << 16) | (g << 8) | b
    }
}
