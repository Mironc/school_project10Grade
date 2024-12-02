pub mod objects;
pub mod ecs;
pub mod depth;
pub mod stencil;
pub mod color;
pub mod face_culling;
pub mod blending;
pub mod compare_opt;
pub mod utils;
pub fn error_check() -> Result<(),u32>{
    let err_code = unsafe {
        gl::GetError()
    };
    if err_code != 0 {
        return Err(err_code);
    }
    return Ok(())
}