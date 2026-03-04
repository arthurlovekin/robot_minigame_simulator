# Game Frontend

Browser-based Leptos application for human play and debugging of the Lunar Lander game. Handles keyboard input, renders the physics world via `display`, drives the game loop via `lunar_lander`, and provides a configuration UI.

## Interface / Public API

This is a binary crate (WASM app). It has no public library API. Entry point: `src/main.rs`.

## Dependencies

- `lunar_lander` — game stepping (`reset`, `step`)
- `display` — SVG rendering (`render`)

## Design Notes

- **Keyboard → Action**: maps key events to 3-throttle action vectors for `LunarLanderEnv::step`.
- **Game loop**: requestAnimationFrame-driven loop; each frame calls `step` then `render`.
- **Config UI**: Leptos reactive panel for adjusting gravity, wind, and episode parameters at runtime.
- **Build**: requires `trunk` CLI and the `wasm32-unknown-unknown` target.
  ```bash
  rustup target add wasm32-unknown-unknown
  cargo install trunk
  cd game_frontend && trunk serve
  ```
- No physics logic, no sensor models, no Python in this crate.
