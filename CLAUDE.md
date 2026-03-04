# CLAUDE.md — Robot Minigame Simulator (workspace root)

## Workspace Overview

Rust workspace implementing a physics-based robot minigame simulator with a browser frontend and Python ML interface.

## Module Map

| Crate | Path | Type | Role |
|---|---|---|---|
| `simulator_types` | `simulator_types/` | `lib` | Shared plain-data types: `BodyId`, `BodyState`, `PhysicsState`, `ColliderShape`, `Contact`, `Vec2` — zero deps |
| `simulator` | `simulator/` | `lib` | Physics engine, time integration, collision detection; all sensor + actuator implementations with configurable params |
| `display` | `display/` | `lib` (WASM) | SVG renderer: `render(state: &PhysicsState) -> String` — deps: `simulator_types` only |
| `lunar_lander` | `lunar_lander/` | `lib` | Configures sensors/actuators, episode setup, termination detection — no reward |
| `game_frontend` | `game_frontend/` | `bin` (WASM) | Leptos browser app: keyboard → action, game loop, score display — built with `trunk` |
| `lunar_lander_gym` | `lunar_lander_gym/` | `cdylib` | PyO3/Maturin Gymnasium wrapper: reward function + `reset`/`step` API |

## Dependency Graph

```
game_frontend    → lunar_lander    → simulator      → simulator_types
game_frontend    → display         → simulator_types
lunar_lander_gym → lunar_lander    → simulator      → simulator_types
```

`simulator_types` is the leaf (zero deps). `display` never pulls in the physics engine.

## Key Architectural Decisions

- **Plain `PhysicsState` struct** — not a trait. `simulator` produces it; consumers read it directly.
- **Sensors and actuators live in `simulator`** — concrete implementations (Lidar, IMU, Thruster, etc.) are physics primitives reusable across games.
- **Reward in `lunar_lander_gym` only** — `lunar_lander` handles episode setup and termination; the RL reward function lives exclusively in the gym wrapper.

## Build Commands

```bash
# Check all crates
cargo check --workspace

# Test all crates
cargo test --workspace

# WASM crates
cd display       && cargo check --target wasm32-unknown-unknown
cd game_frontend && trunk serve   # requires: cargo install trunk

# Verify display does NOT pull in simulator (rapier/nalgebra)
cargo tree -p display

# Python binding
cd lunar_lander_gym && maturin develop   # requires: active Python venv with maturin
```

## Units & Conventions

- **Units**: MKS-Radians throughout (metres, kilograms, seconds, radians)
- **Minimum object size**: 1 cm (0.01 m) — smaller objects cause physics instability
- **Rendering scale**: physics coordinates are in metres; renderers apply a scale factor to SVG user units

## Design Principles

Three principles govern all code in this workspace. Every agent and contributor must apply them.

### 1. Separation of Concerns
Each concern lives in exactly one crate. Cross-crate dependencies must match the documented
graph above. Introducing a new cross-crate dependency requires updating this graph.
Within a crate, each module has one job. New modules must have a DO/DO NOT list.
- Automated gate: `cargo modules dependencies --acyclic` — any cycle = CI failure.
- Metric: `cargo anatomy` D score (distance from main sequence, target ≈ 0 per crate).

### 2. Data is Key
Prefer plain data structs with `pub` fields over getter/setter methods.
Prefer returning data to using callbacks or closures for data transfer.
Avoid `Box<dyn Trait>` when a concrete type or enum suffices.
Never hide the shape of data behind abstraction layers.
- Enforced by: `#[deny(clippy::wildcard_imports)]`.

### 3. Keep It Simple, Stupid (KISS)
Do not introduce a generic type parameter unless used in ≥2 call sites.
Do not introduce a trait unless there are ≥3 concrete implementors (or it is the explicit
public API surface, e.g. `Sensor` / `Actuator`).
Do not optimize a hot path until a criterion benchmark proves it is hot.
Do not add fallbacks or configuration for scenarios that cannot happen yet.
- Enforced by: `clippy::cognitive_complexity`, `clippy::too_many_arguments`.

## Quality Gates

All commits to `main` pass these automated checks (CI: `.github/workflows/ci.yml`):

| Check | Command | Gate |
|---|---|---|
| Compile | `cargo check --workspace` | fail = block |
| Tests | `cargo test --workspace` | fail = block |
| Clippy | `cargo clippy --workspace --all-targets -- -D warnings` | warn = block |
| Unused deps | `cargo machete` | found = block |
| Module cycles | `cargo modules dependencies --acyclic -p <crate>` | cycle = block |
| Security/License | `cargo deny check` | violation = block |
| Benchmarks | `cargo bench --workspace` | artifact, non-blocking |
| Anatomy report | `cargo anatomy` | summary, non-blocking |

### Running checks locally
```bash
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo machete
cargo deny check
cargo modules dependencies --acyclic -p simulator
cargo anatomy            # view crate-level coupling metrics
cargo bench -p simulator # save benchmark baseline
```

### Installing analysis tools
```bash
cargo install cargo-machete
cargo install cargo-deny
cargo install cargo-modules
cargo install cargo-anatomy
```

## Metric Targets (cargo-anatomy)

Run `cargo anatomy` to check coupling health. A D score >0.3 is a smell.

| Crate | Ca | Ce | A (abstractness) | I (instability) | Expected D |
|---|---|---|---|---|---|
| `simulator_types` | high | 0 | ≈0 (all plain data) | ≈0 (stable) | ≈0 |
| `simulator` | medium | 1 (→types) | >0 (Sensor/Actuator traits) | low-medium | ≈0 |
| `display` | medium | 1 (→types) | ≈0 (pure fn) | low-medium | ≈0 |
| `lunar_lander` | 1 (→gym) | 2 (→sim, types) | ≈0 (config + env struct) | medium | ≈0 |
| `lunar_lander_gym` | 0 (nothing deps on it) | 1 (→lander) | ≈0 | ≈1 (unstable) | ≈0 |
| `game_frontend` | 0 (binary) | 2 (→lander, display) | ≈0 | ≈1 (unstable) | ≈0 |

## Separation of Concerns

Each crate has a `CLAUDE.md` with explicit DO / DO NOT lists. The key rule: **each concern lives in exactly one crate**. Never let physics leak into rendering, game logic into the physics engine, or Python bindings into game code.

## Per-Crate CLAUDE.md Files

- `simulator_types/CLAUDE.md`
- `simulator/CLAUDE.md`
- `display/CLAUDE.md`
- `lunar_lander/CLAUDE.md`
- `game_frontend/CLAUDE.md`
- `lunar_lander_gym/CLAUDE.md`
