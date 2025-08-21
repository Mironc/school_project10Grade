use std::{
    error::Error,
    fmt::Display,
    ptr::NonNull,
    sync::{atomic::AtomicUsize, LazyLock, Mutex},
};
pub mod texture;
pub mod texture_handle;
pub mod texture_trait;
pub mod texture_type;

use image::{DynamicImage, EncodableLayout, GenericImageView, RgbaImage};

use crate::{
    objects::texture::texture_type::TextureTypeTrait, utils::{end_debug_marker, start_debug_marker, COPY_FRAGMENT_SHADER, EMPTY, FULLSCREENPASS_VERTEX_SHADER}
};

use super::{
    buffers::{Framebuffer, FramebufferAttachment},
    shader::{Shader, ShaderType, SubShader},
    viewport::Viewport,
};
/// default filtering is Filter::Repeat and default texture wrap is TextureWrap::Nearest
#[derive(Debug, Clone)]
pub struct Texture2DBuilder {
    texture: Texture2D,
    image: Option<DynamicImage>,
    texture_size: (i32, i32),
    texture_format: TextureFormat,
    internal_format: TextureFormat,
    texture_type: TextureDataType,
}
impl Texture2DBuilder {
    pub fn new() -> Self {
        let texture = Texture2D::new();
        texture.bind();
        Self {
            texture,
            image: None,
            texture_format: TextureFormat::RGBA,
            internal_format: TextureFormat::RGBA,
            texture_size: (0, 0),
            texture_type: TextureDataType::UnsignedByte,
        }
    }
    pub fn texture_format(mut self, texture_format: TextureFormat) -> Self {
        self.texture_format = texture_format;
        self
    }
    pub fn texture_type(mut self, texture_type: TextureDataType) -> Self {
        self.texture_type = texture_type;
        self
    }
    pub fn gen_mipmaps(self) -> Self {
        self.texture.gen_mipmaps();
        self
    }
    ///replaces wrap_x and wrap_y
    pub fn wrap(self, texture_wrap: TextureWrap) -> Self {
        self.wrap_x(texture_wrap).wrap_y(texture_wrap)
    }
    ///replaces min_filter and mag_filter
    pub fn filter(self, filter: Filter) -> Self {
        self.mag_filter(filter).min_filter(filter)
    }
    pub fn wrap_x(mut self, texture_wrap: TextureWrap) -> Self {
        self.texture.set_texture_wrap_x(texture_wrap);
        self
    }
    pub fn wrap_y(mut self, texture_wrap: TextureWrap) -> Self {
        self.texture.set_texture_wrap_y(texture_wrap);
        self
    }
    pub fn mag_filter(mut self, filter: Filter) -> Self {
        self.texture.set_mag_filter(filter);
        self
    }
    pub fn min_filter(mut self, filter: Filter) -> Self {
        self.texture.set_min_filter(filter);
        self
    }
    pub fn internal_format(mut self, texture_format: TextureFormat) -> Self {
        self.internal_format = texture_format;
        self
    }
    pub fn size(mut self, size: (i32, i32)) -> Self {
        self.texture_size = size;
        self
    }
    pub fn image(mut self, image: DynamicImage) -> Self {
        self.image = Some(image);
        self
    }
    pub fn build(mut self) -> Result<Texture2D, BuildError> {
        if self.texture_size == (0, 0) && self.image == None {
            return Err(BuildError {});
        }
        match self.image {
            Some(img) => self.texture.from_image(self.internal_format, img),
            None => self.texture.finalize(
                self.internal_format,
                self.texture_format,
                self.texture_type,
                self.texture_size.0,
                self.texture_size.1,
            ),
        }

        Ok(self.texture)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct BuildError {}
impl Display for BuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Bad texture size or Image not given")
    }
}
impl Error for BuildError {}
static mut DRAWFB: LazyLock<Mutex<Framebuffer>> = LazyLock::new(|| {
    Mutex::new({
        let fb = Framebuffer::new(Viewport::new(0, 0, 1, 1));
        fb
    })
});

#[derive(Debug)]
pub struct Texture2D {
    id: u32,
    counter: NonNull<AtomicUsize>,
    wrap_x: TextureWrap,
    wrap_y: TextureWrap,
    min_filter: Filter,
    mag_filter: Filter,
    data_type: TextureDataType,
    internal_format: TextureFormat,
    texture_format: TextureFormat,
    height: i32,
    width: i32,
}
impl Texture2D {
    pub fn new() -> Self {
        unsafe {
            let mut id = 0;
            gl::GenTextures(1, &mut id);
            Self {
                id,
                counter: NonNull::from(Box::leak(Box::new(AtomicUsize::new(1)))),
                wrap_x: TextureWrap::Repeat,
                wrap_y: TextureWrap::Repeat,
                min_filter: Filter::Nearest,
                mag_filter: Filter::Nearest,
                internal_format: TextureFormat::RGBA,
                texture_format: TextureFormat::RGBA,
                data_type: TextureDataType::UnsignedByte,
                height: 0,
                width: 0,
            }
        }
    }
    pub fn copy_to(&self, other: &Self) {
        start_debug_marker("copy");
        unsafe {
            let mut drawfb = DRAWFB.lock().unwrap();
            drawfb.draw_bind();
            let mut copysh = COPY_FRAGMENT_SHADER.lock().unwrap();
            copysh.set_texture2d("color", self, 0);
            drawfb.add_attachment(
                super::buffers::FramebufferAttachment::Color(0),
                other.clone(),
            );
            let shader: &Shader = &*copysh;
            shader.bind();
            Viewport::new(0, 0, other.width, other.height).set_gl_viewport();
            EMPTY.draw();
        }
        end_debug_marker();
    }
    pub fn dcopy_to(&self, other: &Self) {
        unsafe {
            gl::CopyImageSubData(
                self.id,
                gl::TEXTURE_2D,
                0,
                0,
                0,
                0,
                other.id,
                gl::TEXTURE_2D,
                0,
                0,
                0,
                0,
                self.height,
                self.width,
                0,
            );
        }
    }
    pub fn gen_mipmaps(&self) {
        self.bind();
        unsafe {
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }
    }
    pub fn id(&self) -> u32 {
        self.id
    }
    pub fn bind(&self) {
        unsafe { gl::BindTexture(gl::TEXTURE_2D, self.id) }
    }
    pub fn set_active(i: u32) {
        unsafe { gl::ActiveTexture(gl::TEXTURE0 + i) }
    }

    pub fn internal_format(&self) -> TextureFormat {
        self.internal_format
    }
    pub fn texture_format(&self) -> TextureFormat {
        self.texture_format
    }
    pub fn texture_type(&self) -> TextureDataType {
        self.data_type
    }
    pub fn mag_filter(&self) -> Filter {
        self.mag_filter
    }
    pub fn min_filter(&self) -> Filter {
        self.min_filter
    }
    pub fn wrap_x(&self) -> TextureWrap {
        self.wrap_x
    }
    pub fn wrap_y(&self) -> TextureWrap {
        self.wrap_y
    }
    pub fn width(&self) -> i32 {
        self.width
    }
    pub fn height(&self) -> i32 {
        self.height
    }
    pub fn white() -> Self {
        let mut image = RgbaImage::new(1, 1);
        image.fill(255);
        Texture2DBuilder::new()
            .filter(Filter::Linear)
            .image(DynamicImage::ImageRgba8(image))
            .build()
            .unwrap()
    }
    pub fn black() -> Self {
        let mut image = RgbaImage::new(1, 1);
        image.fill(0);
        Texture2DBuilder::new()
            .image(DynamicImage::ImageRgba8(image))
            .build()
            .unwrap()
    }
    pub fn set_texture_wrap_x(&mut self, texture_wrap: TextureWrap) {
        self.wrap_x = texture_wrap;
        unsafe {
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_S,
                texture_wrap.to_param() as i32,
            )
        }
    }
    pub fn set_texture_wrap_y(&mut self, texture_wrap: TextureWrap) {
        self.wrap_y = texture_wrap;
        unsafe {
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_T,
                texture_wrap.to_param() as i32,
            )
        }
    }
    pub fn set_min_filter(&mut self, filter: Filter) {
        self.min_filter = filter;
        unsafe {
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                filter.to_param() as i32,
            )
        }
    }
    pub fn set_mag_filter(&mut self, filter: Filter) {
        self.mag_filter = filter;
        unsafe {
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MAG_FILTER,
                filter.to_param() as i32,
            )
        }
    }
    pub fn from_image(&mut self, internal_format: TextureFormat, image_source: DynamicImage) {
        self.internal_format = internal_format;
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
impl Default for Texture2D {
    fn default() -> Self {
        Self {
            id: 0,
            wrap_x: TextureWrap::Repeat,
            wrap_y: TextureWrap::Repeat,
            min_filter: Filter::Nearest,
            mag_filter: Filter::Nearest,
            data_type: TextureDataType::Byte,
            internal_format: TextureFormat::RGBA,
            texture_format: TextureFormat::RGBA,
            height: 0,
            width: 0,
            counter: NonNull::from(Box::leak(Box::new(AtomicUsize::new(1)))),
        }
    }
}

impl Clone for Texture2D {
    fn clone(&self) -> Self {
        let new_size = unsafe {
            self.counter
                .as_ref()
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed)
        };
        if new_size > usize::MAX / 2 {
            std::process::abort();
        }
        Self {
            id: self.id.clone(),
            counter: self.counter,
            wrap_x: self.wrap_x.clone(),
            wrap_y: self.wrap_y.clone(),
            min_filter: self.min_filter.clone(),
            mag_filter: self.mag_filter.clone(),
            data_type: self.data_type.clone(),
            internal_format: self.internal_format.clone(),
            texture_format: self.texture_format.clone(),
            height: self.height.clone(),
            width: self.width.clone(),
        }
    }
}
impl Drop for Texture2D {
    fn drop(&mut self) {
        unsafe {
            let sub = self
                .counter
                .as_ref()
                .fetch_sub(1, std::sync::atomic::Ordering::Release);
            if sub != 1 {
                return;
            }
            println!("{} actually dropped", self.id);
            gl::DeleteTextures(1, &mut self.id);
            drop(Box::from_raw(self.counter.as_ptr()));
        }
    }
}

/// Texture wrap is defined behaviour, when you try to get a pixel beyond borders
#[derive(Debug, Clone, Copy)]
pub enum TextureWrap {
    /// Repeats the texture image.
    Repeat,
    /// Gives the closest edge color.
    ClampToEdge,
    /// Same as Repeat, but mirrors the image every iteration.
    MirroredRepeat,
    /// Gives use defined color. TODO: NOT IMPLEMENTED RN.
    ClampToBorder,
}
impl TextureWrap {
    pub fn to_param(&self) -> u32 {
        match self {
            TextureWrap::Repeat => gl::REPEAT,
            TextureWrap::ClampToEdge => gl::CLAMP_TO_EDGE,
            TextureWrap::MirroredRepeat => gl::MIRRORED_REPEAT,
            TextureWrap::ClampToBorder => gl::CLAMP_TO_BORDER,
        }
    }
}
/// Determines the behavior of the color sampling
#[derive(Debug, Clone, Copy)]
pub enum Filter {
    /// Bilinear filter. Gives the interpolated value of neighbouring pixels
    Linear,
    /// Point filter. Gives the nearest to the sample pixel
    Nearest,
    ///
    NearestLinearMipMap,
    ///
    NearestMipMap,
}
impl Filter {
    pub fn to_param(&self) -> u32 {
        match self {
            Filter::Linear => gl::LINEAR,
            Filter::Nearest => gl::NEAREST,
            Filter::NearestMipMap => gl::NEAREST_MIPMAP_NEAREST,
            Filter::NearestLinearMipMap => gl::NEAREST_MIPMAP_LINEAR,
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum TextureDataType {
    Byte,
    Int,
    UnsignedInt,
    UnsignedByte,
    UnsignedInt24_8,
    UnsignedInt5_999,
    Float32UnsignedInt8,
    Float,
    HalfFloat,
}
impl TextureDataType {
    pub fn into_glenum(&self) -> u32 {
        match self {
            TextureDataType::UnsignedInt => gl::UNSIGNED_INT,
            TextureDataType::UnsignedByte => gl::UNSIGNED_BYTE,
            TextureDataType::Float => gl::FLOAT,
            TextureDataType::HalfFloat => gl::HALF_FLOAT,
            TextureDataType::UnsignedInt24_8 => gl::UNSIGNED_INT_24_8,
            TextureDataType::Int => gl::INT,
            TextureDataType::Byte => gl::BYTE,
            TextureDataType::UnsignedInt5_999 => gl::UNSIGNED_INT_5_9_9_9_REV,
            TextureDataType::Float32UnsignedInt8 => gl::FLOAT_32_UNSIGNED_INT_24_8_REV,
        }
    }
    pub fn from_glenum(glenum: u32) -> Option<Self> {
        Some(match glenum {
            gl::UNSIGNED_INT => TextureDataType::UnsignedInt,
            gl::UNSIGNED_BYTE => TextureDataType::UnsignedByte,
            gl::FLOAT => TextureDataType::Float,
            gl::HALF_FLOAT => TextureDataType::HalfFloat,
            gl::UNSIGNED_INT_24_8 => TextureDataType::UnsignedInt24_8,
            gl::INT => TextureDataType::Int,
            gl::BYTE => TextureDataType::Byte,
            gl::UNSIGNED_INT_5_9_9_9_REV => TextureDataType::UnsignedInt5_999,
            gl::FLOAT_32_UNSIGNED_INT_24_8_REV => TextureDataType::Float32UnsignedInt8,
            _ => return None,
        })
    }
}
///Defines format of image in which it will be stored
#[derive(Debug, Clone, Copy)]
pub enum TextureFormat {
    //unsigned integer
    RGBA,
    RGB,
    RGB10A2,
    RGBAu32,
    RGBu32,
    ///16 bit floats
    RGBA16F,
    RGB16F,
    //16 bit integers
    RGB16,
    RGBA16,
    //16 bit unsigned ints
    RGB8,
    RGBA8,
    RGB8SNorm,
    RGBA8SNorm,
    //Stencil
    Stencil8,
    StencilIndex,
    //Depth+Stencil
    Depth24Stencil8,
    Depth32FStencil8,
    DepthStencilComponent,
    //depth
    DepthComponent,
    DepthComponent32F,
    SrgbA,
    SRGB,
    BGRA,
    R11G11B10F,
    RGB9E5,
    RG16F,
}
impl TextureFormat {
    pub fn into_glenum(&self) -> u32 {
        match self {
            TextureFormat::RGBA => gl::RGBA,
            TextureFormat::RGB => gl::RGB,
            TextureFormat::RGBA16F => gl::RGBA16F,
            TextureFormat::RGB16F => gl::RGB16F,
            TextureFormat::RGB8 => gl::RGB8,
            TextureFormat::RGBA8 => gl::RGBA8,
            TextureFormat::DepthComponent32F => gl::DEPTH_COMPONENT32F,
            TextureFormat::DepthComponent => gl::DEPTH_COMPONENT,
            TextureFormat::StencilIndex => gl::STENCIL_INDEX,
            TextureFormat::RGBA16 => gl::RGBA16,
            TextureFormat::RGB16 => gl::RGB16,
            TextureFormat::SrgbA => gl::SRGB_ALPHA,
            TextureFormat::BGRA => gl::BGRA,
            TextureFormat::R11G11B10F => gl::R11F_G11F_B10F,
            TextureFormat::RGB9E5 => gl::RGB9_E5,
            TextureFormat::RGBAu32 => gl::RGBA32UI,
            TextureFormat::RGBu32 => gl::RGB32UI,
            TextureFormat::RGB8SNorm => gl::RGB8_SNORM,
            TextureFormat::RG16F => gl::RG16F,
            TextureFormat::Depth24Stencil8 => gl::DEPTH24_STENCIL8,
            TextureFormat::DepthStencilComponent => gl::DEPTH_STENCIL,
            TextureFormat::Stencil8 => gl::STENCIL_INDEX8,
            TextureFormat::RGB10A2 => gl::RGB10_A2,
            TextureFormat::RGBA8SNorm => gl::RGBA8_SNORM,
            TextureFormat::SRGB => gl::SRGB,
            TextureFormat::Depth32FStencil8 => gl::DEPTH32F_STENCIL8,
        }
    }
    pub fn from_glenum(glenum: u32) -> Option<Self> {
        Some(match glenum {
            gl::RGBA => TextureFormat::RGBA,
            gl::RGB => TextureFormat::RGB,
            gl::RGBA16F => TextureFormat::RGBA16F,
            gl::RGB16F => TextureFormat::RGB16F,
            gl::RGB8 => TextureFormat::RGB8,
            gl::RGBA8 => TextureFormat::RGBA8,
            gl::DEPTH_COMPONENT32F => TextureFormat::DepthComponent32F,
            gl::DEPTH_COMPONENT => TextureFormat::DepthComponent,
            gl::STENCIL_INDEX => TextureFormat::StencilIndex,
            gl::RGBA16 => TextureFormat::RGBA16,
            gl::RGB16 => TextureFormat::RGB16,
            gl::SRGB_ALPHA => TextureFormat::SrgbA,
            gl::BGRA => TextureFormat::BGRA,
            gl::R11F_G11F_B10F => TextureFormat::R11G11B10F,
            gl::RGB9_E5 => TextureFormat::RGB9E5,
            gl::RGBA32UI => TextureFormat::RGBAu32,
            gl::RGB32UI => TextureFormat::RGBu32,
            gl::RGB8_SNORM => TextureFormat::RGB8SNorm,
            gl::RG16F => TextureFormat::RG16F,
            gl::DEPTH24_STENCIL8 => TextureFormat::Depth24Stencil8,
            gl::DEPTH_STENCIL => TextureFormat::DepthStencilComponent,
            gl::STENCIL_INDEX8 => TextureFormat::Stencil8,
            gl::RGB10_A2 => TextureFormat::RGB10A2,
            gl::RGBA8_SNORM => TextureFormat::RGBA8SNorm,
            gl::SRGB => TextureFormat::SRGB,
            gl::DEPTH32F_STENCIL8 => TextureFormat::Depth32FStencil8,
            _ => return None,
        })
    }
    ///gives compatible texture format for given internal format
    pub fn to_texture_format<T: TextureTypeTrait>(&self) -> TextureFormat {
        let mut format = 0;
        unsafe {
            gl::GetInternalformativ(
                T::texture_type().into_glenum(),
                self.into_glenum(),
                gl::TEXTURE_IMAGE_FORMAT,
                1,
                &mut format,
            );
        }
        Self::from_glenum(format as u32).unwrap()
    }
    ///gives compatible texture data for given internal format
    pub fn to_texture_type<T: TextureTypeTrait>(&self) -> TextureDataType {
        let mut r#type = 0;
        unsafe {
            gl::GetInternalformativ(
                T::texture_type().into_glenum(),
                self.into_glenum(),
                gl::TEXTURE_IMAGE_TYPE,
                1,
                &mut r#type,
            );
        }
        TextureDataType::from_glenum(r#type as u32).unwrap()
    }
}
