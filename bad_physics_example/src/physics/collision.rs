use std::time::Instant;

use engine_3d::{
    graphics::objects::{
        model::{InstancedModel, Model},
        vertex::ModelVertex,
    },
    math::{vec3, Mat3A, Vec3, Vec3A, Vec3Swizzles},
    specs::{Component, VecStorage},
    transform::Transform,
};
use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct Collision3D {
    shape: Shape,
    collisions: Vec<ObjectCollisionInfo>,
}

impl Collision3D {
    pub fn new(shape: Shape) -> Self {
        Self {
            shape,
            collisions: Vec::new(),
        }
    }
    pub fn shape(&self) -> &Shape {
        &self.shape
    }
    pub fn collisions(&self) -> &Vec<ObjectCollisionInfo> {
        &self.collisions
    }
    pub fn collides(
        &mut self,
        transform: &Transform,
        other: &Self,
        transform_other: &Transform,
    ) -> bool {
        self.shape
            .collides(transform, &other.shape, transform_other)
    }
    pub fn add_collision(
        &mut self,
        lhs_id: u32,
        lhs_transform: &Transform,
        rhs_id: u32,
        rhs: &mut Self,
        rhs_transform: &Transform,
    ) -> Option<CollisionInfo> {
        let collision = self
            .shape
            .collision(lhs_transform, &rhs.shape, rhs_transform);
        println!(
            "collision between {} {} is {}",
            lhs_id,
            rhs_id,
            collision.is_some()
        );
        if let Some(mut collision) = collision {
            collision.collider_1 = lhs_id;
            collision.collider_2 = rhs_id;
            let c = collision.break_down();
            self.collisions.push(c.0);
            rhs.collisions.push(c.1);
        }
        collision
    }
    pub fn clear_collisions(&mut self) {
        self.collisions.clear();
    }
}
impl Component for Collision3D {
    type Storage = VecStorage<Self>;
}
#[derive(Debug, Clone, Copy)]
pub struct CollisionInfo {
    collider_1: u32,
    collider_2: u32,
    normal_lhs: Vec3,
    normal_rhs: Vec3,
    penetration: f32,
}
impl CollisionInfo {
    pub fn new(normal_lhs: Vec3, normal_rhs: Vec3, penetration: f32) -> Self {
        Self {
            collider_1: 0,
            collider_2: 0,
            normal_lhs,
            normal_rhs,
            penetration,
        }
    }
    pub fn normal_lhs(&self) -> Vec3 {
        self.normal_lhs
    }
    pub fn normal_rhs(&self) -> Vec3 {
        self.normal_rhs
    }
    pub fn penetration(&self) -> f32 {
        self.penetration
    }
    pub fn break_down(&self) -> (ObjectCollisionInfo, ObjectCollisionInfo) {
        (
            ObjectCollisionInfo::new(self.collider_2, self.normal_lhs, self.penetration),
            ObjectCollisionInfo::new(self.collider_1, self.normal_rhs, self.penetration),
        )
    }
    fn set_colliders(&mut self, collider_1: u32, collider_2: u32) {
        self.collider_1 = collider_1;
        self.collider_2 = collider_2;
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ObjectCollisionInfo {
    other_obj: u32,
    normal: Vec3,
    penetration: f32,
}
impl ObjectCollisionInfo {
    pub fn new(other_obj: u32, normal: Vec3, penetration: f32) -> Self {
        Self {
            other_obj,
            normal,
            penetration,
        }
    }
    pub fn normal(&self) -> Vec3 {
        self.normal
    }
    pub fn penetration(&self) -> f32 {
        self.penetration
    }
    pub fn other_object(&self) -> u32 {
        self.other_obj
    }
}
#[derive(Debug, Clone)]
pub enum Shape {
    Mesh(MeshShape),
    Sphere(SphereShape),
    Box(BoxShape),
}
impl Shape {
    pub fn collides(
        &self,
        lhs_transform: &Transform,
        other: &Self,
        rhs_transform: &Transform,
    ) -> bool {
        match self {
            Shape::Mesh(mesh_shape) => match other {
                Shape::Mesh(mesh_shape_1) => {
                    mesh_vs_mesh(mesh_shape, lhs_transform, mesh_shape_1, rhs_transform).is_some()
                }
                Shape::Sphere(sphere_shape) => {
                    mesh_vs_sphere(mesh_shape, lhs_transform, sphere_shape, rhs_transform).is_some()
                }
                Shape::Box(box_shape) => {
                    mesh_vs_box(mesh_shape, lhs_transform, box_shape, rhs_transform).is_some()
                }
            },
            Shape::Sphere(sphere_shape) => match other {
                Shape::Mesh(mesh_shape) => {
                    mesh_vs_sphere(mesh_shape, rhs_transform, sphere_shape, lhs_transform).is_some()
                }
                Shape::Sphere(sphere_shape_1) => {
                    sphere_vs_sphere(sphere_shape, lhs_transform, sphere_shape_1, rhs_transform)
                        .is_some()
                }
                Shape::Box(box_shape) => {
                    sphere_vs_box(sphere_shape, lhs_transform, box_shape, rhs_transform).is_some()
                }
            },
            Shape::Box(box_shape) => match other {
                Shape::Mesh(mesh_shape) => {
                    mesh_vs_box(mesh_shape, rhs_transform, box_shape, lhs_transform).is_some()
                }
                Shape::Sphere(sphere_shape) => {
                    sphere_vs_box(sphere_shape, rhs_transform, box_shape, lhs_transform).is_some()
                }
                Shape::Box(box_shape_1) => {
                    box_vs_box(box_shape, lhs_transform, box_shape_1, rhs_transform).is_some()
                }
            },
        }
    }
    pub fn collision(
        &self,
        lhs_transform: &Transform,
        other: &Self,
        rhs_transform: &Transform,
    ) -> Option<CollisionInfo> {
        match self {
            Shape::Mesh(mesh_shape) => match other {
                Shape::Mesh(mesh_shape_1) => {
                    mesh_vs_mesh(mesh_shape, lhs_transform, mesh_shape_1, rhs_transform)
                }
                Shape::Sphere(sphere_shape) => {
                    mesh_vs_sphere(mesh_shape, lhs_transform, sphere_shape, rhs_transform)
                }
                Shape::Box(box_shape) => {
                    mesh_vs_box(mesh_shape, lhs_transform, box_shape, rhs_transform)
                }
            },
            Shape::Sphere(sphere_shape) => match other {
                Shape::Mesh(mesh_shape) => {
                    mesh_vs_sphere(mesh_shape, rhs_transform, sphere_shape, lhs_transform)
                }
                Shape::Sphere(sphere_shape_1) => {
                    sphere_vs_sphere(sphere_shape, lhs_transform, sphere_shape_1, rhs_transform)
                }
                Shape::Box(box_shape) => {
                    sphere_vs_box(sphere_shape, lhs_transform, box_shape, rhs_transform)
                }
            },
            Shape::Box(box_shape) => match other {
                Shape::Mesh(mesh_shape) => {
                    mesh_vs_box(mesh_shape, rhs_transform, box_shape, lhs_transform)
                }
                Shape::Sphere(sphere_shape) => {
                    sphere_vs_box(sphere_shape, rhs_transform, box_shape, lhs_transform)
                }
                Shape::Box(box_shape_1) => {
                    box_vs_box(box_shape, lhs_transform, box_shape_1, rhs_transform)
                }
            },
        }
    }
}
#[derive(Debug, Clone)]
pub struct Triangle {
    //aligling makes everything faster on 1% avg
    verts: [Vec3A; 3],
    normal: Vec3A,
}
impl Triangle {
    #[inline]
    pub fn new(verts: [Vec3; 3], normal: Vec3) -> Self {
        let verts = [verts[0].into(), verts[1].into(), verts[2].into()];
        Self {
            verts,
            normal: normal.into(),
        }
    }
    #[inline]
    pub fn normal(&self) -> Vec3A {
        self.normal
    }
    #[inline]
    pub fn midpoint(&self) -> Vec3A {
        self.verts.iter().fold(Vec3A::ZERO, |acc, v| acc + v) / 3.0
    }
    #[inline]
    pub fn sphere(&self) -> (Vec3A, f32) {
        (
            self.midpoint(),
            self.verts
                .iter()
                .max_by(|x, y| x.length_squared().total_cmp(&y.length_squared()))
                .unwrap()
                .length(),
        )
    }
    /*
    pub fn ab(&self) -> Vec3 {
        self.verts[1] - self.verts[0]
    }
    pub fn ac(&self) -> Vec3 {
        self.verts[2] - self.verts[0]
    }
    pub fn bc(&self) -> Vec3 {
        self.verts[2] - self.verts[1]
    } */
    #[inline]
    pub fn transform(&self, transform: &Transform) -> Self {
        let mut new = Self::new([Vec3::ZERO; 3], Vec3::ZERO);
        for i in 0..3 {
            new.verts[i] = transform.get_matrix().transform_point3a(self.verts[i]);
        }
        new.normal = transform
            .get_rotation_matrix()
            .transform_vector3a(self.normal)
            .normalize();
        new
    }
}
#[derive(Debug, Clone)]
pub struct MeshShape {
    triangles: Vec<Triangle>,
    debug: InstancedModel,
}
impl MeshShape {
    pub fn new(mesh: Model<ModelVertex>) -> Self {
        let debug = mesh.instantiate();
        let mut triangles = Vec::new();
        if let Some(indicies) = mesh.indicies {
            for mut triangle in &indicies.into_iter().chunks(3) {
                let p1 = triangle.next().unwrap() as usize;
                let p2 = triangle.next().unwrap() as usize;
                let p3 = triangle.next().unwrap() as usize;
                let p1 = mesh.verticies[p1].position.into();
                let p2 = mesh.verticies[p2].position.into();
                let p3 = mesh.verticies[p3].position.into();
                triangles.push(Triangle::new([p1, p2, p3], (p2 - p1).cross(p3 - p1)));
            }
        } else {
            for mut triangle in &mesh.verticies.into_iter().chunks(3) {
                let p1 = triangle.next().unwrap().position.into();
                let p2 = triangle.next().unwrap().position.into();
                let p3 = triangle.next().unwrap().position.into();
                triangles.push(Triangle::new([p1, p2, p3], (p1 - p2).cross(p2 - p3)));
            }
        }
        Self { triangles, debug }
    }
    pub fn debug(&self) -> &InstancedModel {
        &self.debug
    }
}
#[derive(Debug, Clone)]
pub struct BoxShape {
    size: Vec3,
    centre: Vec3,
}
impl BoxShape {
    pub fn new(centre: Vec3, size: Vec3) -> Self {
        Self { size, centre }
    }
    pub fn size(&self) -> Vec3 {
        self.size
    }
    pub fn centre(&self) -> Vec3 {
        self.centre
    }
}
#[derive(Debug, Clone)]
pub struct SphereShape {
    radius: f32,
    centre: Vec3,
}
impl SphereShape {
    pub fn new(centre: Vec3, radius: f32) -> Self {
        Self { radius, centre }
    }
    pub fn centre(&self) -> Vec3 {
        self.centre
    }
    pub fn radius(&self) -> f32 {
        self.radius
    }
}
///triangle
#[no_mangle]
pub fn tri_vs_tri(
    lhs_tri: &Triangle,
    lhs_transform: &Transform,
    rhs_tri: &Triangle,
    rhs_transform: &Transform,
) -> Option<CollisionInfo> {
    let lhs_tri = lhs_tri.transform(lhs_transform);
    let rhs_tri = rhs_tri.transform(rhs_transform);

    let mut signed_distances_rhs = Vec3A::ZERO;

    //plane equation
    let abc1 = lhs_tri.normal();
    let d1 = -lhs_tri.normal().dot(lhs_tri.verts[0]);

    for i in 0..3 {
        let signed_distance = abc1.dot(rhs_tri.verts[i]) + d1;
        signed_distances_rhs[i] = signed_distance;
    }
    let mut signed_distances_lhs = Vec3A::ZERO;
    //plane equation
    let abc2 = rhs_tri.normal();
    let d2 = -rhs_tri.normal().dot(rhs_tri.verts[0]);
    for i in 0..3 {
        let signed_distance = abc2.dot(lhs_tri.verts[i]) + d2;
        signed_distances_lhs[i] = signed_distance;
    }
    //println!("{} {}", acc_1, acc_2);
    let (rhs_d0d1, rhs_d0d2) = (
        signed_distances_rhs[0] * signed_distances_rhs[1],
        signed_distances_rhs[0] * signed_distances_rhs[2],
    );
    let (lhs_d0d1, lhs_d0d2) = (
        signed_distances_lhs[0] * signed_distances_lhs[1],
        signed_distances_lhs[0] * signed_distances_lhs[2],
    );
    if (lhs_d0d1 > 0.0 && lhs_d0d2 > 0.0) || (rhs_d0d1 > 0.0 && rhs_d0d2 > 0.0) {
        return None;
    }
    let (sph1,sph2) = (lhs_tri.sphere(),rhs_tri.sphere());
    if (sph1.0-sph2.0).length_squared() >= (sph1.1+sph2.1).powi(2) {
        return None;
    }

    let intersection_line = abc1.cross(abc2).normalize();
    let mut max_i = 0;
    let intersection_line_abs = intersection_line.abs();
    if intersection_line_abs[max_i] < intersection_line_abs[1] {
        max_i = 1;
    }
    if intersection_line_abs[max_i] < intersection_line_abs[2] {
        max_i = 2;
    }
    //println!("axis {} {} ", max_i, intersection_line);
    let pvs_1 = Vec3A::new(
        lhs_tri.verts[0][max_i],
        lhs_tri.verts[1][max_i],
        lhs_tri.verts[2][max_i],
    );
    let pvs_2 = Vec3A::new(
        rhs_tri.verts[0][max_i],
        rhs_tri.verts[1][max_i],
        rhs_tri.verts[2][max_i],
    );
    let (line_t1, line_t2) = intervals(pvs_1, signed_distances_lhs, lhs_d0d1, lhs_d0d2);
    let (line_t1_2, line_t2_2) = intervals(pvs_2, signed_distances_rhs, rhs_d0d1, rhs_d0d2);
    let (max_1, min_1) = (line_t1.max(line_t2), line_t1.min(line_t2));
    let (max_2, min_2) = (line_t1_2.max(line_t2_2), line_t1_2.min(line_t2_2));
    if max_1 < min_2 || max_2 < min_1 {
        return None;
    }
    let min_d_lhs = signed_distances_lhs.min_element().abs();
    let min_d_rhs = signed_distances_rhs.min_element().abs();
    //println!("signed {}",signed_distances_lhs[distinct_lhs]);
    //println!("signed {}",signed_distances_rhs[distinct_rhs]);
    Some(CollisionInfo::new(
        rhs_tri.normal().into(),
        lhs_tri.normal().into(),
        min_d_lhs.min(min_d_rhs),
    ))
}
#[inline(always)]
fn intervals(vv: Vec3A, sd: Vec3A, d0d1: f32, d0d2: f32) -> (f32, f32) {
    let isect = |vv: Vec3A, sd: Vec3A| {
        (
            vv[0] + (vv[1] - vv[0]) * sd[0] / (sd[0] - sd[1]),
            vv[0] + (vv[2] - vv[0]) * sd[0] / (sd[0] - sd[2]),
        )
    };
    if d0d1 > 0.0 {
        //println!("D0D1 > 0.0");
        return isect(vv.zxy(), sd.zxy());
    } else if d0d2 > 0.0 {
        //println!("D0D2 > 0.0");
        return isect(vv.yxz(), sd.yxz());
    } else if sd[1] * sd[2] > 0.0 || sd[0] != 0.0 {
        //println!("D1 * D2 > 0.0 || D0 != 0.0");
        //println!("zero");
        return isect(vv, sd);
    } else {
        //println!("coplanar");
        return (0.0, 0.0);
    }
}
pub fn tri_vs_sphere(
    triangle: &Triangle,
    lhs_transform: &Transform,
    sphere: &SphereShape,
    rhs_transform: &Transform,
) -> Option<CollisionInfo> {
    let triangle = triangle.transform(lhs_transform);
    let centre = sphere.centre + rhs_transform.position;
    let closest = closest_point_on_triangle_to_point(&triangle, &centre);
    let length = centre.distance(closest);
    if length <= sphere.radius() {
        return Some(CollisionInfo::new(
            triangle.normal.into(),
            closest,
            length - sphere.radius(),
        ));
    }
    None
}

///Taken from Real-Time Collision Detection by Christer Ericson (Sony Computer Entertainment America)
fn closest_point_on_triangle_to_point(triangle: &Triangle, p: &Vec3) -> Vec3 {
    let p: Vec3A = (*p).into();
    let a = triangle.verts[0];
    let b = triangle.verts[1];
    let c = triangle.verts[2];
    // Check if P in vertex region outside A
    let ab = b - a;
    let ac = c - a;
    let ap = p - a;
    let d1 = Vec3A::dot(ab, ap);
    let d2 = Vec3A::dot(ac, ap);
    if d1 <= 0.0 && d2 <= 0.0 {
        return a.into();
    } // barycentric coordinates (1,0,0)
      // Check if P in vertex region outside B
    let bp = p - b;
    let d3 = Vec3A::dot(ab, bp);
    let d4 = Vec3A::dot(ac, bp);
    if d3 >= 0.0 && d4 <= d3 {
        return b.into();
    } // barycentric coordinates (0,1,0)
      // Check if P in edge region of AB, if so return projection of P onto AB
    let vc = d1 * d4 - d3 * d2;
    if vc <= 0.0 && d1 >= 0.0 && d3 <= 0.0 {
        let v = d1 / (d1 - d3);
        return (a + v * ab).into(); // barycentric coordinates (1-v,v,0)
    }
    // Check if P in vertex region outside C
    let cp = p - c;
    let d5 = Vec3A::dot(ab, cp);
    let d6 = Vec3A::dot(ac, cp);
    if d6 >= 0.0 && d5 <= d6 {
        return c.into();
    } // barycentric coordinates (0,0,1)
      // Check if P in edge region of AC, if so return projection of P onto AC
    let vb = d5 * d2 - d1 * d6;
    if vb <= 0.0 && d2 >= 0.0 && d6 <= 0.0 {
        let w = d2 / (d2 - d6);
        return (a + w * ac).into(); // barycentric coordinates (1-w,0,w)
    }
    // Check if P in edge region of BC, if so return projection of P onto BC
    let va = d3 * d6 - d5 * d4;
    if va <= 0.0 && (d4 - d3) >= 0.0 && (d5 - d6) >= 0.0 {
        let w = (d4 - d3) / ((d4 - d3) + (d5 - d6));
        return (b + w * (c - b)).into(); // barycentric coordinates (0,1-w,w)
    }
    // P inside face region. Compute Q through its barycentric coordinates (u,v,w)
    let denom = 1.0 / (va + vb + vc);
    let v = vb * denom;
    let w = vc * denom;
    return (a + ab * v + ac * w).into(); // = u*a +v*b+w*c,u=va* denom = 1.0f-v-w
}
pub fn tri_vs_box(
    triangle: &Triangle,
    lhs_transform: &Transform,
    rhs: &BoxShape,
    rhs_transform: &Transform,
) -> bool {
    let half = rhs.size * vec3(0.5, 0.5, 0.5);
    //TODO!:
    triangle.verts.iter().any(move |x| {
        todo!();
        //let delta = ((lhs_transform.position + x) - (rhs.centre + rhs_transform.position)).abs();
        //let closest = delta.clamp(-half, half).normalize();
        false;
    })
}
///Mesh collision check
pub fn mesh_vs_mesh(
    mesh: &MeshShape,
    lhs_transform: &Transform,
    mesh_1: &MeshShape,
    rhs_transform: &Transform,
) -> Option<CollisionInfo> {
    mesh.triangles.iter().find_map(|x| {
        mesh_1.triangles.iter().find_map(|y| {
            let res = tri_vs_tri(x, lhs_transform, y, rhs_transform);
            res
        })
    })
}
pub fn mesh_vs_sphere(
    mesh: &MeshShape,
    lhs_transform: &Transform,
    sphere: &SphereShape,
    rhs_transform: &Transform,
) -> Option<CollisionInfo> {
    mesh.triangles
        .iter()
        .find_map(|x| tri_vs_sphere(x, lhs_transform, sphere, rhs_transform))
}
pub fn mesh_vs_box(
    mesh: &MeshShape,
    lhs_transform: &Transform,
    _box: &BoxShape,
    rhs_transform: &Transform,
) -> Option<CollisionInfo> {
    mesh.triangles
        .iter()
        .any(|x| tri_vs_box(x, lhs_transform, _box, rhs_transform));
    None
}
/// Sphere Collision check
pub fn sphere_vs_sphere(
    lhs: &SphereShape,
    lhs_transform: &Transform,
    rhs: &SphereShape,
    rhs_transform: &Transform,
) -> Option<CollisionInfo> {
    let delta = (lhs_transform.position + lhs.centre) - (rhs.centre + rhs_transform.position);
    let delta_len = delta.length_squared();
    if delta_len <= (lhs.radius + rhs.radius).powi(2) {
        let penetration = delta_len - (lhs.radius + rhs.radius);
        let normal = delta.normalize();
        let point = normal * lhs.radius;
        return Some(CollisionInfo::new(normal, point, penetration));
    }
    None
}
pub fn box_vs_box(
    lhs: &BoxShape,
    lhs_transform: &Transform,
    rhs: &BoxShape,
    rhs_transform: &Transform,
) -> Option<CollisionInfo> {
    let delta = (lhs_transform.position + lhs.centre) - (rhs.centre + rhs_transform.position);
    let halfl = lhs.size * vec3(0.5, 0.5, 0.5);
    let halfr = rhs.size * vec3(0.5, 0.5, 0.5);
    let half_sum = halfl + halfr;
    let delta = delta.abs() - half_sum;
    if (delta.x >= 0.0) || (delta.y >= 0.0) || (delta.z >= 0.0) {
        return None;
    }
    Some(CollisionInfo::new(Vec3::ZERO, Vec3::ZERO, delta.length()))
}
pub fn sphere_vs_box(
    lhs: &SphereShape,
    lhs_transform: &Transform,
    rhs: &BoxShape,
    rhs_transform: &Transform,
) -> Option<CollisionInfo> {
    let halfr = rhs.size / 2.0;
    println!("{}", halfr);
    let delta = (lhs_transform.position + lhs.centre) - (rhs.centre + rhs_transform.position);
    let closest = delta.clamp(-halfr, halfr);
    let length = (closest - delta).length_squared();
    if length < lhs.radius * lhs.radius {
        return Some(CollisionInfo::new(
            delta.normalize(),
            closest,
            lhs.radius() - length.sqrt(),
        ));
    }
    None
}
#[allow(unused_imports)]
mod test {
    use crate::physics::collision::{
        box_vs_box, sphere_vs_box, sphere_vs_sphere, tri_vs_tri, BoxShape, SphereShape, Triangle,
    };
    use engine_3d::{math::vec3, transform::Transform};

    #[test]
    fn test_sph_vs_sph() {
        let transform = Transform::default();
        let fst = SphereShape::new(vec3(2.0, 0.0, 0.0), 1.0);
        let scnd = SphereShape::new(vec3(0.0, 0.0, 0.0), 2.0);
        debug_assert!(sphere_vs_sphere(&fst, &transform, &scnd, &transform).is_some());
        let thrd = SphereShape::new(vec3(0.0, 3.3, 0.0), 1.0);
        debug_assert!(!sphere_vs_sphere(&fst, &transform, &thrd, &transform).is_some());
    }
    #[test]
    fn test_box_vs_box() {
        let transform = Transform::default();
        let fst = BoxShape::new(vec3(0.0, 0.0, 0.0), vec3(2.0, 1.0, 1.0));
        let scnd = BoxShape::new(vec3(0.0, 0.9, 0.0), vec3(2.0, 1.0, 1.0));
        debug_assert!(box_vs_box(&fst, &transform, &scnd, &transform).is_some());
        let thrd = BoxShape::new(vec3(0.0, 2.0, 0.0), vec3(2.0, 1.0, 1.0));
        debug_assert!(!box_vs_box(&fst, &transform, &thrd, &transform).is_some());
        let frth = BoxShape::new(vec3(1.0, 0.0, 0.0), vec3(2.0, 1.0, 1.0));
        debug_assert!(!box_vs_box(&fst, &transform, &frth, &transform).is_some());
    }
    #[test]
    pub fn test_box_vs_sph() {
        let transform = Transform::default();
        let _box = BoxShape::new(vec3(0.0, 0.0, 0.0), vec3(1.0, 1.0, 1.0));
        let sph = SphereShape::new(vec3(0.0, 0.0, 0.0), 2.0);
        let sph_scnd = SphereShape::new(vec3(-1.0, 1.0, 0.0), 0.7);
        debug_assert!(sphere_vs_box(&sph, &transform, &_box, &transform).is_some());
        debug_assert!(!sphere_vs_box(&sph_scnd, &transform, &_box, &transform).is_some());
    }
    #[test]
    pub fn test_mesh_vs_mesh() {}
    #[test]
    pub fn test_tri_vs_tri() {
        let transform = Transform::default();
        let transform_1 = Transform::from_position(vec3(2.0, 0.0, 0.0));
        let verts = [
            vec3(0.0, 0.0, 0.0),
            vec3(2.0, 2.0, 1.0),
            vec3(0.0, -1.0, 2.0),
        ];
        let normal = (verts[0] - verts[1]).cross(verts[1] - verts[2]).normalize();
        let tri = Triangle::new(verts, normal);
        let verts = [
            vec3(-2.0, -2.0, 1.0),
            vec3(0.9, 1.0, 0.0),
            vec3(2.0, 1.0, 2.0),
        ];
        let normal = (verts[0] - verts[1]).cross(verts[1] - verts[2]).normalize();
        let tri_1 = Triangle::new(verts, normal);
        debug_assert!(tri_vs_tri(&tri, &transform, &tri_1, &transform).is_some());
        //debug_assert!(!tri_vs_tri(&tri, &transform, &tri_1, &transform_1));

        let transform_1 = Transform::from_position(vec3(0.0, 1.0, 0.0));
        let verts = [
            vec3(2.0, -0.0, 1.0),
            vec3(1.0, 1.0, 0.0),
            vec3(-2.0, 0.0, 1.0),
        ];
        let normal = (verts[0] - verts[1]).cross(verts[1] - verts[2]).normalize();
        let tri = Triangle::new(verts, normal);
        let verts = [
            vec3(2.0, -0.0, 1.0),
            vec3(1.0, 1.0, 0.0),
            vec3(-2.0, 0.0, 1.0),
        ];
        let normal = (verts[0] - verts[1]).cross(verts[1] - verts[2]).normalize();
        let tri_1 = Triangle::new(verts, normal);
        debug_assert!(!tri_vs_tri(&tri, &transform, &tri_1, &transform_1).is_some())
    }
}
