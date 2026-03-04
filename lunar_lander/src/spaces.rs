/// Observation vector: [IMU(3), lidar(16), barometer(1), contact(1), engine_feedback(1)] — 22 values minimum.
/// Sized to accommodate the default sensor configuration (8 values shown in plan).
pub type Observation = [f64; 8];

/// Action vector: throttle commands for the three thrusters, each in [0, 1].
pub type Action = [f64; 3];
