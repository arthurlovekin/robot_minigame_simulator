use lunar_lander::{Action, LunarLanderConfig, LunarLanderEnv, Observation};

/// Gymnasium-compatible wrapper around `LunarLanderEnv`.
///
/// This is where the RL reward function lives — not in `lunar_lander`.
pub struct LunarLanderGym {
    env: LunarLanderEnv,
}

impl LunarLanderGym {
    #[must_use]
    pub fn new(config: LunarLanderConfig) -> Self {
        LunarLanderGym {
            env: LunarLanderEnv::new(config),
        }
    }

    /// Reset the environment. Returns `(observation, info)`.
    pub fn reset(&mut self) -> (Observation, ()) {
        (self.env.reset(), ())
    }

    /// Step the environment. Returns `(observation, reward, terminated, truncated, info)`.
    pub fn step(&mut self, action: Action) -> (Observation, f64, bool, bool, ()) {
        let (obs, terminated, _state) = self.env.step(action);
        let reward = compute_reward(&obs, &action, terminated);
        (obs, reward, terminated, false, ())
    }
}

/// Reward function for the Lunar Lander task.
///
/// Stub: returns 0.0. Full implementation will consider altitude,
/// velocity, fuel usage, and landing accuracy.
fn compute_reward(_obs: &Observation, _action: &Action, _terminated: bool) -> f64 {
    0.0
}
