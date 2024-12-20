use specs::{Entity, WriteStorage};

use crate::objects::{
    buffers::{Framebuffer, FramebufferAttachment},
    viewport::Viewport,
};

use super::Camera;
pub struct MainCamera {
    main_camera: Option<Entity>,
    viewport: Viewport,
}
impl MainCamera {
    pub fn new(main_camera: Entity,viewport: Viewport) -> Self {
        Self {
            main_camera: Some(main_camera),
            viewport,
        }
    }
    pub fn id(&self) -> Option<Entity> {
        self.main_camera
    }
    pub fn get_mut<'a>(&self, camera: &'a mut WriteStorage<'a, Camera>) -> Option<&'a mut Camera> {
        camera.get_mut(self.main_camera.unwrap())
    }
    pub fn get<'a>(&self, camera: &'a WriteStorage<'a, Camera>) -> Option<&'a Camera> {
        camera.get(self.main_camera.unwrap())
    }
    pub fn set(&mut self, main_camera: Entity) {
        self.main_camera = Some(main_camera)
    }
    pub fn set_viewport(&mut self, viewport: Viewport) {
        self.viewport = viewport;
    }
    pub fn framebuffer(&self) -> Framebuffer{
        Framebuffer::default()
    }
    pub fn viewport(&self) -> Viewport{
        self.viewport
    }
}
impl Default for MainCamera {
    fn default() -> Self {
        Self {
            main_camera: None,
            viewport:Viewport::new(0, 0, 1, 1),
        }
    }
}
