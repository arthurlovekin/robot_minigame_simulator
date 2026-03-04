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

## Separation of Concerns

Each crate has a `CLAUDE.md` with explicit DO / DO NOT lists. The key rule: **each concern lives in exactly one crate**. Never let physics leak into rendering, game logic into the physics engine, or Python bindings into game code.

## Per-Crate CLAUDE.md Files

- `simulator_types/CLAUDE.md`
- `simulator/CLAUDE.md`
- `display/CLAUDE.md`
- `lunar_lander/CLAUDE.md`
- `game_frontend/CLAUDE.md`
- `lunar_lander_gym/CLAUDE.md`
