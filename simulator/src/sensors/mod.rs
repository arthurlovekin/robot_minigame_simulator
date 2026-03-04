pub mod barometer;
pub mod contact;
pub mod engine_feedback;
pub mod imu;
pub mod lidar;

use simulator_types::PhysicsState;

/// A sensor reads the current world snapshot and returns scalar observations.
pub trait Sensor: Send + Sync {
    fn observe(&self, state: &PhysicsState) -> Vec<f32>;
}
