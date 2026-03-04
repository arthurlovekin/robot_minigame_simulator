pub mod config;
pub mod spaces;
pub mod termination;

pub use config::LunarLanderConfig;
pub use spaces::{Action, Observation};
pub use termination::TerminationReason;

use simulator::PhysicsEngine;
use simulator_types::PhysicsState;

/// Lunar Lander environment: episode setup and termination detection.
///
/// Reward computation is NOT here — it lives in `lunar_lander_gym`.
pub struct LunarLanderEnv {
    config: LunarLanderConfig,
    engine: PhysicsEngine,
}

impl LunarLanderEnv {
    pub fn new(config: LunarLanderConfig) -> Self {
        LunarLanderEnv {
            config,
            engine: PhysicsEngine::new(),
        }
    }

    /// Reset the environment to an initial state and return the first observation.
    pub fn reset(&mut self) -> Observation {
        // Stub: return zero observation.
        [0.0; 8]
    }

    /// Advance by one timestep.
    ///
    /// Returns `(observation, terminated, physics_state)`.
    /// Reward is computed by the caller (`lunar_lander_gym`).
    pub fn step(&mut self, _action: Action) -> (Observation, bool, PhysicsState) {
        let state = self.engine.step(1.0 / 60.0);
        let terminated = termination::check_termination(&state).is_some();
        ([0.0; 8], terminated, state)

    }

    pub fn config(&self) -> &LunarLanderConfig {
        &self.config
    }
}
