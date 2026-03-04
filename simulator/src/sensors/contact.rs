use simulator_types::{BodyId, PhysicsState};

use super::Sensor;

/// Boolean ground-contact sensor: returns 1.0 if body is in any contact, 0.0 otherwise.
pub struct ContactSensor {
    pub body_id: BodyId,
}

impl Sensor for ContactSensor {
    fn observe(&self, state: &PhysicsState) -> Vec<f32> {
        let in_contact =
            state.contacts.iter().any(|c| c.body_a == self.body_id || c.body_b == self.body_id);
        vec![if in_contact { 1.0 } else { 0.0 }]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use simulator_types::{BodyState, ColliderShape, ColliderState, Contact, PhysicsState, Vec2};

    fn empty_body_state() -> BodyState {
        BodyState {
            position: Vec2 { x: 0.0, y: 0.0 },
            velocity: Vec2 { x: 0.0, y: 0.0 },
            angle: 0.0,
            angular_velocity: 0.0,
            linear_acceleration: Vec2 { x: 0.0, y: 0.0 },
            angular_acceleration: 0.0,
            colliders: vec![ColliderState {
                shape: ColliderShape::Circle { radius: 0.1 },
                offset: Vec2 { x: 0.0, y: 0.0 },
                angle: 0.0,
            }],
        }
    }

    #[test]
    fn returns_one_when_body_in_contact() {
        let sensor = ContactSensor { body_id: BodyId(0) };
        let state = PhysicsState {
            bodies: vec![empty_body_state(), empty_body_state()],
            contacts: vec![Contact { body_a: BodyId(0), body_b: BodyId(1), impulse: 10.0 }],
        };
        assert_eq!(sensor.observe(&state), vec![1.0]);
    }

    #[test]
    fn returns_zero_when_no_contact() {
        let sensor = ContactSensor { body_id: BodyId(0) };
        let state = PhysicsState {
            bodies: vec![empty_body_state(), empty_body_state()],
            contacts: vec![],
        };
        assert_eq!(sensor.observe(&state), vec![0.0]);
    }
}
