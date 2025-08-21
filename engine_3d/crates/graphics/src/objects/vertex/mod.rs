use math::Mat4;
use gl;

use super::buffers::{Buffer, Data};
pub trait Vertex: Sized {
    fn declaration();
}
#[allow(unused)]
pub trait IntoGLenum {
    fn into_glenum() -> u32;
    fn instanced_attrib(index: &mut u32, update_every: u32) {}
}
#[macro_export]
macro_rules! impl_into_glenum {
    ($($type_name:ty,$return:expr),+) => {
        $(impl IntoGLenum for $type_name{
            fn into_glenum() -> u32{
                $return
            }
        })*
    };
}
//no f16/half-float(not stable)
impl_into_glenum!(
    f64,
    gl::DOUBLE,
    f32,
    gl::FLOAT,
    //f16,gl::HALF_FLOAT,
    i32,
    gl::INT,
    i16,
    gl::SHORT,
    i8,
    gl::BYTE,
    u32,
    gl::UNSIGNED_INT,
    u16,
    gl::UNSIGNED_SHORT,
    u8,
    gl::UNSIGNED_BYTE
);
impl IntoGLenum for Mat4 {
    fn instanced_attrib(index: &mut u32, update_every: u32) {
        let instanced_array: Buffer<Data> = Buffer::<Data>::create();
        instanced_array.bind();
        for i in 0..4 {
            unsafe {
                gl::EnableVertexAttribArray(*index + i);
                gl::VertexAttribPointer(
                    *index + i,
                    4,
                    Self::into_glenum(),
                    gl::FALSE,
                    16 * std::mem::size_of::<f32>() as i32,
                    (std::mem::size_of::<f32>() as u32 * i as u32 * 4) as *const _,
                );
                gl::VertexAttribDivisor(*index + i, update_every);
            }
        }
        Buffer::<Data>::unbind();
    }

    fn into_glenum() -> u32 {
        gl::FLOAT
    }
}
#[macro_export]
macro_rules! define_vertex {
    ($name:ident,$($var_name:ident,$type:ident,$count:expr),+) => {

        #[derive(Debug,Clone,Copy)]
        #[repr(C)]
        pub struct $name{
            $(pub $var_name:[$type;$count]),*
        }
        #[allow(unused_assignments)]
        impl Vertex for $name{
            fn declaration(){
                let mut index = 0;
                let size = std::mem::size_of::<$name>();
                unsafe{
                    $(
                        gl::VertexAttribPointer(index, $count, $type::into_glenum(), gl::FALSE, size as i32, std::mem::offset_of!($name,$var_name) as *const _);
                        gl::EnableVertexAttribArray(index);
                        index +=1;
                    )*
                }
            }
        }
        #[allow(dead_code,unused_assignments)]
        impl $name{
            fn debug_decl(){
                let mut index = 0;
                let size = std::mem::size_of::<$name>();
                $(
                    println!("{} {} {} {} {} {}",index, $count, $type::into_glenum(), gl::FALSE, size as i32, std::mem::offset_of!($name,$var_name));
                    index += 1;
                )*
            }
            pub fn new($($var_name:[$type;$count]),*) -> Self{
                Self{
                    $($var_name),*
                }
            }
        }
    };
}

define_vertex!(
    ModelVertex,
    position,
    f32,
    3,
    normal,
    f32,
    3,
    texture_coords,
    f32,
    2
);
#[test]
pub fn test() {
    ModelVertex::debug_decl();
    println!("{}", std::mem::offset_of!(ModelVertex, texture_coords));
}
