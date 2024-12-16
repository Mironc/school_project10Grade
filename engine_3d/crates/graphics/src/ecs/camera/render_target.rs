use crate::objects::buffers::Framebuffer;
pub enum RenderTarget{
    Texture(Framebuffer),
    MainCameraCompatible,
}