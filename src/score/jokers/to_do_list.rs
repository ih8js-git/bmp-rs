use crate::joker::JokerState;
use crate::levels::Hand;

pub fn score(_state: &JokerState, _hand: Hand) -> [f32; 3] {
    [0.0, 0.0, 0.0]
}

#[cfg(test)]
mod tests {
    #[test]
    #[ignore] // Reminder: unimplemented joker
    fn test_unimplemented() {
        assert!(false, "Joker not yet implemented");
    }
}
