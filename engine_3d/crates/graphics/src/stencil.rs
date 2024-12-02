use crate::compare_opt::CompareOption;

pub struct StencilFunction {
    cmp_opt: CompareOption,
    ref_value: i32,
    mask: u32,
}
impl StencilFunction {
    pub fn new(cmp_opt: CompareOption, ref_value: i32, mask: u32) -> Self {
        Self {
            cmp_opt,
            ref_value,
            mask,
        }
    }
    pub fn with_no_mask(cmp_opt: CompareOption, ref_value: i32) -> Self {
        Self::new(cmp_opt, ref_value, 255)
    }
}
pub fn set_stencil_function(stencil_f: StencilFunction) {
    unsafe {
        gl::StencilFunc(
            stencil_f.cmp_opt.into(),
            stencil_f.ref_value,
            stencil_f.mask,
        );
    }
}
pub fn enable() {
    unsafe {
        gl::Enable(gl::STENCIL_TEST);
    }
}
pub fn disable() {
    unsafe {
        gl::Disable(gl::STENCIL_TEST);
    }
}
pub fn set_write(write: bool) {
    unsafe {
        gl::StencilMask(if write { 255 } else { 0 });
    }
}
pub enum Action {
    Keep,
    Zero,
    Replace,
    Increment,
    IncrementWrap,
    Decrement,
    DecrementWrap,
    Invert,
}
impl Into<u32> for Action {
    fn into(self) -> u32 {
        match self {
            Action::Keep => gl::KEEP,
            Action::Zero => gl::ZERO,
            Action::Replace => gl::REPLACE,
            Action::Increment => gl::INCR,
            Action::IncrementWrap => gl::INCR_WRAP,
            Action::Decrement => gl::DECR,
            Action::DecrementWrap => gl::DECR_WRAP,
            Action::Invert => gl::INVERT,
        }
    }
}
pub struct StencilOptions {
    stencil_fail_opt: Action,
    depth_pass_opt: Action,
    depth_fail_opt: Action,
}
impl StencilOptions {
    pub fn new(stencil_fail_opt: Action, depth_pass_opt: Action, depth_fail_opt: Action) -> Self {
        Self {
            stencil_fail_opt,
            depth_pass_opt,
            depth_fail_opt,
        }
    }
}
pub fn set_stencil_options(stencil_opt: StencilOptions) {
    unsafe {
        gl::StencilOp(
            stencil_opt.stencil_fail_opt.into(),
            stencil_opt.depth_fail_opt.into(),
            stencil_opt.depth_pass_opt.into(),
        );
    }
}
