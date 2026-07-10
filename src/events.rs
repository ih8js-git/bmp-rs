use crate::blinds::Blind;
use crate::card::Card;
use crate::stakes::Stake;
use crate::vouchers::{Voucher, has_voucher};
use crate::{game::state::GameState, score};
use strum_macros::EnumCount;

#[derive(EnumCount, PartialEq, Debug, Clone, Copy)]
#[repr(u8)]
pub enum JokerUpdateEvent {
    HandPlayed,
    Discard,
    BlindSelect,
    GoNext,
    PlanetUsed,
    TarotUsed,
    Sell,
}

pub fn cash_out(gs: &mut GameState) {
    let reward = match gs.next_blind {
        Blind::Small => {
            if gs.stake == Stake::White {
                3
            } else {
                0
            }
        }
        Blind::Big => 4,
        Blind::Boss => 5,
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
    gs.balance += reward + interest + gs.hands_remaining as u32;
}

// TODO: Right now this function is just a wrapper for the get_score function
// but in the future it will do more things. It should be a simple high level
// function that you call that will then call everything it needs to from there
// TODO: either inside or outside this function we need to check if we actually
// have enough hands to play another hand or not, becuase right now nothing is
// stoping us from playing more hands than we actually have
pub fn play_hand(gs: &mut GameState, cards_to_play: [u16; 5]) {
    gs.hands_remaining -= 1;
    let actual_cards: Vec<Card> = cards_to_play
        .iter()
        .map(|&idx| gs.cards[idx as usize])
        .collect();
    gs.current_score = score::core::get_score(gs, &actual_cards);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blinds::Blind::Small;
    use crate::decks::Deck;
    use crate::game::state::create_game_state;

    #[test]
    fn test_cash_out() {
        let mut gs = create_game_state(Deck::Red);
        gs.stake = Stake::White;
        gs.next_blind = Small;
        gs.hands_remaining = 3;

        let initial_balance = gs.balance;
        assert_eq!(initial_balance, 4);

        cash_out(&mut gs);

        assert_eq!(gs.balance, 4 + 3 + 0 + 3);
    }
}
