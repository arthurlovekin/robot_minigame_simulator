# Gym Interface

PyO3/Maturin Python extension that wraps `LunarLanderEnv` with a Gymnasium-compatible API. Intended for ML experiments and RL training pipelines.

## Interface / Public API

Python-side (matches `gym.Env` contract):

```python
env = GymInterface()
obs, info = env.reset(seed=42)
obs, reward, terminated, truncated, info = env.step(action)
```

- `reset(seed: int | None) -> (np.ndarray, dict)`
- `step(action: list[float]) -> (np.ndarray, float, bool, bool, dict)`

## Dependencies

- `lunar_lander` — `LunarLanderEnv`, `Action`, `Observation`

## Design Notes

- **Build**: activate a Python virtualenv, then:
  ```bash
  cd gym_interface
  maturin develop          # editable install for development
  maturin build --release  # build wheel for distribution
  pytest                   # run Python tests
  ```
- `crate-type = ["cdylib"]` is required for Maturin to produce a `.so` / `.pyd`.
- No physics, no rendering, no game logic in this crate — all delegated to `lunar_lander`.
- `terminated` vs `truncated` follows the Gymnasium 0.26+ API (two separate booleans).
