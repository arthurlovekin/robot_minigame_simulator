Robot Minigame simulator

A simple, lightweight, and fast simulator for experimenting with robotics and robot-learning. 

The simulation consists of the following modules:

- Simulator: the simulator core is responsible for propagating the physics of the 2-dimensional environment (collisions, sensors, actuators, and other moving objects) forward in time. It is written in Rust, and is intended to be very lightweight, allowing high iteration speed, large experiments, full interpretability, and a lot of control over the simulated variables. It allows easy switching between different sensor models, actuator models, dynamics models, and time integration (eg. explicit-euler vs symplectic euler). The state of the game is a simple vector containing the position, velocity, acceleration of the robot, along with its rigid-body collision mesh. For the first iteration, the robot will not have joints, and the simulator will be single-threaded. However, in the future legged robots may be supported, and Rayon will be used to add parallelism.
- Display: The display is responsible for visualizing the current state of the game. It is web-native, written in Rust using the Leptos web framework, and represents the visual appearance of the robot as an svg for fast rendering.
- Game: A specific instantiation of state, action space, observation space, rewards/point-system, and rules that define a game that can be played by humans or learning agents. The game uses the Core and Renderer, and selects the physics model, sensor model, and actuator models to be used.
- Game Front-end: For fun, testing, and visualizing experimental results, the user is able to control the robot using keyboard commands in their web browser. There are buttons to start, stop, and configure the different physics/sensor/actuator models in the game.
- Robot-learning Front-end: The simulator uses PyO3 and Maturin to provide a standard python Gymnasium interface (see below). This will be the interface used to run robot-learning experiments. A heuristics-based “teacher” model and a mechanism for storing data collected from a particular set of physics parameters will be provided.

```python
class MyRobotEnv(gym.Env):
def init(self):
self.observation_space = gym.spaces.Box(...)
self.action_space = gym.spaces.Box(...)
def reset(self, seed=None):
    # Return (observation, info)
    pass

def step(self, action):
    # Return (observation, reward, terminated, truncated, info)
    pass
```

# The Game: Lunar Lander

A rocket needs to land softly on the landing pad without expending all of its fuel. This is a classic rocket trajectory optimization problem: the optimal solution is to leave your engines off until exactly the right moment, then turn them on full. However, given environmental disturbances like wind and sensor uncertainty, the agent will have to find a solution that allows a little room for error. 

This game is very similar to the existing Gymnasium Box2D version of the game, except this version takes a more “robotics” perspective:

- Action Space / Actuators: Instead of simplified and discrete action model, the actuation space is continuous, and actuators are independent, and subject to noise and time delay.
    - There are three thrusters - left, right, and bottom - which can each fire with a throttle value between 0 (off) and 1 (full).
- Observation Space /Sensors: the agent learning the game does not see the entire screen or a simplified privileged state, but rather the sensor inputs provided to it. These sensor inputs are subject to noise and time delay, and don’t necessarily show the full state of the environment
    - The observed state is an 32-dimensional vector:
        - 3 values (linear_acceleration_x, linear_acceleration_y, angular_velocity) from an Inertial Measurement Unit
        - 3 engine feedback sensors (left_throttle, bottom_throttle, right_throttle) (these may not match actual throttle values commanded). Each gives a range [0,1]
        - 2 Contact sensors on leg: boolean indicating whether the leg is in contact with the ground.
        - 1 measurement from a barometer that measures the distance from the bottom of the screen [0px to 1000px]
        - 1 fuel-sensor measurement that tells you how much fuel is remaining. Range: [0,1]
        - 22 depth measurements from a 2D lidar that casts rays from the bottom of the lander. Sensor model (may be changed by user) range: [5px,50px], 0 indicates too close or too far or error
- Many variables are configurable in order to study the sim-to-real gap: What happens when the robot is trained on data from a (simulated) environment with one set of physics parameters, but then gets tested in the “real world” - an environment with different physics parameters?.
    - Sensor placement
    - Noise in sensors and actuators
    - Timing delays

## Environmental Variables (same as Box2D Lunar Lander)

- `gravity` dictates the gravitational constant, this is bounded to be within 0 and -12. Default is -10.0
- `enable_wind` determines if there will be wind effects applied to the lander. The wind is generated using
the function `tanh(sin(2 k (t+C)) + sin(pi k (t+C)))` where `k` is set to 0.01 and `C` is sampled randomly between -9999 and 9999.
- `wind_power` dictates the maximum magnitude of linear wind applied to the craft. The recommended value for
`wind_power` is between 0.0 and 20.0.
- `turbulence_power` dictates the maximum magnitude of rotational wind applied to the craft.
The recommended value for `turbulence_power` is between 0.0 and 2.0.

## Starting State

The lander starts at the top center of the viewport with a random initial force and torque applied to its center of mass, and a full fuel tank.

## Episode Termination

The episode finishes if:

1. the lander crashes: the lander body or lander feet contact the ground with force above their respective max force thresholds.
2. the lander gets outside of the viewport above or to the sides (`x` coordinate is greater than 1);

## Rewards (same as the [Lunar Lander Gym environment](https://gymnasium.farama.org/environments/box2d/lunar_lander/))

After every step a reward is granted. The total reward of an episode is the
sum of the rewards for all the steps within that episode.

For each step, the reward:

- is increased/decreased the closer/further the lander is to the landing pad.
- is increased/decreased the slower/faster the lander is moving.
- is decreased the more the lander is tilted (angle not horizontal).
- is increased by 10 points for each leg that is in contact with the ground.
- is decreased by 0.03 points each frame a side engine is firing.
- is decreased by 0.3 points each frame the main engine is firing.

The episode receive an additional reward of -100 or +100 points for crashing or landing safely respectively.

An episode is considered a solution if it scores at least 200 points.

## Determinism

This simulator is fully deterministic (subject to a particular random seed), but consecutive episodes are statistically independent.

---

## Future

In the future, more games will hopefully be supported! The simulator should be built with some sense of generalizability in mind (but nothing that is irrelevant to the current game).  

Sources of inspiration

1. [Gymnasium Box2D games](https://gymnasium.farama.org/environments/box2d/), specifically [Box2D Lunar Lander](https://gymnasium.farama.org/environments/box2d/lunar_lander/). [Box2D](https://box2d.org/documentation/index.html) is the core simulator.
2. [Mujoco Robotics simulator](https://mujoco.readthedocs.io/en/stable/computation/index.html#)