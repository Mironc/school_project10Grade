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