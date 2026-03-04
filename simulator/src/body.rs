use simulator_types::{BodyId, ColliderShape, Vec2};

/// Definition of one collider within a body, plus material property.
#[derive(Debug, Clone)]
pub struct ColliderDef {
    pub shape: ColliderShape,
    /// Offset from the body's centre of mass (body frame, metres).
    pub offset: Vec2,
    /// Angle relative to the body's orientation (radians).
    pub angle: f32,
    /// Coefficient of restitution [0, 1].
    pub restitution: f32,
}

/// Everything needed to add a new rigid body to the simulation.
#[derive(Debug, Clone)]
pub struct BodyDef {
    pub position: Vec2,
    pub velocity: Vec2,
    pub angle: f32,
    pub angular_velocity: f32,
    /// Uniform area density (kg/m²). Use `0.0` for static bodies.
    pub density: f32,
    pub colliders: Vec<ColliderDef>,
    pub is_static: bool,
}

/// A pre-computed force/torque applied to a body for one timestep.
#[derive(Debug, Clone)]
pub struct ExternalForce {
    pub body: BodyId,
    /// Force in world frame (N).
    pub force: Vec2,
    /// Torque about the body's centre of mass (N·m).
    pub torque: f32,
}

/// Internal rigid body with all derived quantities cached.
pub(crate) struct Body {
    pub position: Vec2,
    pub velocity: Vec2,
    pub angle: f32,
    pub angular_velocity: f32,
    pub linear_acceleration: Vec2,
    pub angular_acceleration: f32,
    pub mass: f32,
    pub inv_mass: f32,
    pub inv_moi: f32,
    pub colliders: Vec<ColliderDef>,
    /// Average restitution coefficient across all colliders.
    pub restitution: f32,
    pub is_static: bool,
}

impl Body {
    /// Construct from a `BodyDef`, auto-computing mass and MOI from geometry + density.
    pub fn from_def(def: BodyDef) -> Self {
        let (mass, moi, restitution) = compute_inertia(&def);
        let (inv_mass, inv_moi) = if def.is_static || mass < f32::EPSILON {
            (0.0, 0.0)
        } else {
            (1.0 / mass, 1.0 / moi.max(f32::EPSILON))
        };
        Body {
            position: def.position,
            velocity: def.velocity,
            angle: def.angle,
            angular_velocity: def.angular_velocity,
            linear_acceleration: Vec2 { x: 0.0, y: 0.0 },
            angular_acceleration: 0.0,
            mass,
            inv_mass,
            inv_moi,
            colliders: def.colliders,
            restitution,
            is_static: def.is_static,
        }
    }
}

fn compute_inertia(def: &BodyDef) -> (f32, f32, f32) {
    if def.is_static || def.density < f32::EPSILON {
        return (0.0, 0.0, 0.0);
    }
    let mut total_mass = 0.0_f32;
    let mut total_moi = 0.0_f32;
    let mut total_restitution = 0.0_f32;

    for col in &def.colliders {
        let (area, i_cm_specific) = shape_inertia(&col.shape);
        let m = def.density * area;
        let offset_sq = col.offset.x * col.offset.x + col.offset.y * col.offset.y;
        // I_cm = m * i_cm_specific; parallel axis: I_total = I_cm + m * d²
        let moi = m * (i_cm_specific + offset_sq);
        total_mass += m;
        total_moi += moi;
        total_restitution += col.restitution;
    }

    let avg_restitution = if def.colliders.is_empty() {
        0.0
    } else {
        total_restitution / def.colliders.len() as f32
    };

    (total_mass, total_moi.max(f32::EPSILON), avg_restitution)
}

/// Returns `(area, i_cm_specific)` where `i_cm_specific = I_cm / m`.
fn shape_inertia(shape: &ColliderShape) -> (f32, f32) {
    match shape {
        ColliderShape::Box { half_width: hw, half_height: hh } => {
            let area = 4.0 * hw * hh;
            let i_cm = (hw * hw + hh * hh) / 3.0;
            (area, i_cm)
        }
        ColliderShape::Circle { radius: r } => {
            let area = std::f32::consts::PI * r * r;
            let i_cm = r * r / 2.0;
            (area, i_cm)
        }
        ColliderShape::Capsule { half_length, radius } => {
            let rect_area = 2.0 * radius * 2.0 * half_length;
            let circle_area = std::f32::consts::PI * radius * radius;
            let area = rect_area + circle_area;
            // Rough approximation
            let i_cm = (radius * radius + half_length * half_length) / 3.0;
            (area, i_cm)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn moi_rectangle_matches_formula() {
        let hw = 0.5_f32;
        let hh = 0.3_f32;
        let density = 10.0_f32;
        let def = BodyDef {
            position: Vec2 { x: 0.0, y: 0.0 },
            velocity: Vec2 { x: 0.0, y: 0.0 },
            angle: 0.0,
            angular_velocity: 0.0,
            density,
            colliders: vec![ColliderDef {
                shape: ColliderShape::Box { half_width: hw, half_height: hh },
                offset: Vec2 { x: 0.0, y: 0.0 },
                angle: 0.0,
                restitution: 0.5,
            }],
            is_static: false,
        };
        let body = Body::from_def(def);
        let area = 4.0 * hw * hh;
        let mass = density * area;
        let expected_moi = mass * (hw * hw + hh * hh) / 3.0;
        let expected_inv_moi = 1.0 / expected_moi;
        assert!(
            (body.inv_moi - expected_inv_moi).abs() < 1e-5,
            "inv_moi={} expected={}",
            body.inv_moi,
            expected_inv_moi
        );
    }

    #[test]
    fn static_body_has_zero_inv_mass() {
        let def = BodyDef {
            position: Vec2 { x: 0.0, y: 0.0 },
            velocity: Vec2 { x: 0.0, y: 0.0 },
            angle: 0.0,
            angular_velocity: 0.0,
            density: 10.0,
            colliders: vec![ColliderDef {
                shape: ColliderShape::Box { half_width: 1.0, half_height: 1.0 },
                offset: Vec2 { x: 0.0, y: 0.0 },
                angle: 0.0,
                restitution: 0.5,
            }],
            is_static: true,
        };
        let body = Body::from_def(def);
        assert_eq!(body.inv_mass, 0.0);
        assert_eq!(body.inv_moi, 0.0);
    }
}
