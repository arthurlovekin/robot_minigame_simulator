/// Observation vector: [ax, ay, omega, altitude, contact, thrust_main, thrust_left, thrust_right].
pub type Observation = [f32; 8];

/// Action vector: throttle commands for [main, left, right] thrusters, each in [0, 1].
pub type Action = [f32; 3];
