use math::Vec3;

pub struct DirectionalLight {
    direction: Vec3,
}
impl DirectionalLight {
    pub fn new(direction: Vec3) -> Self {
        Self { direction }
    }
    pub fn direction(&self) -> Vec3 {
        self.direction
    }
    pub fn set_direction(&mut self, direction: Vec3) {
        self.direction = direction
    }
}
