use crate::joker::JokerState;
use crate::levels::Hand;

pub fn score(
    _state: &JokerState,
    _hand: Hand,
    _chips: &mut f64,
    _mult: &mut f64,
) -> Result<(), &'static str> {
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    #[ignore] // Reminder: unimplemented joker
    fn test_unimplemented() {
        assert!(false, "Joker not yet implemented");
    }
}
