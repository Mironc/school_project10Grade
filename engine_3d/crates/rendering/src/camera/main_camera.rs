use specs::{Entity, WriteStorage};


use super::Camera;
pub struct MainCamera {
    main_camera: Option<Entity>,
    //framebuffer: Framebuffer,
}
impl MainCamera {
    pub fn id(&self) -> Option<Entity> {
        self.main_camera
    }
    pub fn get_mut<'a>(&self, camera: &'a mut WriteStorage<'_, Camera>) -> Option<&'a mut Camera> {
        camera.get_mut(self.main_camera.unwrap())
    }
    pub fn get<'a>(&self, camera: &'a WriteStorage<'a, Camera>) -> Option<&'a Camera> {
        camera.get(self.main_camera.unwrap())
    }
    pub fn set(&mut self, main_camera: Entity) {
        self.main_camera = Some(main_camera)
    }
}
impl Default for MainCamera {
    fn default() -> Self {
        /* let mut framebuffer = Framebuffer::new(Viewport::new(0, 0, 1, 1));
        let _ = framebuffer.create_attachment(
            FramebufferAttachment::DepthStencil,
            Texture2DBuilder::new()
                .filter(Filter::Nearest)
                .wrap(TextureWrap::ClampToEdge)
                .internal_format(TextureFormat::Depth32FStencil8)
                .texture_format(TextureFormat::DepthStencilComponent)
                .texture_type(TextureType::Float32UnsignedInt8),
        );
        let _ = framebuffer.create_attachment(
            FramebufferAttachment::Color(0),
            Texture2DBuilder::new()
                .filter(Filter::Nearest)
                .internal_format(TextureFormat::RGBA)
                .texture_type(TextureType::UnsignedByte),
        ); */
        Self {
            main_camera: None,
            //framebuffer,
        }
    }
}
