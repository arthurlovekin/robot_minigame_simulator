/// Lightweight 2-D vector (metres).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

/// Stable handle for a rigid body within the simulation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BodyId(pub usize);

/// Collider geometry tag (no physics mass/material — those live in the engine).
#[derive(Debug, Clone, PartialEq)]
pub enum ColliderShape {
    Circle { radius: f32 },
    Box { half_width: f32, half_height: f32 },
    Capsule { half_length: f32, radius: f32 },
}

/// Snapshot of a single rigid body at one instant.
#[derive(Debug, Clone, PartialEq)]
pub struct BodyState {
    pub position: Vec2,
    pub velocity: Vec2,
    pub angle: f32,
    pub angular_velocity: f32,
    pub shape: ColliderShape,
}

/// Contact event between two bodies.
#[derive(Debug, Clone, PartialEq)]
pub struct Contact {
    pub body_a: BodyId,
    pub body_b: BodyId,
    /// Normal impulse magnitude (N·s).
    pub impulse: f32,
}

/// Complete world snapshot produced by `PhysicsEngine::step`.
///
/// `bodies[id.0]` gives the state for `BodyId(id)`.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PhysicsState {
    pub bodies: Vec<BodyState>,
    pub contacts: Vec<Contact>,
}
