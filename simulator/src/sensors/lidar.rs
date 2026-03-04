use simulator_types::PhysicsState;
use super::Sensor;

/// Lidar ring: casts `num_rays` rays and returns range measurements.
pub struct Lidar {
    pub num_rays: usize,
    /// Maximum measurable range (metres).
    pub max_range: f64,
    /// Standard deviation of additive Gaussian range noise (metres).
    pub noise_stddev: f64,
}

impl Sensor for Lidar {
    fn observe(&self, _state: &PhysicsState) -> Vec<f64> {
        // Stub: returns max_range for every ray.
        vec![self.max_range; self.num_rays]
    }
}
