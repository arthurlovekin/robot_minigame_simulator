use simulator_types::PhysicsState;
use super::Sensor;

/// Boolean ground-contact sensor: returns 1.0 if in contact with ground, 0.0 otherwise.
pub struct ContactSensor;

impl Sensor for ContactSensor {
    fn observe(&self, _state: &PhysicsState) -> Vec<f64> {
        // Stub: always returns no contact.
        vec![0.0]
    }
}
