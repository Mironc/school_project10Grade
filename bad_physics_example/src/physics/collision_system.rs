use std::time::Instant;

use engine_3d::{
    specs::{Entities, Entity, Join, LendJoin, Read, ReadStorage, WriteStorage},
    time::Time,
    transform::Transform,
};

use super::{collision::Collision3D, gravity::Static, rigidbody::Rigidbody};

pub struct CollisionSystem {}
impl CollisionSystem {
    pub fn run(
        &mut self,
        data: (
            &Entities<'_>,
            &mut WriteStorage<'_, Collision3D>,
            &mut WriteStorage<'_, Rigidbody>,
            &mut WriteStorage<'_, Transform>,
            &ReadStorage<'_, Static>,
        ),
    ) {
        let (entities, mut collision_storage, mut rigidbodies, mut transform_storage, _static) =
            data;
        //let instant = Instant::now();
        let mut physical_objects: Vec<(
            Entity,
            &mut Collision3D,
            &mut Rigidbody,
            &mut Transform,
            Option<&Static>,
        )> = (
            entities,
            collision_storage,
            rigidbodies,
            transform_storage,
            _static.maybe(),
        )
            .join()
            .collect();
        for i in 0..physical_objects.len() {
            physical_objects[i].1.clear_collisions();
        }
        for i in 0..physical_objects.len() - 1 {
            for j in i + 1..physical_objects.len() {
                let split = physical_objects.split_at_mut(i + 1);
                let lhs_id = split.0[i].0.id();
                let lhs_transform = *split.0[i].3;
                let rhs_id = split.1[j - i - 1].0.id();
                let rhs_transform = *split.1[j - i - 1].3;
                let collided = Collision3D::add_collision(
                    split.0[i].1,
                    lhs_id,
                    &lhs_transform,
                    rhs_id,
                    split.1[j - i - 1].1,
                    &rhs_transform,
                );
                //println!("{}",collided.is_some());
            }
        }
        //println!("collecting time {}",instant.elapsed().as_secs_f32());
        for i in 0..physical_objects.len() {
            if physical_objects[i].4.is_some() {
                continue;
            }
            for collision_id in 0..physical_objects[i].1.collisions().len() {
                let velocity = physical_objects[i].2.velocity();
                let collision = physical_objects[i].1.collisions()[collision_id];
                let vel_p = collision.normal() * velocity.dot(collision.normal());
                let new_vel = velocity - vel_p;
                println!("velocity {} new velocity {}", velocity, new_vel);
                physical_objects[i].2.set_velocity(new_vel);

                /* let mult =
                    (collision.normal().signum() * velocity.signum()).clamp(Vec3::ZERO, Vec3::ONE);
                physical_objects[i].2.set_velocity(mult * velocity); */
                /* println!(
                    "{velocity} {} {} {} {}",
                    collision.normal(),
                    collision.penetration(),
                    collision.normal() * collision.penetration(),
                    mult
                ); */
                physical_objects[i].3.position += collision.normal() * collision.penetration();
            }
        }
    }
}

pub struct CollisionSolverSystem {
    pub steps: u8,
}
impl CollisionSolverSystem {
    pub fn run(
        &mut self,
        data: (
            &Entities<'_>,
            &mut WriteStorage<'_, Collision3D>,
            &mut WriteStorage<'_, Rigidbody>,
            &mut WriteStorage<'_, Transform>,
            &ReadStorage<'_, Static>,
            &Read<'_, Time>,
        ),
    ) {
        let (
            entities,
            mut collision_storage,
            mut rigidbodies,
            mut transform_storage,
            _static,
            time,
        ) = data;
        //let instant = Instant::now();
        let mut physical_objects: Vec<(
            Entity,
            &mut Collision3D,
            Option<&mut Rigidbody>,
            &mut Transform,
            Option<&Static>,
        )> = (
            entities,
            collision_storage,
            rigidbodies.maybe(),
            transform_storage,
            _static.maybe(),
        )
            .join()
            .collect();
        let mut solved = Vec::new();
        for step in 0..self.steps {
            solved.clear();
            for i in 0..physical_objects.len() {
                if step != 0 && physical_objects[i].1.collisions().len() == 0 {
                    solved.push(i);
                }
                physical_objects[i].1.clear_collisions();
            }
            for i in 0..physical_objects.len() - 1 {
                if solved.contains(&i) {
                    continue;
                }
                for j in i + 1..physical_objects.len() {
                    let split = physical_objects.split_at_mut(i + 1);
                    if split.0[i].2.is_none() && split.1[j-i-1].2.is_none(){
                        continue;
                    }
                    let lhs_id = split.0[i].0.id();
                    let lhs_transform = *split.0[i].3;
                    let rhs_id = split.1[j - i - 1].0.id();
                    let rhs_transform = *split.1[j - i - 1].3;
                    let collided = Collision3D::add_collision(
                        split.0[i].1,
                        lhs_id,
                        &lhs_transform,
                        rhs_id,
                        split.1[j - i - 1].1,
                        &rhs_transform,
                    );
                }
            }
            for i in 0..physical_objects.len() {
                if physical_objects[i].4.is_some() {
                    continue;
                }
                for collision_id in 0..physical_objects[i].1.collisions().len() {
                    if physical_objects[i].2.is_some() {
                        let velocity = physical_objects[i].2.as_ref().unwrap().velocity();
                        let bounciness = physical_objects[i].2.as_ref().unwrap().bounciness();
                        let friction = physical_objects[i].2.as_ref().unwrap().friction();
                        let collision = physical_objects[i].1.collisions()[collision_id];
                        let vel_p = collision.normal() * velocity.dot(collision.normal());
                        let mut new_vel = velocity - vel_p;
                        //Friction
                        new_vel -= new_vel * friction;
                        //print!("velocity {} ",new_vel.normalize().dot(collision.normal()));
                        physical_objects[i]
                            .2
                            .as_mut()
                            .unwrap()
                            .set_velocity(new_vel);
                        physical_objects[i].3.position +=
                            collision.normal() * collision.penetration();
                    }
                }
            }
        }
    }
}
/*
impl<'_> System<'a> for CollisionSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Collision3D>,
        WriteStorage<'a, Rigidbody>,
        WriteStorage<'a, Transform>,
        ReadStorage<'a, Static>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut collision_storage, mut rigidbodies, mut transform_storage, _static) =
            data;
        let mut physical_objects: Vec<(
            Entity,
            &mut Collision3D,
            &mut Rigidbody,
            &mut Transform,
            Option<&Static>,
        )> = (
            &entities,
            &mut collision_storage,
            &mut rigidbodies,
            &mut transform_storage,
            _static.maybe(),
        )
            .join()
            .collect();
        for i in 0..physical_objects.len() {
            let colliders = physical_objects.split_at_mut(i);
            for j in 0..colliders.1.len() {
                if colliders.0.len() < 1 {
                    break;
                }
                let rhs = colliders.0.last_mut().unwrap();
                let transform = &rhs.3.clone();
                let id = rhs.0.id();
                let collided = Collision3D::add_collision(
                    colliders.0.last_mut().unwrap().1,
                    id,
                    transform,
                    colliders.1[j].0.id(),
                    colliders.1[j].1,
                    colliders.1[j].3,
                );
                //println!("{}", collided.is_some());
            }
        }
        for i in 0..physical_objects.len() {
            if physical_objects[i].4.is_some() {
                continue;
            }
            for collision_id in 0..physical_objects[i].1.collisions().len() {
                let velocity = physical_objects[i].2.velocity();
                let collision = physical_objects[i].1.collisions()[collision_id];
                let mult = (collision.normal().signum()*velocity.signum()).clamp(Vec3::ZERO, Vec3::ONE);
                physical_objects[i].2.set_velocity(mult*velocity);
                println!(
                    "{velocity} {} {} {} {}",
                    collision.normal(),
                    collision.penetration(),
                    collision.normal() * collision.penetration(),
                    mult
                );
                physical_objects[i].3.position += collision.normal() * collision.penetration();
            }
        }
        /* let instant = Instant::now();
        for i in 0..collisions.len() {
            let collisions = collisions.split_at_mut(i);
            for j in 0..collisions.1.len() {
                if collisions.0.len() < 1 {
                    break;
                }
                let transform = collisions.0.last().unwrap().1;
                let collided = Collision3D::collides(
                    collisions.0.last_mut().unwrap().0,
                    transform,
                    collisions.1[j].0,
                    collisions.1[j].1,
                );
                println!("{}", collided);
            }
        }
        println!("{}", instant.elapsed().as_secs_f32()); */
    }
}
 */
