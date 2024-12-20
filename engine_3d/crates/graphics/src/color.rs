use glam::Vec4;

pub type Color = Vec4;

static mut MASK:[bool;4] = [true;4];

pub fn set_write(write_r:bool,write_g:bool,write_b:bool,write_a:bool,){
    if [write_r,write_g,write_b,write_a] != unsafe{
        MASK
    }{
        unsafe{
            gl::ColorMask(write_r as u8, write_g as u8, write_b as u8, write_a as u8);
            MASK = [write_r,write_g,write_b,write_a];
        }
    }
}