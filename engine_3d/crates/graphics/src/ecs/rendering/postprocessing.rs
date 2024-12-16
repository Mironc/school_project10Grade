use std::default;

use glam::Vec3;

use crate::{depth, face_culling, objects::{
    buffers::Framebuffer,
    shader::{Shader, ShaderType, SubShader},
    texture::Texture2D,
    viewport::Viewport,
}};

use super::FULLSCREENPASS_VERTEX_SHADER;

pub trait PostProcessing {
    fn apply_to(&mut self,framebuffer: &Framebuffer, texture: Texture2D);
    fn resize(&mut self, viewport: Viewport);
}
pub struct SimplePostProcessing {
    gamma: f32,
    brightness:f32,
    contrast:f32,
    midpoint:f32,
    exposure:f32,
    saturation:f32,
    simple_color_correction: Shader,
}
impl SimplePostProcessing {
    pub fn new(gamma: f32,exposure:f32,contrast:f32,brightness:f32,midpoint:f32,saturation:f32, _viewport: Viewport) -> Self {

        let uber_post = Shader::new([
            *FULLSCREENPASS_VERTEX_SHADER,
            SubShader::new(include_str!("./shaders/color_correction.glsl"), ShaderType::Fragment),
        ]);

        Self {
            gamma,
            simple_color_correction: uber_post,
            brightness,
            contrast,
            midpoint,
            saturation,
            exposure,
            ..Default::default()
        }
    }
}
impl Default for SimplePostProcessing {
    fn default() -> Self {
        Self{
            gamma: 1.,
            brightness: 0.,
            contrast: 1.,
            midpoint: 0.5,
            exposure: 1.0,
            saturation:1.0,
            simple_color_correction:Shader::new([
            *FULLSCREENPASS_VERTEX_SHADER,
            SubShader::new(include_str!("./shaders/color_correction.glsl"), ShaderType::Fragment),
        ])
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
        self.simple_color_correction.set_f32("midpoint", self.midpoint);
        self.simple_color_correction.set_texture2d("color", &texture, 0);
        self.simple_color_correction.set_f32("saturation", self.saturation);
        self.simple_color_correction.set_f32("exposure", self.exposure);

        framebuffer.blit_with(&self.simple_color_correction);
    }
    
    fn resize(&mut self, viewport: Viewport) {

    }
}
