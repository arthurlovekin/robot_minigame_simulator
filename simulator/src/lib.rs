pub mod actuators;
pub mod sensors;
mod body;
mod collision;

pub use body::{BodyDef, ColliderDef, ExternalForce};
pub use simulator_types::{
    BodyId, BodyState, ColliderShape, ColliderState, Contact, PhysicsState, Vec2,
};

use body::Body;

/// Core physics engine.
pub struct PhysicsEngine {
    bodies: Vec<Body>,
    gravity: Vec2,
}

impl PhysicsEngine {
    #[must_use]
    pub fn new(gravity: Vec2) -> Self {
        PhysicsEngine { bodies: Vec::new(), gravity }
    }

    /// Add a body to the simulation and return its stable handle.
    pub fn add_body(&mut self, def: BodyDef) -> BodyId {
        let id = BodyId(self.bodies.len());
        self.bodies.push(Body::from_def(def));
        id
    }

    /// Advance the simulation by `dt` seconds and return the new world snapshot.
    #[must_use]
    pub fn step(&mut self, dt: f32, forces: &[ExternalForce]) -> PhysicsState {
        // 1. Integrate: semi-implicit Euler (velocity first, then position)
        for (idx, body) in self.bodies.iter_mut().enumerate() {
            if body.is_static {
                body.linear_acceleration = Vec2 { x: 0.0, y: 0.0 };
                body.angular_acceleration = 0.0;
                continue;
            }

            let mut total_force = self.gravity * body.mass;
            let mut total_torque = 0.0_f32;
            for ef in forces {
                if ef.body.0 == idx {
                    total_force = total_force + ef.force;
                    total_torque += ef.torque;
                }
            }

            let lin_acc = total_force * body.inv_mass;
            let ang_acc = total_torque * body.inv_moi;
            body.linear_acceleration = lin_acc;
            body.angular_acceleration = ang_acc;

            body.velocity = body.velocity + lin_acc * dt;
            body.angular_velocity += ang_acc * dt;
            body.position = body.position + body.velocity * dt;
            body.angle += body.angular_velocity * dt;
        }

        // 2. Detect contacts
        let raw_contacts = collision::detect_contacts(&self.bodies);

        // 3. Resolve contacts with impulses
        let mut contacts: Vec<Contact> = Vec::with_capacity(raw_contacts.len());
        for (ia, ib, normal, depth) in &raw_contacts {
            let impulse = collision::apply_impulse(&mut self.bodies, *ia, *ib, *normal, *depth);
            contacts.push(Contact { body_a: BodyId(*ia), body_b: BodyId(*ib), impulse });
        }

        // 4. Build PhysicsState snapshot
        let body_states = self
            .bodies
            .iter()
            .map(|b| BodyState {
                position: b.position,
                velocity: b.velocity,
                angle: b.angle,
                angular_velocity: b.angular_velocity,
                linear_acceleration: b.linear_acceleration,
                angular_acceleration: b.angular_acceleration,
                colliders: b
                    .colliders
                    .iter()
                    .map(|c| ColliderState { shape: c.shape.clone(), offset: c.offset, angle: c.angle })
                    .collect(),
            })
            .collect();

        PhysicsState { bodies: body_states, contacts }
    }
}

impl Default for PhysicsEngine {
    fn default() -> Self {
        Self::new(Vec2 { x: 0.0, y: -9.81 })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use body::{BodyDef, ColliderDef};

    fn dynamic_box(y: f32) -> BodyDef {
        BodyDef {
            position: Vec2 { x: 0.0, y },
            velocity: Vec2 { x: 0.0, y: 0.0 },
            angle: 0.0,
            angular_velocity: 0.0,
            density: 1.0,
            colliders: vec![ColliderDef {
                shape: ColliderShape::Box { half_width: 0.5, half_height: 0.5 },
                offset: Vec2 { x: 0.0, y: 0.0 },
                angle: 0.0,
                restitution: 0.5,
            }],
            is_static: false,
        }
    }

    #[test]
    fn gravity_increases_downward_velocity() {
        let mut engine = PhysicsEngine::new(Vec2 { x: 0.0, y: -9.81 });
        engine.add_body(dynamic_box(10.0));
        let s1 = engine.step(1.0 / 60.0, &[]);
        let s2 = engine.step(1.0 / 60.0, &[]);
        assert!(s2.bodies[0].velocity.y < s1.bodies[0].velocity.y);
    }

    #[test]
    fn static_body_does_not_move_under_gravity() {
        let mut engine = PhysicsEngine::new(Vec2 { x: 0.0, y: -9.81 });
        engine.add_body(BodyDef {
            position: Vec2 { x: 0.0, y: 0.0 },
            velocity: Vec2 { x: 0.0, y: 0.0 },
            angle: 0.0,
            angular_velocity: 0.0,
            density: 0.0,
            colliders: vec![ColliderDef {
                shape: ColliderShape::Box { half_width: 10.0, half_height: 0.5 },
                offset: Vec2 { x: 0.0, y: 0.0 },
                angle: 0.0,
                restitution: 0.5,
            }],
            is_static: true,
        });
        let s1 = engine.step(1.0 / 60.0, &[]);
        let s2 = engine.step(1.0 / 60.0, &[]);
        assert_eq!(s1.bodies[0].position, s2.bodies[0].position);
    }
}
