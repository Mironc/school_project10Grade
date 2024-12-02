use crate::{depth, face_culling, objects::{
    buffers::Framebuffer,
    shader::{Shader, ShaderType, SubShader},
    texture::Texture2D,
    viewport::Viewport,
}};

use super::BLIT_VERTEX_SHADER;

pub trait PostProcessing {
    fn apply_to(&mut self,framebuffer: &Framebuffer, texture: Texture2D);
    fn resize(&mut self, viewport: Viewport);
}
pub struct SimplePostProcessing {
    gamma: f32,
    brightness:f32,
    contrast:f32,
    simple_color_correction: Shader,
}
impl SimplePostProcessing {
    pub fn new(gamma: f32,contrast:f32,brightness:f32, _viewport: Viewport) -> Self {

        let uber_post = Shader::new([
            *BLIT_VERTEX_SHADER,
            SubShader::new(include_str!("color_correction.glsl"), ShaderType::Fragment),
        ]);

        Self {
            gamma,
            simple_color_correction: uber_post,
            brightness,
            contrast,
        }
    }
}
impl PostProcessing for SimplePostProcessing {
    fn apply_to(&mut self,framebuffer:&Framebuffer, texture: Texture2D){
        depth::disable();
        face_culling::disable();
        self.simple_color_correction.bind();
        self.simple_color_correction.set_f32("gamma", self.gamma);
        self.simple_color_correction.set_f32("contrast", self.contrast);
        self.simple_color_correction.set_f32("brightness", self.brightness);
        self.simple_color_correction.set_texture2d("color", &texture, 0);

        framebuffer.blit_with(&self.simple_color_correction);
    }

    fn resize(&mut self, viewport: Viewport) {

    }
}
