# CLAUDE.md — lunar_lander_gym

## Purpose

PyO3/Maturin cdylib. Thin Gymnasium-compatible Python wrapper around `LunarLanderEnv`.
**This is the only place where the RL reward function is computed.**

## Build & Test (standalone)

```bash
# Prerequisites: Python virtualenv with maturin and pytest installed
source .venv/bin/activate   # or equivalent

cd lunar_lander_gym
maturin develop              # editable install
pytest                       # run Python-side tests
```

## Public Interface

`#[pyclass] LunarLanderGym` with:
- `reset(seed: Option<u64>) -> (Vec<f64>, PyDict)`
- `step(action: Vec<f64>) -> (Vec<f64>, f64, bool, bool, PyDict)`

Matches the Gymnasium 0.26+ `gym.Env` contract (`terminated`, `truncated` are separate booleans).

## Dependencies

- `lunar_lander` (path dep) — `LunarLanderEnv`, `Action`, `Observation`, `LunarLanderConfig`

## Separation of Concerns — DO and DO NOT

**DO:**
- Wrap `LunarLanderEnv` with `#[pyclass]` and `#[pymethods]`
- Compute the RL reward function here (based on obs, action, termination)
- Convert between Rust types and Python/numpy-compatible types
- Expose `reset` and `step` matching the Gymnasium contract

**DO NOT:**
- Implement physics — that belongs in `simulator`
- Implement sensor/actuator models or episode rules — those belong in `lunar_lander`
- Include rendering code — that belongs in `display`
- Add UI code — that belongs in `game_frontend`

## Testing Requirements

- Every public function must have at least one unit test (`#[cfg(test)]` in the same file).
- Integration tests that cross crate boundaries go in `tests/` at the crate root.
- Performance-sensitive functions must have a criterion benchmark in `benches/`.
- Tests must be deterministic — no wall-clock time, no uncontrolled randomness.
- Do not use `#[allow(dead_code)]` or `#[allow(unused)]` to silence CI — fix the root cause.
