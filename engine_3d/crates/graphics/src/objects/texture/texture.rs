use std::{cell::{Ref, RefCell}, marker::PhantomData, ops::{Deref, DerefMut}, rc::Rc, sync::Arc};

use image::{DynamicImage, RgbaImage};

use crate::objects::texture::{texture_handle::TextureHandle, texture_trait::{Texture1DTrait, Texture2DTrait, TextureTrait}, texture_type::{Tex2D, TextureTypeTrait}, Filter};


///Wrapper around TextureHandle
pub struct Texture<T: TextureTypeTrait> {
    handle: Arc<RefCell<TextureHandle<T>>>,
}
impl<T> Texture<T>
where
    T: TextureTypeTrait,
{
    pub fn new() -> Self {
        Self {
            handle: TextureHandle::new(),
        }
    }
    /// gives access to texture handle, in order to give info about texture
    /// 
    /// SAFETY: Do not call until all *inner_mut()* calls aren't dropped, but you may have unlimited instances of not mutable refs  
    pub fn inner(&self) -> impl Deref<Target = TextureHandle<T>> + '_ {
        self.handle.borrow()
    }
    /// gives access to texture handle, in order to do some manipulations with texture
    /// 
    /// SAFETY: Do not call until previous call is dropped, or you'll get panic
    pub fn inner_mut(&mut self) -> impl DerefMut<Target = TextureHandle<T>> + '_{
        (*self.handle).borrow_mut()
    }
}
impl Texture<Tex2D>
{
    pub fn white(size:(u32,u32)) -> Self {
        let mut image = RgbaImage::new(size.0, size.1);
        image.fill(255);
        let mut tex = Self::default();
        tex.inner_mut().set_image(super::TextureFormat::RGBA8, image::DynamicImage::ImageRgba8(image));
        tex
    }
    pub fn black(size:(u32,u32)) -> Self {
        let mut image = RgbaImage::new(size.0, size.1);
        image.fill(0);
        let mut tex = Self::default();
        tex.inner_mut().set_image(super::TextureFormat::RGBA8, image::DynamicImage::ImageRgba8(image));
        tex
    }
}
impl<T> Default for Texture<T>
where
    T: TextureTypeTrait,
{
    fn default() -> Self {
        Self::new()
    }
}
fn test() {
    let mut texture = Texture::<Tex2D>::new();
    let inner = texture.inner_mut().set_mag_filter(Filter::Linear);
}