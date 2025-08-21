use engine_3d::{
    math::Vec3,
    specs::{Component, Join, LendJoin, Read, ReadStorage, System, VecStorage, WriteStorage},
    time::Time,
    transform::Transform,
};

use super::{collision::Collision3D, rigidbody::Rigidbody};

pub struct Static {}
impl Component for Static {
    type Storage = VecStorage<Self>;
}
pub struct GravitySystem {
    pub g: f32,
}
impl GravitySystem {
    pub fn run(
        &mut self,
        data: (
            &mut WriteStorage<'_, Rigidbody>,
            &mut WriteStorage<'_, Transform>,
            &Read<'_, Time>,
            &ReadStorage<'_, Static>,
        ),
    ) {
        let (rigidbody, transform_storage, time, _static) = data;
        for (transform, _static, rigidbody) in (
            transform_storage,
            _static.maybe(),
            rigidbody,
        )
            .join()
        {
            if _static.is_none() {
                rigidbody.set_velocity(rigidbody.velocity() + Vec3::NEG_Y * rigidbody.mass() * self.g * time.delta_time());
            }
        }
    }
}
/* impl<'a> System<'a> for GravitySystem {
    type SystemData = (
        ReadStorage<'a, Collision3D>,
        WriteStorage<'a, Rigidbody>,
        WriteStorage<'a, Transform>,
        Read<'a, Time>,
        ReadStorage<'a, Static>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (collision_storage, mut rigidbody, mut transform_storage, time, _static) = data;
        for (collision, transform, _static, rigidbody) in (
            &collision_storage,
            &mut transform_storage,
            _static.maybe(),
            &mut rigidbody,
        )
            .join()
        {
            if _static.is_none() {
                rigidbody.add_force(Vec3::NEG_Y * self.g * time.delta_time() * time.delta_time());
            }
        }
    }
} */

pub struct MovingSystem {
    pub speed: f32,
}
impl<'a> System<'a> for MovingSystem {
    type SystemData = (
        ReadStorage<'a, Collision3D>,
        WriteStorage<'a, Rigidbody>,
        WriteStorage<'a, Transform>,
        Read<'a, Time>,
        ReadStorage<'a, Static>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (collision_storage, mut rigidbody, mut transform_storage, time, _static) = data;
        for (collision, transform, _static, rigidbody) in (
            &collision_storage,
            &mut transform_storage,
            _static.maybe(),
            &mut rigidbody,
        )
            .join()
        {
            if _static.is_none() {
                rigidbody
                    .add_force(Vec3::NEG_Z * self.speed * time.delta_time() * time.delta_time());
            }
        }
    }
}
