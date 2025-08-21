use std::time::Instant;

use engine_3d::{
    specs::{Entities, Read, ReadStorage, System, WriteStorage},
    time::Time,
    transform::Transform,
};

use super::{
    collision::Collision3D,
    collision_system::{CollisionSolverSystem, CollisionSystem},
    gravity::{GravitySystem, Static},
    rigidbody::{Rigidbody, RigidbodySystem},
};

pub struct PhysicsSystem {
    simulation_deltatime: f32,
    gravity: GravitySystem,
    rigidbody_system: RigidbodySystem,
    collision_solver_system: CollisionSolverSystem,
    last_time: f32,
}
impl PhysicsSystem {
    pub fn new(max_collision_solve_steps: u8, simulation_deltatime: f32) -> Self {
        Self {
            simulation_deltatime,
            gravity: GravitySystem { g: 9.87 },
            rigidbody_system: RigidbodySystem {},
            collision_solver_system: CollisionSolverSystem {
                steps: max_collision_solve_steps,
            },
            last_time: 0.0,
        }
    }
}
impl<'a> System<'a> for PhysicsSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Collision3D>,
        WriteStorage<'a, Rigidbody>,
        WriteStorage<'a, Transform>,
        ReadStorage<'a, Static>,
        Read<'a, Time>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (e, mut c, mut r, mut t, stat, time) = data;
        if self.last_time - time.time() < self.simulation_deltatime {
            self.gravity.run((&mut r, &mut t, &time, &stat));
            self.rigidbody_system.run((&mut r, &mut t,&time));
            self.collision_solver_system
                .run((&e, &mut c, &mut r, &mut t, &stat,&time));
            self.last_time = time.time();
        }
    }
}
