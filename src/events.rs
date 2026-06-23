use crate::blinds::Blind;
use crate::card::Card;
use crate::stakes::Stake;
use crate::vouchers::{Voucher, has_voucher};
use crate::{GameState, score};

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
    gs.balance += reward + interest + (gs.hands - gs.hands_used) as u32;
}

pub fn select_card(gs: &mut GameState, card_index: usize) {
    gs.selected_card_indices[gs.selected_card_count] = card_index;
    gs.selected_card_count += 1
}

// TODO: Right now this function is just a wrapper for the get_score function
// but in the future it will do more things. It should be a simple high level
// function that you call that will then call everything it needs to from there
// TODO: either inside or outside this function we need to check if we actually
// have enough hands to play another hand or not, becuase right now nothing is
// stoping us from playing more hands than we actually have
pub fn play_hand(gs: &mut GameState) {
    gs.hands_used += 1;
    let dummy_card = gs.hand[0];

    // 2. Initialize our fixed array on the stack (zero heap allocation!)
    let mut cards_to_play: [Card; 5] = [dummy_card; 5];

    // 3. Get only the valid selections based on the count
    let valid_selections = &gs.selected_card_indices[0..gs.selected_card_count];

    // 4. Overwrite the dummy cards with the actual selected cards from the hand
    for (i, &hand_index) in valid_selections.iter().enumerate() {
        cards_to_play[i] = gs.hand[hand_index];
    }

    // 5. Pass a *slice* of the array containing only the valid cards to play_hand
    // By slicing `0..gs.selected_card_count`, play_hand will never see the dummy cards.
    gs.current_score = score::core::get_score(gs, &cards_to_play[0..gs.selected_card_count]);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blinds::Blind::Small;
    use crate::decks::Deck;
    use crate::game::create_game_state;

    #[test]
    fn test_cash_out() {
        let mut gs = create_game_state(Deck::Red);
        gs.stake = Stake::White;
        gs.next_blind = Small;
        gs.hands_used = 1;

        let initial_balance = gs.balance;
        assert_eq!(initial_balance, 4);

        cash_out(&mut gs);

        assert_eq!(gs.balance, 4 + 3 + 0 + 3);
    }
}
