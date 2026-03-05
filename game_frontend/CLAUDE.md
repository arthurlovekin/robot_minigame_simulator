# CLAUDE.md — game_frontend

## Purpose

Leptos WASM binary. Human-playable browser app: keyboard input → game step → SVG render → display.

## Build & Test (standalone)

```bash
# Prerequisites:
rustup target add wasm32-unknown-unknown
cargo install trunk

cd game_frontend
trunk serve          # development server with hot reload
trunk build --release  # production WASM bundle
```

## Public Interface

Binary crate — no public library API. Entry point: `src/main.rs`.

## Inputs from other crates

- `LunarLanderEnv`, `Action`, `Observation` from `lunar_lander`
- `render(state: &PhysicsState, config: &RenderConfig) -> String` from `display`
- `RenderConfig` from `display` — viewport size, scale, camera centre

## Separation of Concerns — DO and DO NOT

**DO:**
- Map keyboard/gamepad events to `Action` vectors for `LunarLanderEnv::step`
- Drive the requestAnimationFrame game loop
- Call `display::render` for SVG output; inject SVG into the DOM
- Provide a Leptos reactive config panel (gravity, wind, episode params)

**DO NOT:**
- Implement physics logic — that belongs in `simulator`
- Implement sensor/actuator models or reward logic — those belong in `lunar_lander`
- Include Python bindings — those belong in `gym_interface`
- Duplicate rendering logic from `display`

## Testing Requirements

- Every public function must have at least one unit test (`#[cfg(test)]` in the same file).
- Integration tests that cross crate boundaries go in `tests/` at the crate root.
- Performance-sensitive functions must have a criterion benchmark in `benches/`.
- Tests must be deterministic — no wall-clock time, no uncontrolled randomness.
- Do not use `#[allow(dead_code)]` or `#[allow(unused)]` to silence CI — fix the root cause.
