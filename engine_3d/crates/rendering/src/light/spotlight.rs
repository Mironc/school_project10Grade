use math::Vec3;
///Properties of spotlight light source
/// 
///Normally, direction which spotlight is oriented is controlled by transform.forward()
/// 
pub struct SpotLight {
    angle: f32,
}
impl SpotLight {
    pub fn new(angle: f32) -> Self {
        Self { angle }
    }
    pub fn angle(&self) -> f32{
        self.angle
    }
    pub fn set_angle(&mut self, angle: f32) {
        self.angle = angle;
    }
}
