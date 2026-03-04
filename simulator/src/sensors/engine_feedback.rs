use simulator_types::{BodyId, PhysicsState};

use super::Sensor;

/// Engine feedback sensor stub.
///
/// In practice, the environment reads thruster state directly via
/// `Thruster::current_normalized()` rather than routing through this sensor.
/// This struct is retained for API symmetry.
pub struct EngineFeedback {
    pub body_id: BodyId,
    pub noise_stddev: f32,
}

impl Sensor for EngineFeedback {
    fn observe(&self, _state: &PhysicsState) -> Vec<f32> {
        vec![0.0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn observe_returns_single_value() {
        let sensor = EngineFeedback { body_id: BodyId(0), noise_stddev: 0.0 };
        let state = simulator_types::PhysicsState::default();
        assert_eq!(sensor.observe(&state).len(), 1);
    }
}
