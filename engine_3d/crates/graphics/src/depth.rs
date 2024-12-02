use crate::compare_opt::CompareOption;


pub fn set_cmp_func(cmp_f:CompareOption){
    unsafe{
        gl::DepthFunc(cmp_f.into());
    }
}
pub fn set_write(write:bool){
    unsafe{
        gl::DepthMask(write as u8);
    }
}
pub fn enable(){
    unsafe{
        gl::Enable(gl::DEPTH_TEST);
    } 
}
pub fn disable(){
    unsafe{
        gl::Disable(gl::DEPTH_TEST);
    } 
}