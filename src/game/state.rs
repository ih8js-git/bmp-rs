use crate::blinds::Blind;
use crate::card::Card;
use crate::consumable::{
    Consumable, ConsumableState, Spectral, Tarot, create_spectral_consumable,
    create_tarot_consumable,
};
use crate::decks::{Deck, create_abandoned_deck, create_checkered_deck, create_default_deck};
use crate::game::action::GameAction;
use crate::game::delta::GameDelta;
use crate::joker::JokerState;
use crate::rng::core::{PrecomputedRngQueue, create_generator};
use crate::rng::queues::{RNGQueueType, create_all_rng_queues};
use crate::stakes::Stake;
use crate::{Voucher, add_voucher};
use smallvec::SmallVec;
use strum::EnumCount;

#[derive(Debug)]
pub struct GameState {
    // Interactive
    pub balance: u32,

    // Card related
    pub hand_size: u8,
    pub cards: SmallVec<[Card; 104]>,
    pub draw_pile: SmallVec<[u16; 104]>,
    pub hand: SmallVec<[u16; 16]>,

    pub jokers: Vec<JokerState>,
    pub joker_slots: u8,
    pub consumables: Vec<ConsumableState>,
    pub consumable_slots: u8,

    // History
    pub last_used: Consumable,
    pub tarots_used: u16,
    pub ecto_hand_size_reduction: u8,
    pub skips_taken: u8,
    pub planet_levels: [u8; 12],
    pub hand_types_played: [u8; 12],
    pub vouchers: u32,

    // Game Progression
    pub next_blind: Blind,
    pub ante: u8,

    // In blind
    pub base_hands: u8,
    pub hands_remaining: u8,
    pub base_discards: u8,
    pub discards_remaining: u8,
    pub required_score: f64,
    pub current_score: f64,

    // Shop
    pub base_reroll_cost: u8,

    // Meta
    pub stake: Stake,
    pub starting_deck_size: u8,

    // RNG
    pub rng_queues: [PrecomputedRngQueue; RNGQueueType::COUNT],
    pub rng_next_queue_indices: [u32; RNGQueueType::COUNT],
}

impl GameState {
    pub fn apply_action(&mut self, action: GameAction) {
        for delta in action.to_deltas(self) {
            self.apply_delta(delta);
        }
    }

    fn apply_delta(&mut self, delta: &GameDelta) {
        delta.apply(self);
    }

    fn revert_delta(&mut self, delta: &GameDelta) {
        delta.revert(self);
    }
}

fn init_deck(deck_fn: fn() -> Vec<Card>) -> (SmallVec<[Card; 104]>, SmallVec<[u16; 104]>, u8) {
    let d = deck_fn();
    let len = d.len();
    let cards: SmallVec<[Card; 104]> = d.into_iter().collect();
    let draw_pile: SmallVec<[u16; 104]> = (0..len as u16).collect();
    (cards, draw_pile, len as u8)
}

pub fn create_game_state(deck: Deck) -> GameState {
    let seed = 123456789;
    let mut rng = create_generator(seed);

    let (cards, draw_pile, starting_deck_size) = match deck {
        Deck::Abandoned => init_deck(create_abandoned_deck),
        Deck::Checkered => init_deck(create_checkered_deck),
        _ => init_deck(create_default_deck),
    };

    let base = GameState {
        balance: 4,
        cards,
        draw_pile,
        hand: SmallVec::new(),
        hand_size: 8,
        jokers: Vec::with_capacity(5),
        joker_slots: 5,
        consumables: Vec::with_capacity(2),
        consumable_slots: 2,
        last_used: Consumable::Tarot(Tarot::Fool),
        tarots_used: 0,
        ecto_hand_size_reduction: 1,
        skips_taken: 0,
        planet_levels: [0; 12],
        hand_types_played: [0; 12],
        vouchers: 0,
        next_blind: Blind::Small,
        ante: 0,
        base_hands: 4,
        hands_remaining: 0,
        base_discards: 3,
        discards_remaining: 0,
        required_score: 0.0,
        current_score: 0.0,
        base_reroll_cost: 5,
        stake: Stake::White,
        starting_deck_size,
        rng_queues: create_all_rng_queues(&mut rng),
        rng_next_queue_indices: [0; RNGQueueType::COUNT],
    };

    match deck {
        Deck::Red => GameState {
            base_discards: base.base_discards + 1,
            ..base
        },
        Deck::Blue => GameState {
            base_hands: base.base_hands + 1,
            ..base
        },
        Deck::Yellow => GameState {
            balance: base.balance + 10,
            ..base
        },
        Deck::Black => GameState {
            joker_slots: base.joker_slots + 1,
            base_hands: base.base_hands - 1,
            ..base
        },
        Deck::Magic => {
            let mut state = GameState {
                consumables: vec![
                    create_tarot_consumable(Tarot::Fool),
                    create_tarot_consumable(Tarot::Fool),
                ],
                ..base
            };
            add_voucher(&mut state, Voucher::CrystalBall);
            state
        }
        Deck::Nebula => {
            let mut state = GameState {
                consumable_slots: base.consumable_slots - 1,
                ..base
            };
            add_voucher(&mut state, Voucher::Telescope);
            state
        }
        Deck::Ghost => GameState {
            consumables: vec![create_spectral_consumable(Spectral::Hex)],
            ..base
        },
        Deck::Zodiac => {
            let mut state = GameState { ..base };
            add_voucher(&mut state, Voucher::TarotMerchant);
            add_voucher(&mut state, Voucher::PlanetMerchant);
            add_voucher(&mut state, Voucher::Overstock);
            state
        }
        Deck::Painted => GameState {
            hand_size: base.hand_size + 2,
            joker_slots: base.joker_slots - 1,
            ..base
        },
        _ => GameState { ..base },
    }
}
