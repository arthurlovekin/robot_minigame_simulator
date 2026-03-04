use simulator_types::{ColliderShape, Vec2};

use crate::body::Body;

/// Oriented Bounding Box
struct Obb {
    pos: Vec2,
    angle: f32,
    hw: f32,
    hh: f32,
}

/// Transform a collider at `geom_idx` to world frame.
/// Returns `(world_pos, world_angle, &shape)`.
pub(crate) fn world_collider(body: &Body, geom_idx: usize) -> (Vec2, f32, &ColliderShape) {
    let col = &body.colliders[geom_idx];
    let world_pos = body.position + col.offset.rotated(body.angle);
    let world_angle = body.angle + col.angle;
    (world_pos, world_angle, &col.shape)
}

/// Separating Axis Theorem (SAT) for Oriented Bounding Box (OBB) vs OBB collision.
/// Returns `Some((normal, penetration_depth))` if the two boxes are intersecting (penetrating),
/// where the normal points from box `b` toward box `a`.
fn obb_obb_penetration_axis(a: &Obb, b: &Obb) -> Option<(Vec2, f32)> {
    let axes = [
        Vec2 { x: a.angle.cos(), y: a.angle.sin() },
        Vec2 { x: -a.angle.sin(), y: a.angle.cos() },
        Vec2 { x: b.angle.cos(), y: b.angle.sin() },
        Vec2 { x: -b.angle.sin(), y: b.angle.cos() },
    ];

    let corners_a = box_corners(a.pos, a.angle, a.hw, a.hh);
    let corners_b = box_corners(b.pos, b.angle, b.hw, b.hh);

    let mut min_depth = f32::MAX;
    let mut min_axis = Vec2 { x: 0.0, y: 0.0 };

    for axis in &axes {
        let (min_a, max_a) = project_corners(&corners_a, *axis);
        let (min_b, max_b) = project_corners(&corners_b, *axis);
        let overlap = max_a.min(max_b) - min_a.max(min_b);
        if overlap <= 0.0 {
            return None;
        }
        if overlap < min_depth {
            min_depth = overlap;
            min_axis = *axis;
        }
    }

    // Orient normal from b toward a
    let d = a.pos - b.pos;
    if d.dot(min_axis) < 0.0 {
        min_axis = -min_axis;
    }

    Some((min_axis, min_depth))
}

fn box_corners(pos: Vec2, angle: f32, hw: f32, hh: f32) -> [Vec2; 4] {
    let (sin, cos) = angle.sin_cos();
    let ax = Vec2 { x: cos * hw, y: sin * hw };
    let ay = Vec2 { x: -sin * hh, y: cos * hh };
    [pos + ax + ay, pos - ax + ay, pos - ax - ay, pos + ax - ay]
}

fn project_corners(corners: &[Vec2; 4], axis: Vec2) -> (f32, f32) {
    let mut min = f32::MAX;
    let mut max = f32::MIN;
    for c in corners {
        let p = c.dot(axis);
        if p < min {
            min = p;
        }
        if p > max {
            max = p;
        }
    }
    (min, max)
}

/// Circle vs OBB. Returns `Some((normal_from_box_toward_circle, depth))`.
pub(crate) fn circle_box(
    circle_pos: Vec2,
    r: f32,
    box_pos: Vec2,
    box_angle: f32,
    hw: f32,
    hh: f32,
) -> Option<(Vec2, f32)> {
    let d = circle_pos - box_pos;
    let (sin, cos) = box_angle.sin_cos();
    let local_x = d.x * cos + d.y * sin;
    let local_y = -d.x * sin + d.y * cos;

    let inside = local_x.abs() <= hw && local_y.abs() <= hh;
    let clamped_x = local_x.clamp(-hw, hw);
    let clamped_y = local_y.clamp(-hh, hh);
    let dx = local_x - clamped_x;
    let dy = local_y - clamped_y;
    let dist_sq = dx * dx + dy * dy;

    if !inside && dist_sq >= r * r {
        return None;
    }

    if inside {
        let overlap_x = hw - local_x.abs();
        let overlap_y = hh - local_y.abs();
        let (normal_local, depth) = if overlap_x < overlap_y {
            (Vec2 { x: local_x.signum(), y: 0.0 }, overlap_x + r)
        } else {
            (Vec2 { x: 0.0, y: local_y.signum() }, overlap_y + r)
        };
        let normal = Vec2 {
            x: normal_local.x * cos - normal_local.y * sin,
            y: normal_local.x * sin + normal_local.y * cos,
        };
        Some((normal, depth))
    } else {
        let dist = dist_sq.sqrt();
        let depth = r - dist;
        let nx = dx / dist;
        let ny = dy / dist;
        let normal = Vec2 {
            x: nx * cos - ny * sin,
            y: nx * sin + ny * cos,
        };
        Some((normal, depth))
    }
}

fn circle_circle(pos_a: Vec2, ra: f32, pos_b: Vec2, rb: f32) -> Option<(Vec2, f32)> {
    let d = pos_a - pos_b;
    let dist = d.length();
    let min_dist = ra + rb;
    if dist >= min_dist {
        return None;
    }
    let depth = min_dist - dist;
    let normal = if dist < f32::EPSILON {
        Vec2 { x: 0.0, y: 1.0 }
    } else {
        d * (1.0 / dist)
    };
    Some((normal, depth))
}

fn test_shapes(
    pos_a: Vec2,
    angle_a: f32,
    shape_a: &ColliderShape,
    pos_b: Vec2,
    angle_b: f32,
    shape_b: &ColliderShape,
) -> Option<(Vec2, f32)> {
    match (shape_a, shape_b) {
        (
            ColliderShape::Box { half_width: hw_a, half_height: hh_a },
            ColliderShape::Box { half_width: hw_b, half_height: hh_b },
        ) => obb_obb_penetration_axis(
            &Obb { pos: pos_a, angle: angle_a, hw: *hw_a, hh: *hh_a },
            &Obb { pos: pos_b, angle: angle_b, hw: *hw_b, hh: *hh_b },
        ),
        (ColliderShape::Circle { radius }, ColliderShape::Box { half_width: hw, half_height: hh }) => {
            circle_box(pos_a, *radius, pos_b, angle_b, *hw, *hh)
        }
        (ColliderShape::Box { half_width: hw, half_height: hh }, ColliderShape::Circle { radius }) => {
            // swap and negate normal
            circle_box(pos_b, *radius, pos_a, angle_a, *hw, *hh).map(|(n, d)| (-n, d))
        }
        (ColliderShape::Circle { radius: ra }, ColliderShape::Circle { radius: rb }) => {
            circle_circle(pos_a, *ra, pos_b, *rb)
        }
        _ => None, // Capsule not yet implemented
    }
}

/// O(n²) broadphase + narrowphase contact detection.
/// Returns `(body_a_idx, body_b_idx, normal_a_from_b, depth)` tuples.
pub(crate) fn detect_contacts(bodies: &[Body]) -> Vec<(usize, usize, Vec2, f32)> {
    let mut contacts = Vec::new();
    for i in 0..bodies.len() {
        for j in (i + 1)..bodies.len() {
            if bodies[i].is_static && bodies[j].is_static {
                continue;
            }
            for gi in 0..bodies[i].colliders.len() {
                for gj in 0..bodies[j].colliders.len() {
                    let (pos_a, angle_a, shape_a) = world_collider(&bodies[i], gi);
                    let (pos_b, angle_b, shape_b) = world_collider(&bodies[j], gj);
                    if let Some((normal, depth)) =
                        test_shapes(pos_a, angle_a, shape_a, pos_b, angle_b, shape_b)
                    {
                        contacts.push((i, j, normal, depth));
                    }
                }
            }
        }
    }
    contacts
}

/// Apply a separating impulse + Baumgarte position correction for one contact.
/// `normal` points from body_b toward body_a. Returns the impulse magnitude.
pub(crate) fn apply_impulse(
    bodies: &mut [Body],
    idx_a: usize,
    idx_b: usize,
    normal: Vec2,
    depth: f32,
) -> f32 {
    // Estimate contact point as midpoint between the two body centres
    let r_a = (bodies[idx_b].position - bodies[idx_a].position) * 0.5;
    let r_b = (bodies[idx_a].position - bodies[idx_b].position) * 0.5;

    let v_a = bodies[idx_a].velocity + Vec2 { x: -r_a.y, y: r_a.x } * bodies[idx_a].angular_velocity;
    let v_b = bodies[idx_b].velocity + Vec2 { x: -r_b.y, y: r_b.x } * bodies[idx_b].angular_velocity;
    let v_rel = v_a - v_b;
    let v_along_normal = v_rel.dot(normal);

    // Only resolve if bodies are approaching
    if v_along_normal > 0.0 {
        return 0.0;
    }

    let e = (bodies[idx_a].restitution + bodies[idx_b].restitution) * 0.5;
    let ra_cross_n = r_a.cross(normal);
    let rb_cross_n = r_b.cross(normal);
    let denom = bodies[idx_a].inv_mass
        + bodies[idx_b].inv_mass
        + ra_cross_n * ra_cross_n * bodies[idx_a].inv_moi
        + rb_cross_n * rb_cross_n * bodies[idx_b].inv_moi;

    if denom < f32::EPSILON {
        return 0.0;
    }

    let j = -(1.0 + e) * v_along_normal / denom;
    let impulse = normal * j;

    if !bodies[idx_a].is_static {
        bodies[idx_a].velocity = bodies[idx_a].velocity + impulse * bodies[idx_a].inv_mass;
        bodies[idx_a].angular_velocity += r_a.cross(impulse) * bodies[idx_a].inv_moi;
    }
    if !bodies[idx_b].is_static {
        bodies[idx_b].velocity = bodies[idx_b].velocity - impulse * bodies[idx_b].inv_mass;
        bodies[idx_b].angular_velocity -= r_b.cross(impulse) * bodies[idx_b].inv_moi;
    }

    // Baumgarte position correction
    const SLOP: f32 = 0.01;
    const BAUMGARTE: f32 = 0.2;
    let lin_denom = bodies[idx_a].inv_mass + bodies[idx_b].inv_mass;
    if lin_denom > f32::EPSILON {
        let correction = (depth - SLOP).max(0.0) * BAUMGARTE / lin_denom;
        let corr = normal * correction;
        if !bodies[idx_a].is_static {
            bodies[idx_a].position = bodies[idx_a].position + corr * bodies[idx_a].inv_mass;
        }
        if !bodies[idx_b].is_static {
            bodies[idx_b].position = bodies[idx_b].position - corr * bodies[idx_b].inv_mass;
        }
    }

    j
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sat_non_overlapping_returns_none() {
        let result = obb_obb_penetration_axis(
            &Obb { pos: Vec2 { x: 0.0, y: 0.0 }, angle: 0.0, hw: 0.5, hh: 0.5 },
            &Obb { pos: Vec2 { x: 5.0, y: 0.0 }, angle: 0.0, hw: 0.5, hh: 0.5 },
        );
        assert!(result.is_none());
    }

    #[test]
    fn sat_overlapping_returns_some() {
        let result = obb_obb_penetration_axis(
            &Obb { pos: Vec2 { x: 0.0, y: 0.0 }, angle: 0.0, hw: 1.0, hh: 1.0 },
            &Obb { pos: Vec2 { x: 1.0, y: 0.0 }, angle: 0.0, hw: 1.0, hh: 1.0 },
        );
        assert!(result.is_some());
        let (normal, depth) = result.unwrap();
        assert!(depth > 0.0 && depth < 2.0);
        assert!((normal.length() - 1.0).abs() < 1e-5);
    }

    #[test]
    fn circle_box_inside_returns_contact() {
        let result =
            circle_box(Vec2 { x: 0.0, y: 0.0 }, 0.1, Vec2 { x: 0.0, y: 0.0 }, 0.0, 1.0, 1.0);
        assert!(result.is_some());
    }

    #[test]
    fn circle_box_far_outside_returns_none() {
        let result =
            circle_box(Vec2 { x: 5.0, y: 0.0 }, 0.3, Vec2 { x: 0.0, y: 0.0 }, 0.0, 1.0, 1.0);
        assert!(result.is_none());
    }
}
