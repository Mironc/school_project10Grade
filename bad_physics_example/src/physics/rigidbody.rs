use engine_3d::{
    math::Vec3,
    specs::{Component, Join, Read, System, VecStorage, WriteStorage},
    time::Time,
    transform::Transform,
};

use super::collision::Collision3D;
pub struct Rigidbody {
    position: Vec3,
    velocity: Vec3,
    mass: f32,
    friction: f32,
    //everything below is useless for some time
    torque: Vec3,
    rotation: Vec3,
    bounciness: f32,
}
impl Rigidbody {
    pub fn new(transform: &Transform, mass: f32, friction: f32, bounciness: f32) -> Self {
        Self {
            position: transform.position,
            velocity: Vec3::ZERO,
            rotation: transform.rotation(),
            torque: Vec3::ZERO,
            mass,
            friction,
            bounciness,
        }
    }
    pub fn friction(&self) -> f32 {
        self.friction
    }
    pub fn mass(&self) -> f32 {
        self.mass
    }
    pub fn bounciness(&self) -> f32 {
        self.bounciness
    }
    pub fn velocity(&self) -> Vec3 {
        self.velocity
    }
    pub fn position(&self) -> Vec3 {
        self.position
    }
    pub fn update(&mut self, position: Vec3) {
        self.position = position;
    }
    pub fn set_velocity(&mut self, velocity: Vec3) {
        self.velocity = velocity;
    }
    //if you use this method dont include time.delta_time()
    pub fn add_force(&mut self, force: Vec3) {
        self.velocity += force / self.mass;
    }
}
impl Component for Rigidbody {
    type Storage = VecStorage<Self>;
}

pub struct RigidbodySystem {}
impl RigidbodySystem {
    pub fn run(
        &mut self,
        data: (
            &mut WriteStorage<'_, Rigidbody>,
            &mut WriteStorage<'_, Transform>,
            &Read<'_, Time>,
        ),
    ) {
        let (r, t, time) = data;
        for physical_object in (r, t).join() {
            let mut vel = physical_object.0.velocity();
            //TODO:Add air resistance
            vel = physical_object.0.velocity();
            physical_object.1.position += vel * time.delta_time();
            let position = physical_object.1.position;
            physical_object.0.set_velocity(vel);
            physical_object.0.update(position);
        }
    }
}
