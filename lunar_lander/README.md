# Lunar Lander

Game definition crate. Wraps the `simulator` core with a concrete Lunar Lander environment: sensor and actuator models, observation and action spaces, reward function, and episode termination rules.

## Interface / Public API

- `LunarLanderEnv::new(config: LunarLanderConfig) -> Self`
- `LunarLanderEnv::reset() -> Observation` — resets episode, returns initial observation
- `LunarLanderEnv::step(action: Action) -> StepResult` where `StepResult = (Observation, f64, bool, Info)`
- `Observation` — 32-element `f64` vector
- `Action` — 3-element `f64` vector (throttles, each in [0, 1])

## Dependencies

- `simulator` — physics engine, `Sensor`/`Actuator` traits, `PhysicsState`

## Design Notes

**Observation space (32 dims):**
- IMU: linear acceleration (3), angular velocity (3)
- Engine feedback: 3 throttle echoes
- Contact sensors: landing-leg contact booleans (2)
- Barometric altimeter: 1
- Fuel remaining: 1
- 22-ray LIDAR rangefinder

**Action space:**
- 3 continuous throttles in [0, 1]: main engine + 2 lateral thrusters

**Reward shaping:** penalise fuel use, reward proximity to landing pad, terminal reward for soft landing.

**Episode termination:** crash (velocity threshold), fuel exhaustion, or successful landing.

No rendering, no Python, no UI code in this crate.
