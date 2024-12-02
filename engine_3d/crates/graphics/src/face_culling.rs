pub enum CullFace{
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
pub fn set_cullface(cull_face:CullFace){
    unsafe{
        gl::CullFace(cull_face.into());
    }
}
pub fn enable(){
    unsafe{
        gl::Enable(gl::CULL_FACE);
    }
}
pub fn disable(){
    unsafe{
        gl::Disable(gl::CULL_FACE);
    }
}
pub enum FrontFaceOrder{
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
pub fn set_frontface_order(frontface_order:FrontFaceOrder){
    unsafe{
        gl::FrontFace(frontface_order.into());
    }
}