use crate::GameState;
use crate::stakes::Stake;
use crate::vouchers::{Voucher, has_voucher};

pub fn cash_out(gs: &mut GameState) {
    let reward = match gs.round % 3 {
        0 => {
            if gs.stake == Stake::White {
                3
            } else {
                0
            }
        }
        1 => 4,
        2 => 5,
        _ => panic!("Cash out could not determine the round"),
    };
    // TODO add logic for green deck (no interest)
    let mut interest = gs.balance / 5;
    if has_voucher(gs, Voucher::MoneyTree) {
        if interest >= 20 {
            interest = 20;
        }
    } else if has_voucher(gs, Voucher::SeedMoney) {
        if interest >= 10 {
            interest = 10;
        }
    } else if interest >= 5 {
        interest = 5;
    }
    gs.balance += reward + interest + (gs.hands - gs.hands_used) as u32;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::decks::Deck;
    use crate::game::create_game_state;

    #[test]
    fn test_cash_out() {
        let mut gs = create_game_state(Deck::Red);
        gs.stake = Stake::White;
        gs.round = 1;
        gs.hands_used = 1;

        let initial_balance = gs.balance;
        assert_eq!(initial_balance, 4);

        cash_out(&mut gs);

        assert_eq!(gs.balance, 4 + 3 + 0 + 3);
    }
}
