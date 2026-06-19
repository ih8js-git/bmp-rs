use crate::GameState;
use crate::card::Edition;
use crate::helper::parse_source_joker_to_enum;
use crate::joker::JokerState;
use crate::vouchers::{Voucher, has_voucher};
use std::cmp::max;
use std::io::BufRead;
use std::sync::OnceLock;
use strum::{EnumCount, IntoEnumIterator};

pub const JOKER_AMOUNT: usize = 150;

pub const DISCOUNT_VOUCHER_MULTIPLIER: [f32; 3] = [1.0, 0.75, 0.5];

pub type CostTable = [[u8; Edition::COUNT]; DISCOUNT_VOUCHER_MULTIPLIER.len()];

// for every joker, a table containing the cost for every voucher (row) & edition (col) combo
pub static JOKER_COST_TABLES: OnceLock<[CostTable; JOKER_AMOUNT]> = OnceLock::new();
pub fn init_joker_base_cost_table() {
    let mut temp_tables =
        [[[0u8; Edition::COUNT]; DISCOUNT_VOUCHER_MULTIPLIER.len()]; JOKER_AMOUNT];
    let mut base_cost = [0u8; JOKER_AMOUNT]; // indexed by joker enum

    // insert imported joker base cost data into array
    JOKER_BASE_COST_RAW
        .lines()
        .filter(|line| !line.is_empty())
        .filter_map(|line| {
            let mut parts = line.split(':');

            let name = parts.next()?;
            let joker = parse_source_joker_to_enum(name.trim());

            let cost_str = parts.next()?;
            let cost = cost_str.trim().parse::<u8>().ok()?;

            Some((joker, cost))
        })
        .for_each(|(joker, cost)| {
            base_cost[joker as usize] = cost;
        });

    for (joker_index, &base_cost) in base_cost.iter().enumerate() {
        for (discount_index, discount_multiplier) in DISCOUNT_VOUCHER_MULTIPLIER.iter().enumerate()
        {
            for edition in Edition::iter() {
                let f_base_cost = base_cost as f32;
                let f_edition_added_cost = edition.added_cost() as f32;
                let f_res = (f_base_cost + f_edition_added_cost + 0.5) * discount_multiplier;

                temp_tables[joker_index][discount_index][edition as usize] = f_res.floor() as u8;
            }
        }
    }

    if JOKER_COST_TABLES.set(temp_tables).is_err() {
        panic!("JOKER_COST_TABLES was already initialized");
    }
}

pub fn get_joker_cost(j: &JokerState, gs: &GameState) -> u8 {
    if j.is_rental() {
        return 1;
    }

    if JOKER_COST_TABLES.get().is_none() {
        // If it's not, run your setup function right here!
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
    use crate::GameState;
    use crate::card::Edition;
    use crate::consumable::Consumable;
    use crate::consumable::Tarot::Hermit;
    use crate::joker::{Joker, JokerState};
    use crate::tables::{get_joker_cost, get_joker_sell_value};

    const BASE_GS: GameState = create_game_state(Deck::Red);

    #[test]
    fn test_joker_base_cost_no_edition_no_voucher_no_added_cost() {
        let gs = GameState { ..BASE_GS };

        let sly = JokerState::new()
            .with_id(Joker::SlyJoker as u8)
            .with_added_sell_value(0)
            .with_edition(Edition::None as u8);

        assert_eq!(get_joker_cost(&sly, &gs), 3);
        assert_eq!(get_joker_sell_value(&sly, &gs), 1);
    }

    //TODO add more unit tests
}
