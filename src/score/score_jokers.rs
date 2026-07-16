use crate::card::Edition;
use crate::joker::JokerState;
use crate::joker::fn_arrays::joker_score::SCORE_FNS;
use crate::levels::Hand;

pub fn score_jokers(jokers: &[JokerState], hand: Hand, chips: &mut f64, mult: &mut f64) -> () {
    for joker in jokers {
        // Apply Edition bonuses
        let edition = joker.edition();
        if edition == Edition::Foil as u8 {
            *chips += 50.0;
        } else if edition == Edition::Holographic as u8 {
            *mult += 10.0;
        } else if edition == Edition::Polychrome as u8 {
            *mult *= 1.5;
        }

        let id = joker.id() as usize;
        let def = crate::joker::core::JOKER_DEFS[id];

        if def.trigger_time == crate::joker::core::ScoringTriggerTime::PostHand {
            let score_fn = SCORE_FNS[id];
            score_fn(joker, hand, chips, mult).unwrap();
        }
    }
}
