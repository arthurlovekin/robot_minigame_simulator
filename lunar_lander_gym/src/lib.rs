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
        LunarLanderGym { env: LunarLanderEnv::new(config) }
    }

    /// Reset the environment. Returns `(observation, info)`.
    pub fn reset(&mut self) -> (Observation, ()) {
        (self.env.reset(), ())
    }

    /// Step the environment. Returns `(observation, reward, terminated, truncated, info)`.
    pub fn step(&mut self, action: Action) -> (Observation, f32, bool, bool, ()) {
        let (obs, terminated, _state) = self.env.step(action);
        let reward = compute_reward(&obs, &action, terminated);
        (obs, reward, terminated, false, ())
    }
}

/// Reward function for the Lunar Lander task.
///
/// Stub: returns 0.0. Full implementation will consider altitude,
/// velocity, fuel usage, and landing accuracy.
fn compute_reward(_obs: &Observation, _action: &Action, _terminated: bool) -> f32 {
    0.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reset_and_step_do_not_panic() {
        let mut gym = LunarLanderGym::new(LunarLanderConfig::default());
        let (obs, ()) = gym.reset();
        assert_eq!(obs.len(), 8);
        let (obs2, reward, _terminated, _truncated, ()) = gym.step([0.0, 0.0, 0.0]);
        assert_eq!(obs2.len(), 8);
        assert_eq!(reward, 0.0);
    }
}
