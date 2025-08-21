use image::DynamicImage;
use math::{IVec2, Vec2};

use super::{texture_type::TextureType, Filter, TextureDataType, TextureFormat, TextureWrap};

pub(crate) trait Texture1DTrait:TextureTrait {
    fn width(&self) -> i32;
    fn set_texture_wrap_x(&mut self, texture_wrap: TextureWrap);
}
pub(crate) trait Texture2DTrait: Texture1DTrait {
    fn height(&self) -> i32;
    fn wrap_y(&self) -> TextureWrap;
    fn set_texture_wrap_y(&mut self, texture_wrap: TextureWrap);
}
pub(crate) trait Texture3DTrait: Texture1DTrait {
    fn depth(&self) -> i32;
    fn wrap_z(&self) -> TextureWrap;
    fn set_texture_wrap_z(&mut self, texture_wrap: TextureWrap);
}
pub trait TextureTrait {
    fn set_min_filter(&mut self, filter: Filter);

    fn set_mag_filter(&mut self, filter: Filter);

    fn gen_mipmaps(&self) {
        self.bind();
        unsafe {
            gl::GenerateMipmap(self.texture_type().into_glenum());
        }
    }
    fn texture_type(&self) -> TextureType;

    fn id(&self) -> u32;

    fn bind(&self) {
        unsafe { gl::BindTexture(self.texture_type().into_glenum(), self.id()) }
    }
    
    fn set_active(i: u32) {
        unsafe { gl::ActiveTexture(gl::TEXTURE0 + i) }
    }

    fn internal_format(&self) -> TextureFormat;

    fn texture_format(&self) -> TextureFormat;

    fn texture_data_type(&self) -> TextureDataType;

    fn mag_filter(&self) -> Filter;

    fn min_filter(&self) -> Filter;
}/* 
pub trait CopyTexture<T:TextureTrait>:TextureTrait{
    ///Copies color data from lhs to rhs
    /// 
    ///Comes without any restrictions
    ///
    fn copy(lhs:&T,rhs:&T){
    }
    ///Comes along with a lot of restrictions
    fn dcopy(lhs:&T,rhs:&T);
    fn copy_subdata(lhs:&T,rhs:&T,lhs_left_corner:IVec2,lhs_right_corner:IVec2,rhs_left_corner:IVec2,rhs_right_corner:IVec2);
    fn dcopy_subdata(lhs:&T,rhs:&T,lhs_left_corner:IVec2,lhs_right_corner:IVec2,rhs_left_corner:IVec2,rhs_right_corner:IVec2);
} */