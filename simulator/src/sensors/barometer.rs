use simulator_types::{BodyId, PhysicsState};

use super::Sensor;

/// Barometric altimeter: returns the y-position of the attached body.
pub struct Barometer {
    pub body_id: BodyId,
    pub noise_stddev: f32,
}

impl Sensor for Barometer {
    fn observe(&self, state: &PhysicsState) -> Vec<f32> {
        let y = state.bodies[self.body_id.0].position.y;
        vec![y]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use simulator_types::{BodyState, ColliderState, ColliderShape, PhysicsState, Vec2};

    fn make_state(y: f32) -> PhysicsState {
        PhysicsState {
            bodies: vec![BodyState {
                position: Vec2 { x: 0.0, y },
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
            }],
            contacts: vec![],
        }
    }

    #[test]
    fn observe_returns_body_y_position() {
        let sensor = Barometer { body_id: BodyId(0), noise_stddev: 0.0 };
        let state = make_state(5.5);
        let obs = sensor.observe(&state);
        assert_eq!(obs, vec![5.5]);
    }
}
