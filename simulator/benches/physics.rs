use criterion::{criterion_group, criterion_main, Criterion};
use simulator::PhysicsEngine;

fn bench_step(c: &mut Criterion) {
    c.bench_function("PhysicsEngine::step 1/60s", |b| {
        let mut engine = PhysicsEngine::new();
        b.iter(|| engine.step(1.0 / 60.0));
    });
}

criterion_group!(benches, bench_step);
criterion_main!(benches);
