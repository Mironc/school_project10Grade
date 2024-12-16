use std::{error::Error, fmt::Display};

use image::{DynamicImage, EncodableLayout, GenericImageView, RgbaImage};
/// default filtering is Filter::Repeat and default texture wrap is TextureWrap::Nearest
#[derive(Debug, Clone)]
pub struct Texture2DBuilder {
    texture: Texture2D,
    image: Option<DynamicImage>,
    texture_size: (i32, i32),
    texture_format: TextureFormat,
    internal_format: TextureFormat,
    texture_type: TextureType,
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
            texture_type: TextureType::UnsignedByte,
        }
    }
    pub fn texture_format(mut self, texture_format: TextureFormat) -> Self {
        self.texture_format = texture_format;
        self
    }
    pub fn texture_type(mut self, texture_type: TextureType) -> Self {
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
#[derive(Debug, Clone)]
pub struct Texture2D {
    id: u32,
    wrap_x: TextureWrap,
    wrap_y: TextureWrap,
    min_filter: Filter,
    mag_filter: Filter,
    data_type: TextureType,
    internal_format: TextureFormat,
    texture_format: TextureFormat,
    height:i32,
    width:i32,
}
impl Texture2D {
    pub fn new() -> Self {
        unsafe {
            let mut id = 0;
            gl::GenTextures(1, &mut id);
            Self {
                id,
                wrap_x: TextureWrap::Repeat,
                wrap_y: TextureWrap::Repeat,
                min_filter: Filter::Nearest,
                mag_filter: Filter::Nearest,
                internal_format: TextureFormat::RGBA,
                texture_format: TextureFormat::RGBA,
                data_type: TextureType::UnsignedByte,
                height: 0,
                width: 0,
            }
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
    pub fn texture_type(&self) -> TextureType {
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
    pub fn width(&self) -> i32{
        self.width
    }
    pub fn height(&self) -> i32{
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
                internal_format.to_internal_format_code() as i32,
                size.0 as i32,
                size.1 as i32,
                0,
                TextureFormat::RGBA.to_internal_format_code(),
                TextureType::UnsignedByte.into_glenum(),
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
        texture_type: TextureType,
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
                internal_format.to_internal_format_code() as i32,
                width,
                height,
                0,
                texture_format.to_internal_format_code(),
                texture_type.into_glenum(),
                std::ptr::null(),
            )
        }
    }
}
/// Texture wrap is defining behaviour, when you try to get a pixel beyond borders
#[derive(Debug, Clone, Copy)]
pub enum TextureWrap {
    Repeat,
    ClampToEdge,
    MirroredRepeat,
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
/// Defines behaviour when accessing between pixels
#[derive(Debug, Clone, Copy)]
pub enum Filter {
    Linear,
    Nearest,
    NearestLinearMipMap,
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
pub enum TextureType {
    Byte,
    Int,
    UnsignedInt,
    UnsignedByte,
    UnsignedInt24_8,
    Float,
    HalfFloat,
}
impl TextureType {
    pub fn into_glenum(&self) -> u32 {
        match self {
            TextureType::UnsignedInt => gl::UNSIGNED_INT,
            TextureType::UnsignedByte => gl::UNSIGNED_BYTE,
            TextureType::Float => gl::FLOAT,
            TextureType::HalfFloat => gl::HALF_FLOAT,
            TextureType::UnsignedInt24_8 => gl::UNSIGNED_INT_24_8,
            TextureType::Int => gl::INT,
            TextureType::Byte => gl::BYTE,
        }
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
    DepthStencilComponent,
    //depth
    DepthComponent,
    DepthComponent32F,
    RgbaSrgb,
    BGRA,
    R11G11B10F,
    RGB9E5,
    RG16F,
}
impl TextureFormat {
    pub fn to_internal_format_code(&self) -> u32 {
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
            TextureFormat::RgbaSrgb => gl::SRGB_ALPHA,
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
        }
    }
    pub fn to_texture_type(&self) -> TextureType {
        match self {
            TextureFormat::RGBAu32 | TextureFormat::RGBu32 => TextureType::UnsignedInt,
            TextureFormat::RGBA16F
            | TextureFormat::RG16F
            | Self::R11G11B10F
            | Self::DepthComponent32F
            | TextureFormat::RGB16F => TextureType::Float,

            TextureFormat::RGB8SNorm | TextureFormat::RGBA8SNorm => TextureType::Byte,
            TextureFormat::Stencil8
            | TextureFormat::StencilIndex
            | TextureFormat::DepthStencilComponent
            | TextureFormat::RGBA8
            | TextureFormat::RGB8
            | TextureFormat::RGBA
            | TextureFormat::RGB
            | TextureFormat::RGB16
            | TextureFormat::RGBA16
            | TextureFormat::DepthComponent
            | TextureFormat::BGRA
            | TextureFormat::RGB9E5
            | TextureFormat::RGB10A2
            | TextureFormat::RgbaSrgb => TextureType::UnsignedByte,
            TextureFormat::Depth24Stencil8 => TextureType::UnsignedInt24_8,
        }
    }
}
