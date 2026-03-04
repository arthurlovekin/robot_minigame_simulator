# CLAUDE.md — lunar_lander

## Purpose

Lunar Lander game environment. Configures and attaches sensors/actuators (from `simulator`),
sets up episode initial conditions, and detects termination. **Does not compute reward** —
that lives in `lunar_lander_gym`.

## Build & Test (standalone)

```bash
cd lunar_lander
cargo build
cargo test
```

## Public Interface

- `LunarLanderEnv::new(config: LunarLanderConfig) -> Self`
- `LunarLanderEnv::reset() -> Observation`
- `LunarLanderEnv::step(action: Action) -> (Observation, bool, PhysicsState)`
  - Returns `(obs, terminated, state)`. Reward is the caller's responsibility.
- `Observation` — `[f64; 8]`
- `Action` — `[f64; 3]` throttles in [0, 1]
- `TerminationReason` — `Crash | OutOfBounds | FuelExhausted | SoftLanding`
- `LunarLanderConfig` — gravity, wind, turbulence, sensor params, actuator params

## Dependencies

- `simulator_types` (path dep) — `PhysicsState` and related types
- `simulator` (path dep) — `PhysicsEngine`, `Sensor`/`Actuator` traits, concrete sensors/actuators

## Internal Layout

```
lunar_lander/src/
├── lib.rs              ← LunarLanderEnv, re-exports
├── config.rs           ← LunarLanderConfig
├── spaces.rs           ← Observation, Action type aliases
└── termination.rs      ← TerminationReason, check_termination()
```

## Separation of Concerns — DO and DO NOT

**DO:**
- Configure and attach `simulator` sensors/actuators (Lidar, IMU, Thruster, etc.)
- Define observation/action space types and episode configuration
- Detect and report termination conditions (crash, out-of-bounds, fuel, soft landing)

**DO NOT:**
- Implement sensor or actuator physics — those belong in `simulator`
- Compute RL reward — that belongs in `lunar_lander_gym`
- Render anything — that belongs in `display`
- Include Python bindings (`#[pyclass]`) — those belong in `lunar_lander_gym`
- Include UI or input-handling code — that belongs in `game_frontend`

## Testing Requirements

- Every public function must have at least one unit test (`#[cfg(test)]` in the same file).
- Integration tests that cross crate boundaries go in `tests/` at the crate root.
- Performance-sensitive functions must have a criterion benchmark in `benches/`.
- Tests must be deterministic — no wall-clock time, no uncontrolled randomness.
- Do not use `#[allow(dead_code)]` or `#[allow(unused)]` to silence CI — fix the root cause.
