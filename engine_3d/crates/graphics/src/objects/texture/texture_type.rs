pub trait TextureTypeTrait: Clone {
    fn texture_type() -> TextureType;
}
pub (super) trait D1 {}
pub (super) trait D2: D1 {}
pub (super) trait D3: D2 {}
#[derive(Clone, Copy, Debug)]
pub struct Tex1D;
impl D1 for Tex1D {}
impl TextureTypeTrait for Tex1D {
    fn texture_type() -> TextureType {
        TextureType::OneDimensional
    }
}
#[derive(Clone, Copy, Debug)]
pub struct Tex2D;
impl TextureTypeTrait for Tex2D {
    fn texture_type() -> TextureType {
        TextureType::TwoDimensional
    }
}
impl D1 for Tex2D {}
impl D2 for Tex2D {}

#[derive(Clone, Copy, Debug)]
pub struct Tex3D;
impl TextureTypeTrait for Tex3D {
    fn texture_type() -> TextureType {
        TextureType::ThreeDimensional
    }
}
impl D1 for Tex3D {}
impl D2 for Tex3D {}
impl D3 for Tex3D {}

#[derive(Clone, Copy, Debug)]
pub struct TextureArray1D;
impl TextureTypeTrait for TextureArray1D {
    fn texture_type() -> TextureType {
        TextureType::Array1D
    }
}

#[derive(Clone, Copy, Debug)]
pub struct TextureArray2D;
impl TextureTypeTrait for TextureArray2D {
    fn texture_type() -> TextureType {
        TextureType::Array2D
    }
}
#[derive(Clone, Copy, Debug)]
pub struct CubeMapTexture;
impl TextureTypeTrait for CubeMapTexture {
    fn texture_type() -> TextureType {
        TextureType::CubeMap
    }
}

#[derive(Clone, Copy, Debug)]
pub enum TextureType {
    OneDimensional,
    TwoDimensional,
    ThreeDimensional,
    Array1D,
    Array2D,
    CubeMap,
}
impl TextureType {
    pub fn into_glenum(&self) -> u32 {
        match self {
            TextureType::OneDimensional => gl::TEXTURE_1D,
            TextureType::TwoDimensional => gl::TEXTURE_2D,
            TextureType::ThreeDimensional => gl::TEXTURE_3D,
            TextureType::Array1D => gl::TEXTURE_1D_ARRAY,
            TextureType::Array2D => gl::TEXTURE_2D_ARRAY,
            TextureType::CubeMap => gl::TEXTURE_CUBE_MAP,
        }
    }
}
