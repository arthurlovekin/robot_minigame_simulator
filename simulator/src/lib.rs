pub mod actuators;
pub mod sensors;

pub use simulator_types::{BodyId, BodyState, ColliderShape, Contact, PhysicsState, Vec2};

/// Core physics engine.
pub struct PhysicsEngine;

impl PhysicsEngine {
    pub fn new() -> Self {
        PhysicsEngine
    }

    /// Advance the simulation by `dt` seconds and return the new world snapshot.
    pub fn step(&mut self, _dt: f64) -> PhysicsState {
        PhysicsState::default()
    }
}

impl Default for PhysicsEngine {
    fn default() -> Self {
        Self::new()
    }
}
