pub mod config;
pub mod spaces;
pub mod termination;

pub use config::LunarLanderConfig;
pub use spaces::{Action, Observation};
pub use termination::TerminationReason;

use simulator::actuators::thruster::Thruster;
use simulator::actuators::Actuator;
use simulator::sensors::barometer::Barometer;
use simulator::sensors::contact::ContactSensor;
use simulator::sensors::imu::Imu;
use simulator::sensors::Sensor;
use simulator::{BodyDef, ColliderDef, ColliderShape, ExternalForce, PhysicsEngine, Vec2};
use simulator_types::{BodyId, PhysicsState};

const DT: f32 = 1.0 / 60.0;
const GROUND_ID: BodyId = BodyId(0);
const LANDER_ID: BodyId = BodyId(1);

// Thruster body-frame attachment offsets
const OFFSET_LEFT: Vec2 = Vec2 { x: -0.55, y: -0.5 };
const OFFSET_RIGHT: Vec2 = Vec2 { x: 0.55, y: -0.5 };

/// Lunar Lander environment: episode setup and termination detection.
///
/// Reward computation is NOT here — it lives in `lunar_lander_gym`.
pub struct LunarLanderEnv {
    config: LunarLanderConfig,
    engine: PhysicsEngine,
    last_state: PhysicsState,
    thruster_main: Thruster,
    thruster_left: Thruster,
    thruster_right: Thruster,
    imu: Imu,
    barometer: Barometer,
    contact_sensor: ContactSensor,
}

impl LunarLanderEnv {
    #[must_use]
    pub fn new(config: LunarLanderConfig) -> Self {
        let gravity = Vec2 { x: 0.0, y: -config.gravity };
        let engine = PhysicsEngine::new(gravity);
        let thruster_main = Thruster::new(
            config.thruster_max_force,
            config.thruster_spin_up_time_constant,
        );
        let thruster_left = Thruster::new(
            config.thruster_max_force * 0.5,
            config.thruster_spin_up_time_constant,
        );
        let thruster_right = Thruster::new(
            config.thruster_max_force * 0.5,
            config.thruster_spin_up_time_constant,
        );
        let imu = Imu { body_id: LANDER_ID, noise_stddev: config.imu_noise_stddev, bias_drift: config.imu_bias_drift };
        let barometer = Barometer { body_id: LANDER_ID, noise_stddev: config.barometer_noise_stddev };
        let contact_sensor = ContactSensor { body_id: LANDER_ID };

        LunarLanderEnv {
            config,
            engine,
            last_state: PhysicsState::default(),
            thruster_main,
            thruster_left,
            thruster_right,
            imu,
            barometer,
            contact_sensor,
        }
    }

    /// Reset the environment to its initial state and return the first observation.
    pub fn reset(&mut self) -> Observation {
        let gravity = Vec2 { x: 0.0, y: -self.config.gravity };
        self.engine = PhysicsEngine::new(gravity);

        // Ground: wide static box
        let ground = self.engine.add_body(BodyDef {
            position: Vec2 { x: 0.0, y: -0.5 },
            velocity: Vec2 { x: 0.0, y: 0.0 },
            angle: 0.0,
            angular_velocity: 0.0,
            density: 0.0,
            colliders: vec![ColliderDef {
                shape: ColliderShape::Box { half_width: 20.0, half_height: 0.5 },
                offset: Vec2 { x: 0.0, y: 0.0 },
                angle: 0.0,
                restitution: 0.3,
            }],
            is_static: true,
        });
        debug_assert_eq!(ground, GROUND_ID);

        // Lander: hull + left leg + right leg
        let lander = self.engine.add_body(BodyDef {
            position: Vec2 { x: 0.0, y: 10.0 },
            velocity: Vec2 { x: 0.0, y: 0.0 },
            angle: 0.0,
            angular_velocity: 0.0,
            density: 20.0,
            colliders: vec![
                // Hull: 1.0 m wide × 0.6 m tall
                ColliderDef {
                    shape: ColliderShape::Box { half_width: 0.5, half_height: 0.3 },
                    offset: Vec2 { x: 0.0, y: 0.0 },
                    angle: 0.0,
                    restitution: 0.3,
                },
                // Left leg: 0.15 m wide × 0.4 m tall
                ColliderDef {
                    shape: ColliderShape::Box { half_width: 0.075, half_height: 0.2 },
                    offset: OFFSET_LEFT,
                    angle: 0.0,
                    restitution: 0.3,
                },
                // Right leg: 0.15 m wide × 0.4 m tall
                ColliderDef {
                    shape: ColliderShape::Box { half_width: 0.075, half_height: 0.2 },
                    offset: OFFSET_RIGHT,
                    angle: 0.0,
                    restitution: 0.3,
                },
            ],
            is_static: false,
        });
        debug_assert_eq!(lander, LANDER_ID);

        // Reset thrusters
        self.thruster_main.current_thrust = 0.0;
        self.thruster_left.current_thrust = 0.0;
        self.thruster_right.current_thrust = 0.0;

        // Step once with dt=0 to populate PhysicsState (compute initial accelerations)
        self.last_state = self.engine.step(0.0, &[]);
        self.build_observation(&self.last_state.clone())
    }

    /// Advance by one timestep.
    ///
    /// Returns `(observation, terminated, physics_state)`.
    /// Reward is computed by the caller (`lunar_lander_gym`).
    pub fn step(&mut self, action: Action) -> (Observation, bool, PhysicsState) {
        let lander_angle =
            self.last_state.bodies.get(LANDER_ID.0).map(|b| b.angle).unwrap_or(0.0);

        // Compute body-frame forces and torques from each thruster
        let (f_main, _) = self.thruster_main.apply(&[action[0]], DT);
        let (f_left, _) = self.thruster_left.apply(&[action[1]], DT);
        let (f_right, _) = self.thruster_right.apply(&[action[2]], DT);

        // Torques from leg thrusters (computed in body frame — scalar torque is frame-independent)
        let torque = OFFSET_LEFT.cross(f_left) + OFFSET_RIGHT.cross(f_right);

        // Rotate total body-frame force to world frame
        let total_body_force = f_main + f_left + f_right;
        let world_force = total_body_force.rotated(lander_angle);

        let forces = [ExternalForce { body: LANDER_ID, force: world_force, torque }];

        let state = self.engine.step(DT, &forces);
        self.last_state = state.clone();

        let terminated = termination::check_termination(
            &state,
            LANDER_ID,
            500.0,
            (-20.0, 20.0, -5.0, 200.0),
            (-5.0, 5.0),
            1.0,
        )
        .is_some();

        let obs = self.build_observation(&state);
        (obs, terminated, state)
    }

    #[must_use]
    pub fn config(&self) -> &LunarLanderConfig {
        &self.config
    }

    fn build_observation(&self, state: &PhysicsState) -> Observation {
        let imu_obs = self.imu.observe(state);
        let baro_obs = self.barometer.observe(state);
        let contact_obs = self.contact_sensor.observe(state);
        [
            imu_obs[0],                              // ax
            imu_obs[1],                              // ay
            imu_obs[2],                              // omega
            baro_obs[0],                             // altitude
            contact_obs[0],                          // contact
            self.thruster_main.current_normalized(), // thrust_main
            self.thruster_left.current_normalized(), // thrust_left
            self.thruster_right.current_normalized(), // thrust_right
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reset_returns_nonzero_observation() {
        let mut env = LunarLanderEnv::new(LunarLanderConfig::default());
        let obs = env.reset();
        // Lander starts at y=10, so altitude should be clearly above zero
        assert!(obs[3] > 5.0, "expected altitude > 5, got {}", obs[3]);
    }

    #[test]
    fn step_after_reset_does_not_panic() {
        let mut env = LunarLanderEnv::new(LunarLanderConfig::default());
        env.reset();
        let (obs, _terminated, _state) = env.step([0.0, 0.0, 0.0]);
        // Lander should still be above ground after one step
        assert!(obs[3] > 0.0);
    }
}
