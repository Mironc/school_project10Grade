pub struct AreaLight{
    radius:f32,
}
impl AreaLight{
    pub fn new(&self,radius:f32) -> Self{
        Self { radius }
    } 
    pub fn radius(&self) -> f32{
        self.radius
    }
    pub fn set_radius(&mut self, radius:f32){
        self.radius = radius
    }
}