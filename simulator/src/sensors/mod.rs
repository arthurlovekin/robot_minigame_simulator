use simulator_types::PhysicsState;

pub mod barometer;
pub mod contact;
pub mod engine_feedback;
pub mod imu;
pub mod lidar;

/// A sensor reads the current world snapshot and returns a vector of scalar observations.
pub trait Sensor: Send + Sync {
    fn observe(&self, state: &PhysicsState) -> Vec<f64>;
}
