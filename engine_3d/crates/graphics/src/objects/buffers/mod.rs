use gl::*;
use std::marker::PhantomData;
mod framebuffer;
use super::vertex::Vertex;
pub use framebuffer::*;
#[macro_export]
macro_rules! impl_data_type {
    ($data_type_name:ident,$($field_name:ident,$field_type:ty),+) => {
        #[repr(C,align(16))]
        pub struct $data_type_name{
            $($field_name:$field_type),*
        }
        impl DataType for $data_type_name{
            fn aligned_offset() -> isize{
                size_of::<Self>() as isize
            }
        }
    };
}
pub trait DataType {
    fn aligned_offset() -> isize;
}
impl VBO {
    pub fn default() -> Buffer<VBO> {
        Buffer {
            _type: PhantomData::<VBO> {},
            id: 0,
        }
    }
}
#[derive(Debug, Clone)]
pub struct Data;
#[derive(Debug, Clone)]
pub struct EBO;
#[derive(Debug, Clone)]
pub struct VBO;
#[derive(Debug, Clone)]
pub struct Uniform;
#[derive(Debug, Clone)]
pub struct ShaderStorage;

#[derive(Debug, Clone)]
pub struct Buffer<T> {
    _type: PhantomData<T>,
    id: u32,
}
impl<T> Buffer<T> {
    pub fn create() -> Self {
        let mut id = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
        }
        Self {
            _type: PhantomData,
            id,
        }
    }
    pub fn bind_buffer_base(&self, base_point_index: u32) {
        unsafe {
            gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, base_point_index, self.id);
        }
    }
}
impl<T> Drop for Buffer<T> {
    fn drop(&mut self) {
        if self.id == 0 {
            return;
        }
        unsafe { DeleteBuffers(1, &mut self.id) };
    }
}

impl Buffer<Data> {
    pub fn set_data<T>(&self, data: impl AsRef<[T]>) {
        unsafe {
            gl::BufferData(
                ARRAY_BUFFER,
                (data.as_ref().len() * std::mem::size_of::<T>()) as isize,
                data.as_ref().as_ptr() as *const _,
                gl::STREAM_DRAW,
            )
        }
    }
    pub fn bind(&self) {
        unsafe { gl::BindBuffer(ARRAY_BUFFER, self.id) }
    }
    pub fn unbind() {
        unsafe { gl::BindBuffer(ARRAY_BUFFER, 0) }
    }
}
impl Buffer<Uniform> {
    pub fn set_data<T: DataType>(&self, data: impl AsRef<[T]>) {
        unsafe {
            gl::BufferData(
                gl::UNIFORM_BUFFER,
                (data.as_ref().len() * size_of::<T>()) as isize,
                data.as_ref().as_ptr() as *const _,
                gl::STATIC_DRAW,
            );
        }
    }
    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::UNIFORM_BUFFER, self.id);
        }
    }
    pub fn unbind() {
        unsafe {
            gl::BindBuffer(gl::UNIFORM_BUFFER, 0);
        }
    }
}
impl Buffer<ShaderStorage> {
    pub fn set_data<T: DataType>(&self, data: impl AsRef<[T]>) {
        unsafe {
            gl::BufferData(
                gl::SHADER_STORAGE_BUFFER,
                T::aligned_offset() * data.as_ref().len() as isize,
                data.as_ref().as_ptr() as *const _,
                gl::STATIC_DRAW,
            );
            //println!("{}", std::mem::size_of::<T>());
        }
    }
    pub fn set_data_immut<T: DataType>(&self, data: impl AsRef<[T]>) {
        unsafe {
            gl::BufferStorage(
                gl::SHADER_STORAGE_BUFFER,
                T::aligned_offset() * data.as_ref().len() as isize,
                data.as_ref().as_ptr() as *const _,
                gl::STATIC_DRAW,
            );
            //println!("{}", std::mem::size_of::<T>());
        }
    }
    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, self.id);
        }
    }
    pub fn unbind() {
        unsafe {
            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, 0);
        }
    }
}
impl Buffer<VBO> {
    pub fn set_data<T>(&self, data: impl AsRef<[T]>)
    where
        T: Vertex,
    {
        unsafe {
            gl::BufferData(
                ARRAY_BUFFER,
                (data.as_ref().len() * std::mem::size_of::<T>()) as isize,
                data.as_ref().as_ptr() as *const _,
                gl::STREAM_DRAW,
            )
        }
    }
    pub fn bind(&self) {
        unsafe { gl::BindBuffer(ARRAY_BUFFER, self.id) }
    }
    pub fn unbind() {
        unsafe { gl::BindBuffer(ARRAY_BUFFER, 0) }
    }
}

impl Buffer<EBO> {
    pub fn set_data<T>(&self, data: impl AsRef<[T]>) {
        unsafe {
            gl::BufferData(
                ELEMENT_ARRAY_BUFFER,
                (data.as_ref().len() * std::mem::size_of::<T>()) as isize,
                data.as_ref().as_ptr() as *const _,
                gl::STATIC_DRAW,
            )
        }
    }
    pub fn bind(&self) {
        unsafe { gl::BindBuffer(ELEMENT_ARRAY_BUFFER, self.id) }
    }
    pub fn unbind() {
        unsafe { gl::BindBuffer(ELEMENT_ARRAY_BUFFER, 0) }
    }
}

#[derive(Debug, Clone)]
pub struct VAO {
    id: u32,
}
static mut BINDED_VAO: u32 = 0;
impl VAO {
    pub fn new() -> Self {
        unsafe {
            let mut id = 0;
            GenVertexArrays(1, &mut id);
            Self { id }
        }
    }
    pub fn bind(&self) {
        unsafe {
            if BINDED_VAO != self.id {
                gl::BindVertexArray(self.id);
                BINDED_VAO = self.id;
            }
        }
    }
    pub fn unbind(){
        Self{id:0}.bind();
    }
}
impl Drop for VAO {
    fn drop(&mut self) {
        if self.id == 0 {
            return;
        }
        unsafe {
            gl::DeleteVertexArrays(1, &self.id);
        }
    }
}
impl Default for VAO {
    fn default() -> Self {
        Self { id: 0 }
    }
}
