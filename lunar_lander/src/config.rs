/// Top-level configuration for a Lunar Lander episode.
#[derive(Debug, Clone)]
pub struct LunarLanderConfig {
    /// Gravitational acceleration (m/s², positive = downward). Default: 1.62 (Moon).
    pub gravity: f64,
    /// Wind force magnitude (N). 0.0 = no wind.
    pub wind_power: f64,
    /// Turbulence magnitude (N, random per step). 0.0 = no turbulence.
    pub turbulence_power: f64,

    // --- Sensor params ---
    /// IMU noise standard deviation (m/s², rad/s).
    pub imu_noise_stddev: f64,
    /// IMU bias drift rate (m/s²/s, rad/s/s).
    pub imu_bias_drift: f64,
    /// Lidar ray count.
    pub lidar_num_rays: usize,
    /// Lidar maximum range (metres).
    pub lidar_max_range: f64,
    /// Lidar range noise standard deviation (metres).
    pub lidar_noise_stddev: f64,
    /// Barometer noise standard deviation (metres).
    pub barometer_noise_stddev: f64,
    /// Engine feedback noise standard deviation (normalised 0–1).
    pub engine_feedback_noise_stddev: f64,

    // --- Actuator params ---
    /// Maximum thruster force (Newtons).
    pub thruster_max_force: f64,
    /// Thruster force noise standard deviation (fraction of max_force).
    pub thruster_noise_stddev: f64,
    /// Thruster spin-up time constant (seconds).
    pub thruster_spin_up_time_constant: f64,
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
