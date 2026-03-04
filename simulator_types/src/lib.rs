use std::ops::{Add, Mul, Neg, Sub};

/// Lightweight 2-D vector (metres).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    #[must_use]
    pub fn dot(self, other: Vec2) -> f32 {
        self.x * other.x + self.y * other.y
    }

    #[must_use]
    pub fn length(self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    #[must_use]
    pub fn normalized(self) -> Vec2 {
        let len = self.length();
        if len < f32::EPSILON {
            Vec2 { x: 0.0, y: 0.0 }
        } else {
            Vec2 { x: self.x / len, y: self.y / len }
        }
    }

    /// Rotate this vector by `angle` radians counter-clockwise.
    #[must_use]
    pub fn rotated(self, angle: f32) -> Vec2 {
        let (sin, cos) = angle.sin_cos();
        Vec2 {
            x: self.x * cos - self.y * sin,
            y: self.x * sin + self.y * cos,
        }
    }

    /// 2-D "cross product" — returns the z-component of the 3-D cross product.
    #[must_use]
    pub fn cross(self, other: Vec2) -> f32 {
        self.x * other.y - self.y * other.x
    }
}

impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, other: Vec2) -> Vec2 {
        Vec2 { x: self.x + other.x, y: self.y + other.y }
    }
}

impl Sub for Vec2 {
    type Output = Vec2;
    fn sub(self, other: Vec2) -> Vec2 {
        Vec2 { x: self.x - other.x, y: self.y - other.y }
    }
}

impl Mul<f32> for Vec2 {
    type Output = Vec2;
    fn mul(self, s: f32) -> Vec2 {
        Vec2 { x: self.x * s, y: self.y * s }
    }
}

impl Neg for Vec2 {
    type Output = Vec2;
    fn neg(self) -> Vec2 {
        Vec2 { x: -self.x, y: -self.y }
    }
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

/// A single collider attached to a body, with body-frame offset and angle.
#[derive(Debug, Clone, PartialEq)]
pub struct ColliderState {
    pub shape: ColliderShape,
    /// Position offset relative to the body centre of mass (body frame).
    pub offset: Vec2,
    /// Rotation relative to the body angle (radians).
    pub angle: f32,
}

/// Snapshot of a single rigid body at one instant.
#[derive(Debug, Clone, PartialEq)]
pub struct BodyState {
    pub position: Vec2,
    pub velocity: Vec2,
    pub angle: f32,
    pub angular_velocity: f32,
    pub linear_acceleration: Vec2,
    pub angular_acceleration: f32,
    pub colliders: Vec<ColliderState>,
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::FRAC_PI_2;

    #[test]
    fn vec2_rotate_90_deg() {
        let v = Vec2 { x: 1.0, y: 0.0 };
        let r = v.rotated(FRAC_PI_2);
        assert!((r.x).abs() < 1e-6, "expected x≈0, got {}", r.x);
        assert!((r.y - 1.0).abs() < 1e-6, "expected y≈1, got {}", r.y);
    }

    #[test]
    fn vec2_dot() {
        let a = Vec2 { x: 1.0, y: 2.0 };
        let b = Vec2 { x: 3.0, y: 4.0 };
        assert_eq!(a.dot(b), 11.0);
    }

    #[test]
    fn vec2_length() {
        let v = Vec2 { x: 3.0, y: 4.0 };
        assert!((v.length() - 5.0).abs() < 1e-6);
    }

    #[test]
    fn vec2_cross() {
        let a = Vec2 { x: 1.0, y: 0.0 };
        let b = Vec2 { x: 0.0, y: 1.0 };
        assert_eq!(a.cross(b), 1.0);
    }
}
