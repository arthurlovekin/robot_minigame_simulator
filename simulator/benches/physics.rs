use criterion::{criterion_group, criterion_main, Criterion};
use simulator::{
    BodyDef, ColliderDef, ColliderShape, ExternalForce, PhysicsEngine, Vec2,
};

fn bench_step(c: &mut Criterion) {
    c.bench_function("PhysicsEngine::step 1/60s", |b| {
        let gravity = Vec2 { x: 0.0, y: -1.62 };
        let mut engine = PhysicsEngine::new(gravity);

        // Ground
        engine.add_body(BodyDef {
            position: Vec2 { x: 0.0, y: -0.5 },
            velocity: Vec2 { x: 0.0, y: 0.0 },
            angle: 0.0,
            angular_velocity: 0.0,
            density: 0.0,
            colliders: vec![ColliderDef {
                shape: ColliderShape::Box { half_width: 20.0, half_height: 0.5 },
                offset: Vec2 { x: 0.0, y: 0.0 },
                angle: 0.0,
                restitution: 0.3,
            }],
            is_static: true,
        });

        // Lander (3 colliders)
        engine.add_body(BodyDef {
            position: Vec2 { x: 0.0, y: 10.0 },
            velocity: Vec2 { x: 0.0, y: 0.0 },
            angle: 0.0,
            angular_velocity: 0.0,
            density: 20.0,
            colliders: vec![
                ColliderDef {
                    shape: ColliderShape::Box { half_width: 0.5, half_height: 0.3 },
                    offset: Vec2 { x: 0.0, y: 0.0 },
                    angle: 0.0,
                    restitution: 0.3,
                },
                ColliderDef {
                    shape: ColliderShape::Box { half_width: 0.075, half_height: 0.2 },
                    offset: Vec2 { x: -0.55, y: -0.5 },
                    angle: 0.0,
                    restitution: 0.3,
                },
                ColliderDef {
                    shape: ColliderShape::Box { half_width: 0.075, half_height: 0.2 },
                    offset: Vec2 { x: 0.55, y: -0.5 },
                    angle: 0.0,
                    restitution: 0.3,
                },
            ],
            is_static: false,
        });

        let forces: Vec<ExternalForce> = vec![];
        b.iter(|| engine.step(1.0 / 60.0, &forces));
    });
}

criterion_group!(benches, bench_step);
criterion_main!(benches);
