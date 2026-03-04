/// Top-level configuration for a Lunar Lander episode.
#[derive(Debug, Clone)]
pub struct LunarLanderConfig {
    /// Gravitational acceleration (m/s², positive = downward). Default: 1.62 (Moon).
    pub gravity: f32,
    /// Wind force magnitude (N). 0.0 = no wind.
    pub wind_power: f32,
    /// Turbulence magnitude (N, random per step). 0.0 = no turbulence.
    pub turbulence_power: f32,

    // --- Sensor params ---
    pub imu_noise_stddev: f32,
    pub imu_bias_drift: f32,
    pub lidar_num_rays: usize,
    pub lidar_max_range: f32,
    pub lidar_noise_stddev: f32,
    pub barometer_noise_stddev: f32,
    pub engine_feedback_noise_stddev: f32,

    // --- Actuator params ---
    pub thruster_max_force: f32,
    pub thruster_noise_stddev: f32,
    pub thruster_spin_up_time_constant: f32,
}

impl Default for LunarLanderConfig {
    fn default() -> Self {
        LunarLanderConfig {
            gravity: 1.62,
            wind_power: 0.0,
            turbulence_power: 0.0,
            imu_noise_stddev: 0.01,
            imu_bias_drift: 0.001,
            lidar_num_rays: 16,
            lidar_max_range: 50.0,
            lidar_noise_stddev: 0.05,
            barometer_noise_stddev: 0.05,
            engine_feedback_noise_stddev: 0.01,
            thruster_max_force: 200.0,
            thruster_noise_stddev: 0.02,
            thruster_spin_up_time_constant: 0.1,
        }
    }
}
