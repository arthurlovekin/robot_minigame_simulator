use simulator_types::PhysicsState;
use super::Actuator;

/// Single thruster: converts a throttle command in [0, 1] to a force impulse.
pub struct Thruster {
    /// Maximum thrust force (Newtons).
    pub max_force: f64,
    /// Standard deviation of multiplicative force noise (fraction of max_force).
    pub noise_stddev: f64,
    /// First-order lag time constant for spin-up/spin-down (seconds).
    pub spin_up_time_constant: f64,
}

impl Actuator for Thruster {
    fn apply(&self, _input: &[f64], _state: &mut PhysicsState) {
        // Stub: no-op.
    }
}
