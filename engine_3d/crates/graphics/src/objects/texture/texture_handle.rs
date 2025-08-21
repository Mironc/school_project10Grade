use std::{
    borrow::BorrowMut,
    cell::RefCell,
    marker::PhantomData,
    rc::Rc, sync::Arc,
};

use image::{DynamicImage, EncodableLayout, GenericImageView};

use super::{
    texture_trait::{Texture1DTrait, Texture2DTrait, Texture3DTrait, TextureTrait},
    texture_type::*,
    Filter, TextureDataType, TextureFormat, TextureWrap,
};
/* pub struct Texture<T: TextureTypeTrait> {
    handle: AtomicRefCell<TextureHandle<T>>,
}
impl<T: TextureTypeTrait> Texture<T> {
    pub fn new() -> Self {
        Self {
            handle: TextureHandle::new(),
        }
    }
    fn clone_handle(&self) -> AtomicRefCell<TextureHandle<T>> {
        self.handle.clone()
    }
    pub fn muts(&mut self) {

    }
    pub fn get_handle(&mut self) -> &mut TextureHandle<T> {
        self.handle.get_mut()
    }
} */
//TODO!: Array for everything
///Holds data about texture
///
#[derive(Debug)]
pub struct TextureHandle<T: TextureTypeTrait> {
    id: u32,
    wrap_x: TextureWrap,
    wrap_y: TextureWrap,
    wrap_z: TextureWrap,
    min_filter: Filter,
    mag_filter: Filter,
    texture_type: PhantomData<T>,
    data_type: TextureDataType,
    internal_format: TextureFormat,
    texture_format: TextureFormat,
    height: i32,
    width: i32,
    depth: i32,
}
impl<T: TextureTypeTrait> TextureHandle<T> {
    pub fn new() -> Arc<RefCell<Self>> {
        let mut id = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
        }
        let th = Self {
            id,
            wrap_x: TextureWrap::Repeat,
            wrap_y: TextureWrap::Repeat,
            wrap_z: TextureWrap::Repeat,
            min_filter: Filter::Nearest,
            mag_filter: Filter::Nearest,
            texture_type: PhantomData,
            internal_format: TextureFormat::RGBA,
            texture_format: TextureFormat::RGBA,
            data_type: TextureDataType::UnsignedByte,
            height: 0,
            width: 0,
            depth: 0,
        };
        let shared = Arc::new(RefCell::new(th));
        shared
    }

    pub fn gen_mipmaps(&self) {
        unsafe {
            gl::GenerateMipmap(T::texture_type().into_glenum());
        }
    }
}
impl TextureHandle<Tex1D> {
    pub fn from_array<T>(
        &mut self,
        array: &[T],
        internal_format: TextureFormat,
        texture_format: TextureFormat,
        data_type: TextureDataType,
    ) {
        self.data_type = data_type;
        self.width = array.len() as i32;
        if array.len() == 0 {
            return;
        }
        self.bind();
        unsafe {
            gl::TexImage1D(
                gl::TEXTURE_1D,
                0,
                internal_format.into_glenum() as i32,
                array.len() as i32,
                0,
                texture_format.into_glenum(),
                data_type.into_glenum(),
                array.as_ptr() as *const _,
            );
        }
    }
    pub fn finalize(
        &mut self,
        internal_format: TextureFormat,
        texture_format: TextureFormat,
        data_type: TextureDataType,
        width: i32,
    ) {
        self.internal_format = internal_format;
        self.texture_format = texture_format;
        self.width = width;
        self.data_type = data_type;
        unsafe {
            gl::TexImage1D(
                gl::TEXTURE_1D,
                0,
                internal_format.into_glenum() as i32,
                width,
                0,
                texture_format.into_glenum(),
                data_type.into_glenum(),
                std::ptr::null(),
            );
        }
    }
}
impl TextureHandle<Tex2D> {
    pub fn set_image(&mut self, internal_format: TextureFormat, image_source: DynamicImage) {
        self.borrow_mut().internal_format = internal_format;
        self.bind();
        let size = image_source.dimensions();
        unsafe {
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                internal_format.into_glenum() as i32,
                size.0 as i32,
                size.1 as i32,
                0,
                TextureFormat::RGBA.into_glenum(),
                TextureDataType::UnsignedByte.into_glenum(),
                match image_source {
                    DynamicImage::ImageRgba8(img) => img,
                    x => x.to_rgba8(),
                }
                .as_bytes()
                .as_ptr() as *const _,
            );
        }
    }

    pub fn finalize(
        &mut self,
        internal_format: TextureFormat,
        texture_format: TextureFormat,
        texture_type: TextureDataType,
        width: i32,
        height: i32,
    ) {
        self.internal_format = internal_format;
        self.texture_format = texture_format;
        self.data_type = texture_type;
        self.width = width;
        self.height = height;
        self.bind();
        unsafe {
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                internal_format.into_glenum() as i32,
                width,
                height,
                0,
                texture_format.into_glenum(),
                texture_type.into_glenum(),
                std::ptr::null(),
            )
        }
    }
}
impl TextureHandle<CubeMapTexture> {
    pub fn finalize(
        &mut self,
        internal_format: TextureFormat,
        texture_format: TextureFormat,
        data_type: TextureDataType,
        width: i32,
        height: i32,
    ) {
        self.bind();
        self.internal_format = internal_format;
        self.texture_format = texture_format;
        self.data_type = data_type;
        self.width = width;
        for i in 0..6 {
            unsafe {
                gl::TexImage2D(
                    gl::TEXTURE_CUBE_MAP_POSITIVE_X + i,
                    0,
                    internal_format.into_glenum() as i32,
                    width,
                    height,
                    0,
                    texture_format.into_glenum(),
                    data_type.into_glenum(),
                    std::ptr::null(),
                );
            }
        }
    }
    /// The correct order for textures is: Right, Left, Top, Bottom, Front, Back
    pub fn set_images(&self, internal_format: TextureFormat, images: [DynamicImage; 6]) {
        self.bind();
        for i in 0..6 {
            unsafe {
                let (width, height) = images[i as usize].dimensions();
                gl::TexImage2D(
                    gl::TEXTURE_CUBE_MAP_POSITIVE_X + i,
                    0,
                    internal_format.into_glenum() as i32,
                    width as i32,
                    height as i32,
                    0,
                    TextureFormat::RGBA.into_glenum(),
                    TextureDataType::UnsignedByte.into_glenum(),
                    match images.get(i as usize).unwrap() {
                        DynamicImage::ImageRgba8(img) => img.clone(),
                        x => x.to_rgba8(),
                    }
                    .as_bytes()
                    .as_ptr() as *const _,
                );
            }
        }
    }
}

impl<T> Texture1DTrait for TextureHandle<T>
where
    T: D1 + TextureTypeTrait,
{
    fn width(&self) -> i32 {
        self.width
    }
    fn set_texture_wrap_x(&mut self, texture_wrap: TextureWrap) {
        self.wrap_x = texture_wrap;
        unsafe {
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_S,
                texture_wrap.to_param() as i32,
            )
        }
    }
}
impl<T> Texture2DTrait for TextureHandle<T>
where
    T: D2 + TextureTypeTrait,
{
    fn height(&self) -> i32 {
        self.height
    }

    fn wrap_y(&self) -> TextureWrap {
        self.wrap_y
    }

    fn set_texture_wrap_y(&mut self, texture_wrap: TextureWrap) {
        self.wrap_y = texture_wrap;
        unsafe {
            gl::TexParameteri(
                self.texture_type().into_glenum(),
                gl::TEXTURE_WRAP_T,
                texture_wrap.to_param() as i32,
            )
        }
    }
}
impl<T> Texture3DTrait for TextureHandle<T>
where
    T: D3 + TextureTypeTrait,
{
    fn depth(&self) -> i32 {
        self.depth
    }

    fn wrap_z(&self) -> TextureWrap {
        self.wrap_z
    }

    fn set_texture_wrap_z(&mut self, texture_wrap: TextureWrap) {
        self.wrap_z = texture_wrap;
        unsafe {
            gl::TexParameteri(
                self.texture_type().into_glenum(),
                gl::TEXTURE_WRAP_R,
                texture_wrap.to_param() as i32,
            )
        }
    }
}
impl<T> TextureTrait for TextureHandle<T>
where
    T: TextureTypeTrait,
{
    fn texture_type(&self) -> TextureType {
        T::texture_type()
    }

    fn id(&self) -> u32 {
        self.id
    }

    fn internal_format(&self) -> TextureFormat {
        self.internal_format
    }

    fn texture_format(&self) -> TextureFormat {
        self.texture_format
    }

    fn texture_data_type(&self) -> TextureDataType {
        self.data_type
    }

    fn mag_filter(&self) -> Filter {
        self.mag_filter
    }

    fn min_filter(&self) -> Filter {
        self.min_filter
    }

    fn set_min_filter(&mut self, filter: Filter) {
        self.min_filter = filter;
        unsafe {
            gl::TexParameteri(
                self.texture_type().into_glenum(),
                gl::TEXTURE_MIN_FILTER,
                filter.to_param() as i32,
            )
        }
    }

    fn set_mag_filter(&mut self, filter: Filter) {
        self.mag_filter = filter;
        unsafe {
            gl::TexParameteri(
                self.texture_type().into_glenum(),
                gl::TEXTURE_MAG_FILTER,
                filter.to_param() as i32,
            )
        }
    }
}
