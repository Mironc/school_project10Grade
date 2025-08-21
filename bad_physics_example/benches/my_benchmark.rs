use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use engine_3d::{math::vec3, transform::Transform};
use game::physics::collision::{tri_vs_tri, CollisionInfo, Triangle};
#[no_mangle]
fn bench_triangle(triangle:Triangle,transform:Transform,triangle_2:Triangle,transform_2:Transform) -> Option<CollisionInfo>{
    tri_vs_tri(&triangle, &transform, &triangle_2, &transform_2)
} 
fn fibonacci(mut n: u64) -> u64 {
    let mut prev = 0;
    let mut current = 1;
    while n > 1 {
        prev = current;
        current = prev + current;
        n-=1;
    }
    current
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("tri");
    group.significance_level(0.1).sample_size(100000);
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
    group.bench_function("tri vs tri", |b| b.iter(|| bench_triangle(black_box(tri.clone()), black_box(Transform::default()), black_box(tri_1.clone()), black_box(transform_1))));

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);