use simulator_types::PhysicsState;
use super::Sensor;

/// Inertial Measurement Unit: returns linear acceleration and angular velocity.
pub struct Imu {
    /// Standard deviation of additive Gaussian noise (m/s², rad/s).
    pub noise_stddev: f64,
    /// Rate at which bias drifts per second (m/s², rad/s).
    pub bias_drift: f64,
}

impl Sensor for Imu {
    fn observe(&self, _state: &PhysicsState) -> Vec<f64> {
        // Stub: returns zeros.
        vec![0.0; 3] // [ax, ay, omega]
    }
}
