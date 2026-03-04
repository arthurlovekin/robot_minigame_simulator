use simulator_types::PhysicsState;

/// Reason an episode ended.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TerminationReason {
    /// Lander collided with terrain above the survivable impact threshold.
    Crash,
    /// Lander left the allowed airspace bounding box.
    OutOfBounds,
    /// All fuel was exhausted.
    FuelExhausted,
    /// Lander touched down gently within the landing zone.
    SoftLanding,
}

/// Check whether the current state constitutes a terminal condition.
///
/// Returns `None` if the episode should continue, or `Some(reason)` to end it.
#[must_use]
pub fn check_termination(_state: &PhysicsState) -> Option<TerminationReason> {
    // Stub: episodes never terminate.
    None
}
