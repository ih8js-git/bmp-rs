use crate::GameState;
use crate::card::Edition;
use crate::joker::{JOKER_DEFS, JokerState};
use crate::vouchers::{Voucher, has_voucher};
use std::cmp::max;
use std::sync::OnceLock;
use strum::{EnumCount, IntoEnumIterator};

pub const JOKER_AMOUNT: usize = 150;

pub const DISCOUNT_VOUCHER_MULTIPLIER: [f32; 3] = [1.0, 0.75, 0.5];

pub type CostTable = [[u8; Edition::COUNT]; DISCOUNT_VOUCHER_MULTIPLIER.len()];

// for every joker, a table containing the cost for every voucher (row) & edition (col) combo
pub static JOKER_COST_TABLES: OnceLock<[CostTable; JOKER_AMOUNT]> = OnceLock::new();
pub fn init_joker_base_cost_table() {
    JOKER_COST_TABLES.get_or_init(|| {
        let mut temp_tables =
            [[[0u8; Edition::COUNT]; DISCOUNT_VOUCHER_MULTIPLIER.len()]; JOKER_AMOUNT];

        for (joker, base_cost) in JOKER_DEFS.iter().map(|def| def.base_price()).enumerate() {
            for (discount_index, discount_multiplier) in
                DISCOUNT_VOUCHER_MULTIPLIER.iter().enumerate()
            {
                for edition in Edition::iter() {
                    let f_base_cost = base_cost as f32;
                    let f_edition_added_cost = edition.added_cost() as f32;
                    let f_res = (f_base_cost + f_edition_added_cost + 0.5) * discount_multiplier;

                    temp_tables[joker][discount_index][edition as usize] = f_res.floor() as u8;
                }
            }
        }

        temp_tables
    });
}

pub fn get_joker_cost(j: &JokerState, gs: &GameState) -> u8 {
    if j.is_rental() {
        return 1;
    }

    if JOKER_COST_TABLES.get().is_none() {
        init_joker_base_cost_table();
    }

    // 0 = no voucher, 1 = only 1st voucher, 2 = both vouchers
    let voucher = if has_voucher(gs, Voucher::ClearanceSale) {
        1
    } else {
        0
    } + if has_voucher(gs, Voucher::Liquidation) {
        1
    } else {
        0
    };

    let tables = JOKER_COST_TABLES.get().unwrap();
    tables[j.id() as usize][voucher as usize][j.edition() as usize]
}

pub fn get_joker_sell_value(j: &JokerState, gs: &GameState) -> u8 {
    return max(1, j.added_sell_value() + (get_joker_cost(j, gs) >> 1));
}

#[cfg(test)]
mod tests {
    use crate::card::Edition;
    use crate::decks::Deck;
    use crate::game::create_game_state;
    use crate::joker::{Joker, JokerState};
    use crate::tables::{get_joker_cost, get_joker_sell_value};

    #[test]
    fn test_joker_base_cost_no_edition_no_voucher() {
        let gs = create_game_state(Deck::Red);

        // Joker
        let joker = JokerState::new()
            .with_id(Joker::Joker as u8)
            .with_added_sell_value(1)
            .with_edition(Edition::None as u8);

        assert_eq!(get_joker_cost(&joker, &gs), 2);
        assert_eq!(get_joker_sell_value(&joker, &gs), 2);

        // sly joker
        let sly = JokerState::new()
            .with_id(Joker::SlyJoker as u8)
            .with_added_sell_value(0)
            .with_edition(Edition::None as u8);

        assert_eq!(get_joker_cost(&sly, &gs), 3);
        assert_eq!(get_joker_sell_value(&sly, &gs), 1);

        // mad joker
        let mad = JokerState::new()
            .with_id(Joker::MadJoker as u8)
            .with_added_sell_value(0)
            .with_edition(Edition::None as u8);

        assert_eq!(get_joker_cost(&mad, &gs), 4);
        assert_eq!(get_joker_sell_value(&mad, &gs), 2);
    }

    #[test]
    fn test_joker_base_cost_with_edition_no_voucher() {
        let gs = create_game_state(Deck::Red);

        let j = JokerState::new()
            .with_id(Joker::FourFingers as u8)
            .with_added_sell_value(0)
            .with_edition(Edition::Foil as u8);

        assert_eq!(get_joker_cost(&j, &gs), 9);
        assert_eq!(get_joker_sell_value(&j, &gs), 4);

        let j = JokerState::new()
            .with_id(Joker::Joker as u8)
            .with_added_sell_value(0)
            .with_edition(Edition::Polychrome as u8);

        assert_eq!(get_joker_cost(&j, &gs), 7);
        assert_eq!(get_joker_sell_value(&j, &gs), 3);

        let j = JokerState::new()
            .with_id(Joker::Joker as u8)
            .with_added_sell_value(1)
            .with_edition(Edition::Polychrome as u8);

        assert_eq!(get_joker_cost(&j, &gs), 7);
        assert_eq!(get_joker_sell_value(&j, &gs), 4);

        let j = JokerState::new()
            .with_id(Joker::SlyJoker as u8)
            .with_added_sell_value(0)
            .with_edition(Edition::Holographic as u8);

        assert_eq!(get_joker_cost(&j, &gs), 6);
        assert_eq!(get_joker_sell_value(&j, &gs), 3);

        let j = JokerState::new()
            .with_id(Joker::Superposition as u8)
            .with_added_sell_value(0)
            .with_edition(Edition::Holographic as u8);

        assert_eq!(get_joker_cost(&j, &gs), 7);
        assert_eq!(get_joker_sell_value(&j, &gs), 3);

        let j = JokerState::new()
            .with_id(Joker::Blueprint as u8)
            .with_added_sell_value(0)
            .with_edition(Edition::Negative as u8);

        assert_eq!(get_joker_cost(&j, &gs), 15);
        assert_eq!(get_joker_sell_value(&j, &gs), 7);
    }

    //TODO add tests for vouchers
}
