use simulator_types::{BodyId, PhysicsState};

/// Reason an episode ended.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TerminationReason {
    /// Lander collided with terrain above the survivable impact threshold.
    Crash,
    /// Lander left the allowed airspace bounding box.
    OutOfBounds,
    /// All fuel was exhausted (deferred).
    FuelExhausted,
    /// Lander touched down gently within the landing zone.
    SoftLanding,
}

/// Check whether the current state constitutes a terminal condition.
///
/// Returns `None` if the episode should continue, or `Some(reason)` to end it.
#[must_use]
pub fn check_termination(
    state: &PhysicsState,
    lander_id: BodyId,
    crash_threshold: f32,
    bounds: (f32, f32, f32, f32),
    landing_zone_x: (f32, f32),
    soft_landing_v_threshold: f32,
) -> Option<TerminationReason> {
    let body = &state.bodies[lander_id.0];
    let (min_x, max_x, min_y, max_y) = bounds;

    if body.position.x < min_x
        || body.position.x > max_x
        || body.position.y < min_y
        || body.position.y > max_y
    {
        return Some(TerminationReason::OutOfBounds);
    }

    for contact in &state.contacts {
        if contact.body_a == lander_id || contact.body_b == lander_id {
            if contact.impulse > crash_threshold {
                return Some(TerminationReason::Crash);
            }
            let v_y = body.velocity.y.abs();
            let in_zone =
                body.position.x >= landing_zone_x.0 && body.position.x <= landing_zone_x.1;
            if v_y <= soft_landing_v_threshold && in_zone {
                return Some(TerminationReason::SoftLanding);
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use simulator_types::{BodyState, ColliderShape, ColliderState, Contact, PhysicsState, Vec2};

    fn lander_state(pos: Vec2, v_y: f32) -> PhysicsState {
        PhysicsState {
            bodies: vec![BodyState {
                position: pos,
                velocity: Vec2 { x: 0.0, y: v_y },
                angle: 0.0,
                angular_velocity: 0.0,
                linear_acceleration: Vec2 { x: 0.0, y: 0.0 },
                angular_acceleration: 0.0,
                colliders: vec![ColliderState {
                    shape: ColliderShape::Box { half_width: 0.5, half_height: 0.3 },
                    offset: Vec2 { x: 0.0, y: 0.0 },
                    angle: 0.0,
                }],
            }],
            contacts: vec![],
        }
    }

    #[test]
    fn out_of_bounds_x() {
        let state = lander_state(Vec2 { x: 100.0, y: 5.0 }, 0.0);
        let result = check_termination(
            &state,
            BodyId(0),
            500.0,
            (-20.0, 20.0, -10.0, 200.0),
            (-5.0, 5.0),
            1.0,
        );
        assert_eq!(result, Some(TerminationReason::OutOfBounds));
    }

    #[test]
    fn soft_landing_in_zone() {
        let mut state = lander_state(Vec2 { x: 0.0, y: 0.1 }, -0.5);
        state.contacts =
            vec![Contact { body_a: BodyId(0), body_b: BodyId(1), impulse: 1.0 }];
        let result = check_termination(
            &state,
            BodyId(0),
            500.0,
            (-20.0, 20.0, -10.0, 200.0),
            (-5.0, 5.0),
            1.0,
        );
        assert_eq!(result, Some(TerminationReason::SoftLanding));
    }

    #[test]
    fn no_termination_in_flight() {
        let state = lander_state(Vec2 { x: 0.0, y: 5.0 }, -1.0);
        let result = check_termination(
            &state,
            BodyId(0),
            500.0,
            (-20.0, 20.0, -10.0, 200.0),
            (-5.0, 5.0),
            1.0,
        );
        assert!(result.is_none());
    }
}
