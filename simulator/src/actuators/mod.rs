pub mod thruster;

use simulator_types::Vec2;

/// An actuator receives a command and returns a body-frame force (N) + torque (N·m).
pub trait Actuator: Send + Sync {
    fn apply(&mut self, input: &[f32], dt: f32) -> (Vec2, f32);
}
