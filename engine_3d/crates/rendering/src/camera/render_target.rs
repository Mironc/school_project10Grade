use graphics::objects::buffers::Framebuffer;
pub enum RenderTarget{
    Texture(Framebuffer),
    MainCamera,
}