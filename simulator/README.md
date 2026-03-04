# Simulator

Core physics engine — the single source of ground-truth simulation for all games in this workspace.

The simulator is tuned for Meter-Kilogram-Second-Radians units. Keep the size of moving objects larger than 1 cm. You'll need to use some scaling system when you render your environment and actors.

## Interface / Public API

- `PhysicsState` — snapshot of all rigid-body positions, velocities, and orientations at a given timestep
- `Collider` — shape + mass properties attached to a body
- `Sensor` trait — reads from `PhysicsState` and produces an observation vector
- `Actuator` trait — applies forces/torques to bodies in `PhysicsState`
- `PhysicsEngine::step(dt: f64)` — advances simulation by one timestep; returns the new `PhysicsState`

## Dependencies

No other workspace crates. `simulator` is a leaf node in the dependency graph.

## Design Notes

- **Units**: MKS-Radians throughout. Minimum object dimension: 1 cm (0.01 m).
- **No rendering**: SVG/display concerns belong in the `display` crate.
- **No game logic**: reward functions, episode termination, and wind models live in game crates (e.g., `lunar_lander`).
- **No Python bindings**: PyO3 glue lives in `gym_interface`.
- Time integration: semi-implicit Euler by default; timestep is caller-controlled.
- Collision detection is discrete; continuous collision detection (CCD) is a future concern.
