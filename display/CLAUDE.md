# CLAUDE.md — display

## Purpose

Stateless SVG renderer. Pure function: `render(state: &PhysicsState) -> String`.
Targets WebAssembly. Depends **only** on `simulator_types` — never on `simulator` —
so the rapier/nalgebra physics engine does not enter the WASM bundle.

## Build & Test (standalone)

```bash
cd display
cargo check --target wasm32-unknown-unknown
# Full WASM build (built via game_frontend with trunk):
cd ../game_frontend && trunk serve
```

## Public Interface

- `RenderConfig` — plain-data struct: `width_px`, `height_px`, `pixels_per_metre`, `camera_centre`
- `render(state: &PhysicsState, config: &RenderConfig) -> String`

## Dependencies

- `simulator_types` (path dep) — plain data types only; **not** `simulator`

## Separation of Concerns — DO and DO NOT

**DO:**
- Convert a `PhysicsState` snapshot into SVG markup
- Apply a scale transform from MKS metres to SVG user units

**DO NOT:**
- Depend on `simulator` — use `simulator_types` directly
- Call `PhysicsEngine::step()` or advance the simulation — that's the caller's job
- Encode game logic (reward, done flag, episode rules) — those live in `lunar_lander`
- Handle keyboard or mouse input — that belongs in `game_frontend`
- Include Python bindings — those belong in `lunar_lander_gym`

## Testing Requirements

- Every public function must have at least one unit test (`#[cfg(test)]` in the same file).
- Integration tests that cross crate boundaries go in `tests/` at the crate root.
- Performance-sensitive functions must have a criterion benchmark in `benches/`.
- Tests must be deterministic — no wall-clock time, no uncontrolled randomness.
- Do not use `#[allow(dead_code)]` or `#[allow(unused)]` to silence CI — fix the root cause.
