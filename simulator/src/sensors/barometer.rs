use simulator_types::PhysicsState;
use super::Sensor;

/// Barometric altimeter: returns altitude above the reference ground plane.
pub struct Barometer {
    /// Standard deviation of additive Gaussian noise (metres).
    pub noise_stddev: f64,
}

impl Sensor for Barometer {
    fn observe(&self, _state: &PhysicsState) -> Vec<f64> {
        // Stub: returns zero altitude.
        vec![0.0]
    }
}
