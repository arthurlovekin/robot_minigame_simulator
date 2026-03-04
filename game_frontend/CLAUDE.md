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
- `render(state: &PhysicsState) -> String` from `display`

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
