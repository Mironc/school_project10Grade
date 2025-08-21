use specs::{Read, ReadStorage, System, WriteStorage};

use crate::camera::{Camera, MainCamera};
use graphics::objects::{
    buffers::{Framebuffer, FramebufferAttachment},
    shader::{Shader, ShaderType, SubShader},
    texture::Filter,
};
use graphics::resize_event::ResizeEvent;

use graphics::utils::{FULLSCREENPASS_VERTEX_SHADER};

pub struct CompositionSystem {
    shader: Shader,
}
impl CompositionSystem {
    pub fn new() -> Self {
        let shader = Shader::new([
            SubShader::new(include_str!("shaders/copy.glsl"), ShaderType::Fragment),
            *FULLSCREENPASS_VERTEX_SHADER,
        ]);
        Self { shader }
    }
}
impl<'a> System<'a> for CompositionSystem {
    type SystemData = (
        Read<'a, MainCamera>,
        WriteStorage<'a, Camera>,
        Read<'a, ResizeEvent>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (maincamera, camera, resize_event) = data;
        let mut front_buffer = Framebuffer::default();
        resize_event.viewport().set_gl_viewport();
        front_buffer.draw_bind();
        let mut shader = &self.shader;
        shader.bind();
        shader.set_texture2d(
            "color",
            &maincamera
                .get(&camera)
                .unwrap()
                .render_image
                .attachment_texture(FramebufferAttachment::Color(0))
                .expect("No color texture in for maincamera"),
            0,
        );
        front_buffer.blit_with(&shader);
        /*
        {
            let this = &maincamera.framebuffer();
            let other: &Framebuffer = &front_buffer;
            unsafe {
                this.read_bind();
                other.draw_bind();
            }
            let filter = Filter::Linear;
            unsafe {
                gl::BlitFramebuffer(
                    0,
                    0,
                    this.viewport().width(),
                    this.viewport().height(),
                    0,
                    0,
                    resize_event.viewport().width(),
                    resize_event.viewport().height(),
                    gl::COLOR_BUFFER_BIT,
                    filter.to_param(),
                );
            }
        }; */
    }
}
