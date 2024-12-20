use std::{collections::HashMap, ffi::CString};

use glam::*;

use super::{
    buffers::{ Buffer, ShaderStorage},
    texture::Texture2D,
};

static mut CURRENT_SHADER:u32 = 0;
#[derive(Debug, Clone)]
pub struct Shader {
    uniforms: HashMap<String, i32>,
    id: u32,
}
impl Shader {
    pub fn id(&self) -> u32 {
        self.id
    }
    pub fn set_matrix4(&mut self, uniform_name: &str, data: &Mat4) {
        self.bind();
        unsafe { gl::UniformMatrix4fv(self.u_location(uniform_name), 1, gl::FALSE, &data.to_cols_array()[0]) }
    }
    pub fn set_vec3(&mut self, uniform_name: &str, data: &Vec3) {
        self.bind();
        unsafe { gl::Uniform3fv(self.u_location(uniform_name), 1, &data[0]) }
    }
    pub fn set_vec2(&mut self, uniform_name: &str, data: &Vec2) {
        self.bind();
        unsafe { gl::Uniform2fv(self.u_location(uniform_name), 1, &data[0]) }
    }
    pub fn set_f32(&mut self, uniform_name: &str, data: f32) {
        self.bind();
        unsafe { gl::Uniform1f(self.u_location(uniform_name), data) }
    }
    pub fn set_int(&mut self, uniform_name: &str, data: i32) {
        self.bind();
        unsafe { gl::Uniform1i(self.u_location(uniform_name), data) }
    }
    pub fn set_bool(&mut self, uniform_name: &str, data: bool) {
        self.bind();
        self.set_int(uniform_name, data.into());
    }
    pub fn set_texture2d(&mut self, uniform_name: &str, data: &Texture2D, i: u32) {
        self.bind();
        Texture2D::set_active(i);
        data.bind();
        self.set_int(uniform_name, i as i32);
    }
    pub fn set_shader_storage_block(&mut self, block_name: &str, buffer: &Buffer<ShaderStorage>,block_binding:u32) {
        self.bind();
        unsafe {
            buffer.bind_buffer_base(block_binding);
            gl::ShaderStorageBlockBinding ( self.id, self.shader_storage_loc(block_name), block_binding );
        }
    }
    fn shader_storage_loc(&mut self, name: &str) -> u32 {
        if let Some(&addr) = self.uniforms.get(name) {
            addr as u32
        } else {
            let c_name = CString::new(name).unwrap();
            let addr = unsafe {
                gl::GetProgramResourceIndex(self.id, gl::SHADER_STORAGE_BLOCK, c_name.as_ptr())
            };
            if addr == u32::MAX {
                log::error!("no shader storage with name:{}", name);
            }
            self.uniforms.insert(name.to_owned(), addr as i32);
            addr as u32
        }
    }
    fn u_location(&mut self, uniform_name: &str) -> i32 {
        if let Some(&addr) = self.uniforms.get(uniform_name) {
            addr
        } else {
            let cuniform_name = CString::new(uniform_name).unwrap();
            let addr = unsafe { gl::GetUniformLocation(self.id, cuniform_name.as_ptr()) };
            if addr == -1 {
                log::error!("no uniform with name:{}", uniform_name);
            }
            self.uniforms.insert(uniform_name.to_owned(), addr);
            addr
        }
    }
}
impl Shader {
    pub fn new<T: IntoIterator<Item = SubShader>>(subshaders: T) -> Self {
        unsafe {
            let id = gl::CreateProgram();
            let mut iter = subshaders.into_iter();
            for subshader in iter.by_ref() {
                gl::AttachShader(id, subshader.id);
            }
            gl::LinkProgram(id);
            let mut status = 0;
            gl::GetProgramiv(id, gl::LINK_STATUS, &mut status);
            if status == 0 {
                let mut len = 0;
                gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len);
                let mut buff = vec![0; len as usize];
                gl::GetProgramInfoLog(id, len, std::ptr::null_mut(), buff.as_mut_ptr() as *mut i8);
                let log = std::ffi::CStr::from_bytes_with_nul_unchecked(&buff)
                    .to_str()
                    .unwrap();
                log::error!("shader link error:{}", log);
            }
            for subshader in iter {
                gl::DetachShader(id, subshader.id);
            }
            Self {
                id,
                uniforms: HashMap::new(),
            }
        }
    }
    pub fn bind(&self) {
        if self.id() != unsafe{CURRENT_SHADER}{
            unsafe {
                gl::UseProgram(self.id);
                CURRENT_SHADER = self.id()
            }
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum ShaderType {
    Vertex,
    Fragment,
    Compute,
    Geometry,
    TessalationControl,
    TesselationEvaluation,
}
impl Into<u32> for ShaderType {
    fn into(self) -> u32 {
        match self {
            Self::Vertex => gl::VERTEX_SHADER,
            Self::Fragment => gl::FRAGMENT_SHADER,
            Self::Compute => gl::COMPUTE_SHADER,
            Self::Geometry => gl::GEOMETRY_SHADER,
            Self::TessalationControl => gl::TESS_CONTROL_SHADER,
            Self::TesselationEvaluation => gl::TESS_EVALUATION_SHADER,
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub struct SubShader {
    id: u32,
}
impl SubShader {
    pub fn new(source: &str, shader_type: ShaderType) -> Self {
        unsafe {
            let id = gl::CreateShader(shader_type.into());
            let cstring = std::ffi::CString::new(source).unwrap();
            gl::ShaderSource(id, 1, &cstring.as_ptr(), std::ptr::null());
            gl::CompileShader(id);
            let mut success = 0;
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                let mut len = 0;
                gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
                let mut log_buff = vec![0u8; len as usize];
                gl::GetShaderInfoLog(
                    id,
                    len,
                    std::ptr::null_mut(),
                    log_buff.as_mut_ptr() as *mut i8,
                );
                let log = std::ffi::CStr::from_bytes_with_nul(&log_buff)
                    .unwrap()
                    .to_str()
                    .unwrap();
                log::error!("Failed to compile shader: {}", log);
            }
            Self { id }
        }
    }
}
