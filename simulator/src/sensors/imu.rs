use simulator_types::{BodyId, PhysicsState};

use super::Sensor;

/// Inertial Measurement Unit: returns linear acceleration and angular velocity.
/// Noise is deferred — `noise_stddev` and `bias_drift` are stored for future use.
pub struct Imu {
    pub body_id: BodyId,
    pub noise_stddev: f32,
    pub bias_drift: f32,
}

impl Sensor for Imu {
    /// Returns `[ax, ay, omega]` for the attached body.
    fn observe(&self, state: &PhysicsState) -> Vec<f32> {
        let body = &state.bodies[self.body_id.0];
        vec![
            body.linear_acceleration.x,
            body.linear_acceleration.y,
            body.angular_velocity,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use simulator_types::{BodyState, ColliderShape, ColliderState, PhysicsState, Vec2};

    #[test]
    fn observe_returns_acceleration_and_omega() {
        let sensor = Imu { body_id: BodyId(0), noise_stddev: 0.0, bias_drift: 0.0 };
        let state = PhysicsState {
            bodies: vec![BodyState {
                position: Vec2 { x: 0.0, y: 0.0 },
                velocity: Vec2 { x: 0.0, y: 0.0 },
                angle: 0.0,
                angular_velocity: 2.5,
                linear_acceleration: Vec2 { x: 1.0, y: -9.81 },
                angular_acceleration: 0.0,
                colliders: vec![ColliderState {
                    shape: ColliderShape::Circle { radius: 0.1 },
                    offset: Vec2 { x: 0.0, y: 0.0 },
                    angle: 0.0,
                }],
            }],
            contacts: vec![],
        };
        let obs = sensor.observe(&state);
        assert_eq!(obs[0], 1.0);
        assert_eq!(obs[1], -9.81);
        assert_eq!(obs[2], 2.5);
    }
}
