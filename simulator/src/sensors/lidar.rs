use simulator_types::{BodyId, ColliderShape, PhysicsState, Vec2};

use super::Sensor;

/// Lidar ring sensor: casts `num_rays` evenly-spaced rays and returns range measurements.
pub struct Lidar {
    pub body_id: BodyId,
    pub num_rays: usize,
    pub max_range: f32,
    pub noise_stddev: f32,
}

impl Sensor for Lidar {
    fn observe(&self, state: &PhysicsState) -> Vec<f32> {
        let origin = state.bodies[self.body_id.0].position;
        let mut ranges = vec![self.max_range; self.num_rays];

        for (ray_idx, r) in ranges.iter_mut().enumerate() {
            let angle = 2.0 * std::f32::consts::PI * ray_idx as f32 / self.num_rays as f32;
            let dir = Vec2 { x: angle.cos(), y: angle.sin() };

            for (body_idx, body) in state.bodies.iter().enumerate() {
                if body_idx == self.body_id.0 {
                    continue;
                }
                for col in &body.colliders {
                    let world_pos = body.position + col.offset.rotated(body.angle);
                    let world_angle = body.angle + col.angle;
                    if let Some(t) =
                        ray_hit(origin, dir, world_pos, world_angle, &col.shape, self.max_range)
                        && t < *r
                    {
                        *r = t;
                    }
                }
            }
        }
        ranges
    }
}

fn ray_hit(
    origin: Vec2,
    dir: Vec2,
    shape_pos: Vec2,
    shape_angle: f32,
    shape: &ColliderShape,
    max_range: f32,
) -> Option<f32> {
    match shape {
        ColliderShape::Box { half_width: hw, half_height: hh } => {
            ray_box(origin, dir, shape_pos, shape_angle, *hw, *hh, max_range)
        }
        ColliderShape::Circle { radius } => ray_circle(origin, dir, shape_pos, *radius, max_range),
        ColliderShape::Capsule { .. } => None,
    }
}

fn ray_box(
    origin: Vec2,
    dir: Vec2,
    box_pos: Vec2,
    box_angle: f32,
    hw: f32,
    hh: f32,
    max_range: f32,
) -> Option<f32> {
    // Transform ray to box local frame
    let (sin, cos) = box_angle.sin_cos();
    let d = origin - box_pos;
    let local_ox = d.x * cos + d.y * sin;
    let local_oy = -d.x * sin + d.y * cos;
    let local_dx = dir.x * cos + dir.y * sin;
    let local_dy = -dir.x * sin + dir.y * cos;

    let inv_dx = if local_dx.abs() > f32::EPSILON { 1.0 / local_dx } else { f32::INFINITY };
    let inv_dy = if local_dy.abs() > f32::EPSILON { 1.0 / local_dy } else { f32::INFINITY };

    let tx1 = (-hw - local_ox) * inv_dx;
    let tx2 = (hw - local_ox) * inv_dx;
    let ty1 = (-hh - local_oy) * inv_dy;
    let ty2 = (hh - local_oy) * inv_dy;

    let t_min = tx1.min(tx2).max(ty1.min(ty2));
    let t_max = tx1.max(tx2).min(ty1.max(ty2));

    if t_max < 0.0 || t_min > t_max {
        return None;
    }
    let t = if t_min >= 0.0 { t_min } else { t_max };
    if t >= 0.0 && t <= max_range { Some(t) } else { None }
}

fn ray_circle(origin: Vec2, dir: Vec2, center: Vec2, radius: f32, max_range: f32) -> Option<f32> {
    let oc = origin - center;
    let a = dir.dot(dir);
    let b = 2.0 * oc.dot(dir);
    let c = oc.dot(oc) - radius * radius;
    let disc = b * b - 4.0 * a * c;
    if disc < 0.0 {
        return None;
    }
    let sqrt_disc = disc.sqrt();
    let t1 = (-b - sqrt_disc) / (2.0 * a);
    let t2 = (-b + sqrt_disc) / (2.0 * a);
    let t = if t1 >= 0.0 { t1 } else if t2 >= 0.0 { t2 } else { return None };
    if t <= max_range { Some(t) } else { None }
}

#[cfg(test)]
mod tests {
    use super::*;
    use simulator_types::{BodyState, ColliderShape, ColliderState, PhysicsState, Vec2};

    fn box_body(pos: Vec2, hw: f32, hh: f32) -> BodyState {
        BodyState {
            position: pos,
            velocity: Vec2 { x: 0.0, y: 0.0 },
            angle: 0.0,
            angular_velocity: 0.0,
            linear_acceleration: Vec2 { x: 0.0, y: 0.0 },
            angular_acceleration: 0.0,
            colliders: vec![ColliderState {
                shape: ColliderShape::Box { half_width: hw, half_height: hh },
                offset: Vec2 { x: 0.0, y: 0.0 },
                angle: 0.0,
            }],
        }
    }

    #[test]
    fn ray_hits_nearby_box() {
        // Lidar on body 0 at origin; body 1 is a box 5 m to the right
        let state = PhysicsState {
            bodies: vec![
                box_body(Vec2 { x: 0.0, y: 0.0 }, 0.1, 0.1),
                box_body(Vec2 { x: 5.0, y: 0.0 }, 0.5, 0.5),
            ],
            contacts: vec![],
        };
        let lidar = Lidar { body_id: BodyId(0), num_rays: 4, max_range: 50.0, noise_stddev: 0.0 };
        let obs = lidar.observe(&state);
        // Ray 0 points right (angle=0): should hit the box at ~4.5 m
        assert!(obs[0] < 5.0, "expected hit, got {}", obs[0]);
    }

    #[test]
    fn ray_returns_max_range_when_no_obstacle() {
        let state = PhysicsState {
            bodies: vec![box_body(Vec2 { x: 0.0, y: 0.0 }, 0.1, 0.1)],
            contacts: vec![],
        };
        let lidar =
            Lidar { body_id: BodyId(0), num_rays: 4, max_range: 50.0, noise_stddev: 0.0 };
        let obs = lidar.observe(&state);
        assert!(obs.iter().all(|&r| (r - 50.0).abs() < 1e-5));
    }
}
