use simulator_types::Vec2;

use super::Actuator;

/// Single thruster with first-order spin-up lag.
pub struct Thruster {
    pub max_force: f32,
    pub noise_stddev: f32,
    pub spin_up_time_constant: f32,
    /// Thrust direction in body frame (normalised). Default: `{0, 1}` (straight up).
    pub direction: Vec2,
    /// Current normalised thrust level [0, 1].
    pub current_thrust: f32,
}

impl Thruster {
    #[must_use]
    pub fn new(max_force: f32, spin_up_time_constant: f32) -> Self {
        Thruster {
            max_force,
            noise_stddev: 0.0,
            spin_up_time_constant,
            direction: Vec2 { x: 0.0, y: 1.0 },
            current_thrust: 0.0,
        }
    }

    /// Normalised current thrust level [0, 1].
    #[must_use]
    pub fn current_normalized(&self) -> f32 {
        self.current_thrust
    }
}

impl Actuator for Thruster {
    /// Apply throttle command `input[0]` in [0, 1] with first-order lag.
    /// Returns body-frame force (N) and torque (N·m, always 0 for a centred thruster).
    fn apply(&mut self, input: &[f32], dt: f32) -> (Vec2, f32) {
        let target = input.first().copied().unwrap_or(0.0).clamp(0.0, 1.0);
        let tau = self.spin_up_time_constant.max(f32::EPSILON);
        // First-order lag: T += (target - T) * dt / τ, clamped to avoid overshoot
        self.current_thrust += (target - self.current_thrust) * (dt / tau).min(1.0);
        let force = self.direction * (self.current_thrust * self.max_force);
        (force, 0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spinup_reaches_63_percent_after_one_time_constant() {
        let tau = 1.0_f32;
        let mut thruster = Thruster::new(100.0, tau);
        // Simulate with many small steps to approximate continuous first-order lag
        let steps = 10_000u32;
        let dt = tau / steps as f32;
        for _ in 0..steps {
            thruster.apply(&[1.0], dt);
        }
        // After 1 time constant, first-order lag reaches 1 - 1/e ≈ 0.6321
        let expected = 1.0 - 1.0 / std::f32::consts::E;
        assert!(
            (thruster.current_normalized() - expected).abs() < 0.01,
            "got {:.4}, expected ~{:.4}",
            thruster.current_normalized(),
            expected
        );
    }

    #[test]
    fn thruster_clamps_input_above_one() {
        let mut thruster = Thruster::new(100.0, 0.0);
        thruster.apply(&[2.0], 1.0);
        assert!(thruster.current_thrust <= 1.0);
    }
}
