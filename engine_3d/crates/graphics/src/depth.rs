use crate::compare_opt::CompareOption;
static mut DEPTH_FUNC:CompareOption = CompareOption::Less;
static mut DEPTH_MASK:bool = true;
static mut ENABLED:bool = false;


pub fn set_cmp_func(cmp_f:CompareOption){
    if cmp_f != unsafe { DEPTH_FUNC }{
        unsafe{
            gl::DepthFunc(cmp_f.into());
            DEPTH_FUNC = cmp_f;
        }
    }
}
pub fn set_write(write:bool){
    if write != unsafe { DEPTH_MASK} {
        unsafe{
            gl::DepthMask(write as u8);
            DEPTH_MASK = write;
        }
    }
}
pub fn enable(){
    if !unsafe{ENABLED} {
        unsafe{
            gl::Enable(gl::DEPTH_TEST);
            ENABLED = true;
        } 
    }
}
pub fn disable(){
    if unsafe{ ENABLED}{
        unsafe{
            gl::Disable(gl::DEPTH_TEST);
            ENABLED = false ;
        }
    }
}