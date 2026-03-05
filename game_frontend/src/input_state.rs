use lunar_lander::Action;

/// Snapshot of currently held keys.
#[derive(Default)]
pub struct KeysState {
    pub thrust_main: bool,  // Space / ArrowUp
    pub thrust_left: bool,  // A / ArrowLeft  (fires left thruster, rotates CCW)
    pub thrust_right: bool, // D / ArrowRight (fires right thruster, rotates CW)
    pub restart: bool,      // R
}

impl KeysState {
    #[must_use]
    pub fn to_action(&self) -> Action {
        [
            if self.thrust_main { 1.0 } else { 0.0 },
            if self.thrust_left { 1.0 } else { 0.0 },
            if self.thrust_right { 1.0 } else { 0.0 },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_off_gives_zero_action() {
        assert_eq!(KeysState::default().to_action(), [0.0, 0.0, 0.0]);
    }

    #[test]
    fn space_fires_main_only() {
        let k = KeysState { thrust_main: true, ..Default::default() };
        assert_eq!(k.to_action(), [1.0, 0.0, 0.0]);
    }

    #[test]
    fn left_fires_left_only() {
        let k = KeysState { thrust_left: true, ..Default::default() };
        assert_eq!(k.to_action(), [0.0, 1.0, 0.0]);
    }

    #[test]
    fn right_fires_right_only() {
        let k = KeysState { thrust_right: true, ..Default::default() };
        assert_eq!(k.to_action(), [0.0, 0.0, 1.0]);
    }

    #[test]
    fn restart_false_by_default() {
        assert!(!KeysState::default().restart);
    }
}
