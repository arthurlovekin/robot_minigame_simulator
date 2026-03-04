use simulator_types::PhysicsState;
use super::Sensor;

/// Engine feedback sensor: reads back actual thruster RPM/thrust level.
pub struct EngineFeedback {
    /// Standard deviation of additive Gaussian noise (normalised 0–1).
    pub noise_stddev: f64,
}

impl Sensor for EngineFeedback {
    fn observe(&self, _state: &PhysicsState) -> Vec<f64> {
        // Stub: returns zero thrust readback.
        vec![0.0]
    }
}
