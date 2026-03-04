use simulator_types::PhysicsState;

pub mod thruster;

/// An actuator receives a command vector and mutates the physics state (e.g. applies forces).
pub trait Actuator: Send + Sync {
    fn apply(&self, input: &[f64], state: &mut PhysicsState);
}
