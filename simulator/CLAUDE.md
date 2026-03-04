# CLAUDE.md — simulator

## Purpose

Core rigid-body physics engine. The only source of ground-truth simulation in the workspace.
Also owns all concrete sensor and actuator implementations — these are physics primitives,
reusable across games.

## Build & Test (standalone)

```bash
cd simulator
cargo build
cargo test
```

## Public Interface

- `PhysicsEngine::new() -> Self`
- `PhysicsEngine::step(dt: f64) -> PhysicsState`
- `sensors::Sensor` trait — `fn observe(&self, state: &PhysicsState) -> Vec<f64>`
- `sensors::Imu`, `sensors::Lidar`, `sensors::Barometer`, `sensors::ContactSensor`, `sensors::EngineFeedback`
- `actuators::Actuator` trait — `fn apply(&self, input: &[f64], state: &mut PhysicsState)`
- `actuators::Thruster`
- Re-exports from `simulator_types`: `BodyId`, `BodyState`, `ColliderShape`, `Contact`, `PhysicsState`, `Vec2`

## Dependencies

- `simulator_types` (path dep) — plain data types only

## Internal Layout

```
simulator/src/
├── lib.rs              ← PhysicsEngine, step(), re-exports
├── sensors/
│   ├── mod.rs          ← Sensor trait
│   ├── imu.rs          ← params: noise_stddev, bias_drift
│   ├── lidar.rs        ← params: num_rays, max_range, noise_stddev
│   ├── barometer.rs    ← params: noise_stddev
│   ├── contact.rs      ← boolean ground contact
│   └── engine_feedback.rs ← params: noise_stddev
└── actuators/
    ├── mod.rs          ← Actuator trait
    └── thruster.rs     ← params: max_force, noise_stddev, spin_up_time_constant
```

## Separation of Concerns — DO and DO NOT

**DO:**
- Rigid-body dynamics, time integration, collision detection
- Define `Sensor` and `Actuator` traits with concrete implementations here
- Re-export `simulator_types` types for callers' convenience
- Accept configurable parameters (noise, ranges) at construction time

**DO NOT:**
- Render anything (SVG, canvas, pixels) — that belongs in `display`
- Encode game-specific reward logic or episode rules — those belong in game crates
- Include Python bindings (`#[pyclass]`, `pyo3`) — those belong in `lunar_lander_gym`
- Model wind, gravity presets, or other game-level environment parameters — callers configure these
