use std::sync::{LazyLock, Mutex};
use crate::objects::{model::InstancedModel, shader::{Shader, ShaderType, SubShader}, vertex::{IntoGLenum, Vertex}};

pub fn start_debug_marker(name:&str){
    unsafe{
        let message = std::ffi::CString::new(name).unwrap();
        gl::PushDebugGroup(gl::DEBUG_SOURCE_APPLICATION, 1, message.count_bytes() as i32, message.to_bytes().as_ptr() as *const i8);
    }
}
pub fn end_debug_marker(){
    unsafe{
        gl::PopDebugGroup();
    }
}
pub fn error_check() -> Result<(),u32>{
    let err_code = unsafe {
        gl::GetError()
    };
    if err_code != 0 {
        return Err(err_code);
    }
    return Ok(())
}

///
pub static EMPTY: LazyLock<InstancedModel> =
    LazyLock::new(|| InstancedModel::new_without_vertex(3));

///
pub static FULLSCREENPASS_VERTEX_SHADER: LazyLock<SubShader> =
    LazyLock::new(|| SubShader::new(include_str!("./opt_vert.glsl"), ShaderType::Vertex));
/// 
pub static mut COPY_FRAGMENT_SHADER: LazyLock<Mutex<Shader>> = LazyLock::new(|| {
    Mutex::new(Shader::new([
        SubShader::new(
            include_str!("./copy.glsl"),
            ShaderType::Fragment,
        ),
        *FULLSCREENPASS_VERTEX_SHADER,
    ]))
});