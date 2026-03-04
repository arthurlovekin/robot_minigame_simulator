use simulator_types::PhysicsState;

/// Render a world snapshot as an SVG string.
///
/// Pure function — no side effects, no simulation state.
#[must_use]
pub fn render(_state: &PhysicsState) -> String {
    // Stub: empty SVG canvas.
    r#"<svg xmlns="http://www.w3.org/2000/svg" width="800" height="600"></svg>"#.to_string()
}
