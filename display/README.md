# Display

Stateless SVG renderer — a pure function from a `PhysicsState` snapshot to SVG markup. Targets WebAssembly (WASM) for use in the browser via the `game_frontend` Leptos app.

## Interface / Public API

- `render(state: &PhysicsState) -> String` — converts a physics snapshot to an SVG string
- (Planned) Leptos reactive component variant: `<WorldView state=state_signal />`

## Dependencies

- `simulator` — for the `PhysicsState` type

## Design Notes

- **Pure function**: `render` takes a snapshot and returns SVG. No internal state, no side effects.
- **No simulation stepping**: the caller advances physics; `display` only visualises.
- **No game logic**: reward, episode state, and input handling are not display concerns.
- **No input handling**: keyboard/mouse events belong in `game_frontend`.
- **WASM target**: build with `wasm-pack` or `trunk`. Always verify with `cargo check --target wasm32-unknown-unknown`.
- Coordinate system: physics uses MKS-Radians; the renderer applies a scale factor to map metres → SVG user units.
