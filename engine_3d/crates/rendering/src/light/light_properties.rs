use math::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct LightProperties {
    pub range: f32,
    pub intensity: f32,
    pub color: Vec3,
}
impl LightProperties {
    pub fn new(&self, range: f32, intensity: f32, color: Vec3) -> Self {
        Self {
            range,
            intensity,
            color,
        }
    }
    pub fn range(&self) -> f32 {
        self.range
    }
    pub fn intensity(&self) -> f32 {
        self.intensity
    }
    pub fn color(&self) -> Vec3 {
        self.color
    }
    pub fn set_range(&mut self, range: f32) {
        self.range = range
    }
    pub fn set_intesity(&mut self, intensity: f32) {
        self.intensity = intensity
    }
    pub fn set_color(&mut self, color: Vec3) {
        self.color = color
    }
}
