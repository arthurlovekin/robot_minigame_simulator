use simulator_types::{ColliderShape, PhysicsState, Vec2};

/// Configuration for the SVG renderer. Owned by the caller; `render` is a pure function.
#[derive(Debug, Clone, PartialEq)]
pub struct RenderConfig {
    pub width_px: u32,
    pub height_px: u32,
    /// Screen pixels per world metre.
    pub pixels_per_metre: f32,
    /// World-space point that maps to the centre of the viewport.
    pub camera_centre: Vec2,
}

impl Default for RenderConfig {
    fn default() -> Self {
        RenderConfig {
            width_px: 800,
            height_px: 600,
            pixels_per_metre: 20.0,
            camera_centre: Vec2 { x: 0.0, y: 5.0 },
        }
    }
}

/// Render a world snapshot as an SVG string.
///
/// Pure function — no side effects, no simulation state.
#[must_use]
pub fn render(state: &PhysicsState, config: &RenderConfig) -> String {
    let w = config.width_px as f32;
    let h = config.height_px as f32;
    let ppm = config.pixels_per_metre;
    let cam = config.camera_centre;

    let to_svg_x = |wx: f32| (wx - cam.x) * ppm + w * 0.5;
    let to_svg_y = |wy: f32| h * 0.5 - (wy - cam.y) * ppm;

    let mut elements = String::new();

    for (idx, body) in state.bodies.iter().enumerate() {
        let fill = if idx == 0 { "#4a4a4a" } else { "#c8d8e8" };

        for col in &body.colliders {
            let world_pos = body.position + col.offset.rotated(body.angle);
            let world_angle = body.angle + col.angle;

            let sx = to_svg_x(world_pos.x);
            let sy = to_svg_y(world_pos.y);
            // Negate angle: physics CCW positive, SVG CW positive (y-flip already applied).
            let deg = -world_angle.to_degrees();

            match col.shape {
                ColliderShape::Box { half_width, half_height } => {
                    let rw = half_width * 2.0 * ppm;
                    let rh = half_height * 2.0 * ppm;
                    elements.push_str(&format!(
                        r#"<rect x="{x:.1}" y="{y:.1}" width="{rw:.1}" height="{rh:.1}" fill="{fill}" transform="rotate({deg:.2},{sx:.1},{sy:.1})"/>"#,
                        x = sx - rw * 0.5,
                        y = sy - rh * 0.5,
                    ));
                }
                ColliderShape::Circle { radius } => {
                    let r = radius * ppm;
                    elements.push_str(&format!(
                        r#"<circle cx="{sx:.1}" cy="{sy:.1}" r="{r:.1}" fill="{fill}"/>"#,
                    ));
                }
                ColliderShape::Capsule { .. } => {
                    // Not used in lunar lander — skip.
                }
            }
        }
    }

    for contact in &state.contacts {
        if let (Some(a), Some(b)) =
            (state.bodies.get(contact.body_a.0), state.bodies.get(contact.body_b.0))
        {
            let mid_x = to_svg_x((a.position.x + b.position.x) * 0.5);
            let mid_y = to_svg_y((a.position.y + b.position.y) * 0.5);
            elements.push_str(&format!(
                r#"<circle cx="{mid_x:.1}" cy="{mid_y:.1}" r="4" fill="red"/>"#,
            ));
        }
    }

    format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" style="background:#0a0a1a">{}</svg>"#,
        config.width_px, config.height_px, elements
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use simulator_types::{BodyState, ColliderState, PhysicsState};

    fn box_body(position: Vec2, hw: f32, hh: f32) -> BodyState {
        BodyState {
            position,
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
    fn render_contains_svg_and_two_rects() {
        let state = PhysicsState {
            bodies: vec![
                box_body(Vec2 { x: 0.0, y: -0.5 }, 20.0, 0.5), // ground
                box_body(Vec2 { x: 0.0, y: 10.0 }, 0.5, 0.3),  // lander
            ],
            contacts: vec![],
        };
        let svg = render(&state, &RenderConfig::default());
        assert!(svg.contains("<svg"), "missing <svg tag");
        let rect_count = svg.matches("<rect").count();
        assert_eq!(rect_count, 2, "expected 2 <rect elements, got {rect_count}");
    }
}
