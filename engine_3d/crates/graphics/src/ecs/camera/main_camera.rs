use specs::{Entities, Entity, WriteStorage};

use crate::objects::{
    buffers::{Framebuffer, FramebufferAttachment},
    texture::{Filter, Texture2DBuilder, TextureFormat},
    viewport::Viewport,
};

use super::Camera;
pub struct MainCamera {
    main_camera: Option<Entity>,
    main_framebuffer: Framebuffer,
}
impl MainCamera {
    pub fn new(main_camera: Entity) -> Self {
        let mut main_framebuffer = Framebuffer::new(Viewport::new(0, 0, 1, 1));
        main_framebuffer.create_attachment(
            FramebufferAttachment::Color(0),
            Texture2DBuilder::new()
                .filter(Filter::Nearest)
                .internal_format(TextureFormat::RGBA16F),
        );
        main_framebuffer.create_attachment(
            FramebufferAttachment::DepthStencil,
            Texture2DBuilder::new()
                .filter(Filter::Nearest)
                .internal_format(TextureFormat::Depth24Stencil8)
                .texture_format(TextureFormat::DepthStencilComponent),
        );
        Self {
            main_camera: Some(main_camera),
            main_framebuffer,
        }
    }
    pub fn get_maincamera<'a>(
        &self,
        camera: &'a mut WriteStorage<'a, Camera>,
    ) -> Option<&'a mut Camera> {
        let camera = camera.get_mut(self.main_camera.unwrap()).unwrap();
        Some(camera)
    }
    pub fn framebuffer(&self) -> &Framebuffer {
        &self.main_framebuffer
    }
    pub fn set(&mut self, main_camera: Entity) {
        self.main_camera = Some(main_camera)
    }
    pub fn resize(&mut self, viewport: Viewport) {
        self.main_framebuffer = self.main_framebuffer.resize(viewport).unwrap()
    }
}
impl Default for MainCamera {
    fn default() -> Self {
        let mut main_framebuffer = Framebuffer::new(Viewport::new(1, 1, 0, 0));
        main_framebuffer.create_attachment(
            FramebufferAttachment::Color(0),
            Texture2DBuilder::new()
                .filter(Filter::Nearest)
                .internal_format(TextureFormat::RGBA16F),
        );
        main_framebuffer.create_attachment(
            FramebufferAttachment::DepthStencil,
            Texture2DBuilder::new()
                .filter(Filter::Nearest)
                .internal_format(TextureFormat::Depth24Stencil8)
                .texture_format(TextureFormat::DepthStencilComponent),
        );
        Self {
            main_camera: None,
            main_framebuffer,
        }
    }
}
