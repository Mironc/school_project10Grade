use crate::objects::buffers::Framebuffer;
pub enum RenderTarget{
    Texture(Option<Framebuffer>),
    MainCameraCompatible,
}