use engine_3d::graphics::{utils::FULLSCREENPASS_VERTEX_SHADER, objects::{buffers::Framebuffer, shader::{Shader, ShaderType, SubShader}, texture::Texture2D, viewport::Viewport}};
use engine_3d::post_processing::PostProcessing;


#[derive(Debug, Clone)]
pub struct White {
    shader: Shader,
}
impl White {
    pub fn new() -> Self {
        let shader = Shader::new([
            *FULLSCREENPASS_VERTEX_SHADER,
            SubShader::new(
                include_str!("white_fs.glsl"),
                ShaderType::Fragment,
            ),
        ]);

        Self {
            shader,
            ..Default::default()
        }
    }
}
impl Default for White {
    fn default() -> Self {
        let shader = Shader::new([
            *FULLSCREENPASS_VERTEX_SHADER,
            SubShader::new(
                include_str!("white_fs.glsl"),
                ShaderType::Fragment,
            ),
        ]);
        Self { shader }
    }
}
impl PostProcessing for White {
    fn apply_to(&mut self, framebuffer: &mut Framebuffer, texture: Texture2D) {
        self.shader.set_texture2d("color", &texture, 0);
        framebuffer.blit_with(&self.shader);
    }

    fn resize(&mut self, _viewport: Viewport) {}
}
