#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CullFace {
    Front,
    Back,
    FrontBack,
}
impl Into<u32> for CullFace {
    fn into(self) -> u32 {
        match self {
            CullFace::Front => gl::FRONT,
            CullFace::Back => gl::BACK,
            CullFace::FrontBack => gl::FRONT_AND_BACK,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrontFaceOrder {
    Clockwise,
    CounterClockwise,
}
impl Into<u32> for FrontFaceOrder {
    fn into(self) -> u32 {
        match self {
            FrontFaceOrder::Clockwise => gl::CW,
            FrontFaceOrder::CounterClockwise => gl::CCW,
        }
    }
}

static mut ENABLED: bool = false;
static mut ORDER: FrontFaceOrder = FrontFaceOrder::CounterClockwise;
static mut CULLFACE: CullFace = CullFace::Back;
pub fn set_cullface(cull_face: CullFace) {
    if cull_face != unsafe { CULLFACE } {
        unsafe {
            gl::CullFace(cull_face.into());
            CULLFACE = cull_face;
        }
    }
}
pub fn set_frontface_order(frontface_order: FrontFaceOrder) {
    if frontface_order != unsafe { ORDER } {
        unsafe {
            gl::FrontFace(frontface_order.into());
            ORDER = frontface_order;
        }
    }
}
pub fn enable() {
    if unsafe { !ENABLED } {
        unsafe {
            gl::Enable(gl::CULL_FACE);
            ENABLED = true;
        }
    }
}
pub fn disable() {
    if unsafe { ENABLED } {
        unsafe {
            gl::Disable(gl::CULL_FACE);
            ENABLED = false;
        }
    }
}
