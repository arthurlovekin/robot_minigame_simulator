# CLAUDE.md — simulator_types

## Purpose

Leaf crate containing shared plain-data types used across the workspace.
Zero dependencies. Safe to pull into WASM bundles without dragging in the physics engine.

## Public Types

- `Vec2` — 2-D vector (metres)
- `BodyId` — newtype over `usize`; stable handle for a rigid body
- `ColliderShape` — geometry tag (`Circle`, `Box`, `Capsule`)
- `BodyState` — position, velocity, angle, angular_velocity, shape
- `Contact` — body pair + impulse magnitude
- `PhysicsState` — `Vec<BodyState>` + `Vec<Contact>`; world snapshot

## Separation of Concerns — DO and DO NOT

**DO:**
- Define plain data structs and enums with `#[derive(Debug, Clone, PartialEq)]`
- Add simple accessor methods if they are pure data transformations

**DO NOT:**
- Add any physics logic (integration, collision) — that belongs in `simulator`
- Add rendering logic — that belongs in `display`
- Add game logic — that belongs in `lunar_lander`
- Add any dependencies (keep this crate at zero deps)

## Testing Requirements

- Every public function must have at least one unit test (`#[cfg(test)]` in the same file).
- Integration tests that cross crate boundaries go in `tests/` at the crate root.
- Performance-sensitive functions must have a criterion benchmark in `benches/`.
- Tests must be deterministic — no wall-clock time, no uncontrolled randomness.
- Do not use `#[allow(dead_code)]` or `#[allow(unused)]` to silence CI — fix the root cause.
