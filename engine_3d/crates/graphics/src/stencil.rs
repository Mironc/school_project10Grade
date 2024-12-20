use crate::compare_opt::CompareOption;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
static mut STENCIL_FUNC: StencilFunction = StencilFunction {
    cmp_opt: CompareOption::Always,
    ref_value: 0,
    mask: 255,
};
static mut STENCIL_OPTIONS: StencilOptions = StencilOptions {
    stencil_fail_opt: Action::Keep,
    depth_fail_opt: Action::Keep,
    depth_pass_opt: Action::Keep,
};
static mut WRITE: bool = true;
static mut ENABLED: bool = false;

pub fn set_stencil_function(stencil_f: StencilFunction) {
    if stencil_f != unsafe { STENCIL_FUNC } {
        unsafe {
            gl::StencilFunc(
                stencil_f.cmp_opt.into(),
                stencil_f.ref_value,
                stencil_f.mask,
            );
            STENCIL_FUNC = stencil_f;
        }
    }
}
pub fn enable() {
    if unsafe { !ENABLED } {
        unsafe {
            gl::Enable(gl::STENCIL_TEST);
            ENABLED = true;
        }
    }
}
pub fn disable() {
    if unsafe { ENABLED } {
        unsafe {
            gl::Disable(gl::STENCIL_TEST);
            ENABLED = false;
        }
    }
}
pub fn set_write(write: bool) {
    if write != unsafe{WRITE}{
        unsafe {
            gl::StencilMask(if write { 255 } else { 0 });
            WRITE = write;
        }
    }
}
pub fn set_stencil_options(stencil_opt: StencilOptions) {
    if stencil_opt != unsafe{STENCIL_OPTIONS}{
        unsafe {
            gl::StencilOp(
                stencil_opt.stencil_fail_opt.into(),
                stencil_opt.depth_fail_opt.into(),
                stencil_opt.depth_pass_opt.into(),
            );
            STENCIL_OPTIONS = stencil_opt;
        }
    }
}
