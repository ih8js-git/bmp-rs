use crate::blinds::Blind;
use crate::card::Card;
use crate::consumable::{
    Consumable, ConsumableState, Spectral, Tarot, create_spectral_consumable,
    create_tarot_consumable,
};
use crate::decks::{Deck, create_abandoned_deck, create_checkered_deck, create_default_deck};
use crate::joker::JokerState;
use crate::rng::{PrecomputedRngQueue, RNGQueueType, create_all_rng_queues, create_generator};
use crate::stakes::Stake;
use crate::{Voucher, add_voucher};
use strum::EnumCount;

#[derive(Debug)]
pub struct GameState {
    // Interactive
    pub balance: u32,
    pub vouchers: u32,
    pub hand: Vec<Card>,
    // These indices are based off of the hand vec
    pub selected_card_indices: [usize; 5],
    pub selected_card_count: usize,
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

    // Game Progression
    pub next_blind: Blind,
    pub ante: u8,

    // Card related
    pub deck: Vec<Card>,
    pub hand_size: u8,

    // In blind
    pub hands: u8,
    pub hands_used: u8,
    pub discards: u8,
    pub discards_used: u8,
    pub required_score: f64,
    pub current_score: f64,

    // Shop
    pub base_reroll_cost: u8,

    // Meta
    pub stake: Stake,
    pub starting_deck_size: u8,

    // RNG
    pub rng_queues: [PrecomputedRngQueue; RNGQueueType::COUNT],
    pub rng_next_queue_indices: [u32; RNGQueueType::COUNT], // keeps track of the next idx we need to pull from for every queue
}

pub fn create_game_state(deck: Deck) -> GameState {
    let seed = 123456789;
    let mut rng = create_generator(seed);

    let base = GameState {
        last_used: Consumable::Tarot(Tarot::Fool),
        tarots_used: 0,
        deck: Vec::new(),
        vouchers: 0,
        hand: Vec::with_capacity(8),
        selected_card_indices: [0, 0, 0, 0, 0],
        selected_card_count: 0,
        hand_size: 8,
        jokers: Vec::with_capacity(5),
        joker_slots: 5,
        consumables: Vec::with_capacity(2),
        consumable_slots: 2,
        stake: Stake::White,
        balance: 4,
        hands: 4,
        hands_used: 0,
        discards: 3,
        discards_used: 0,
        required_score: 0.0,
        current_score: 0.0,
        ante: 0,
        next_blind: Blind::Small,
        starting_deck_size: 52,
        skips_taken: 0,
        base_reroll_cost: 5,
        planet_levels: [0; 12],
        hand_types_played: [0; 12],

        rng_queues: create_all_rng_queues(&mut rng),
        rng_next_queue_indices: [0; RNGQueueType::COUNT], // next idx we need to pull from for each queue
        ecto_hand_size_reduction: 1,
    };

    match deck {
        Deck::Red => GameState {
            deck: create_default_deck(),
            discards: base.discards + 1,
            ..base
        },
        Deck::Blue => GameState {
            deck: create_default_deck(),
            hands: base.hands + 1,
            ..base
        },
        Deck::Yellow => GameState {
            deck: create_default_deck(),
            balance: base.balance + 10,
            ..base
        },
        Deck::Black => GameState {
            deck: create_default_deck(),
            joker_slots: base.joker_slots + 1,
            hands: base.hands - 1,
            ..base
        },
        Deck::Magic => {
            let mut state = GameState {
                deck: create_default_deck(),
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
                deck: create_default_deck(),
                consumable_slots: base.consumable_slots - 1,
                ..base
            };
            add_voucher(&mut state, Voucher::Telescope);
            state
        }
        Deck::Ghost => GameState {
            deck: create_default_deck(),
            consumables: vec![create_spectral_consumable(Spectral::Hex)],
            ..base
        },
        Deck::Abandoned => GameState {
            deck: create_abandoned_deck(),
            starting_deck_size: 40,
            ..base
        },
        Deck::Checkered => GameState {
            deck: create_checkered_deck(),
            ..base
        },
        Deck::Zodiac => {
            let mut state = GameState {
                deck: create_default_deck(),
                ..base
            };
            add_voucher(&mut state, Voucher::TarotMerchant);
            add_voucher(&mut state, Voucher::PlanetMerchant);
            add_voucher(&mut state, Voucher::Overstock);
            state
        }
        Deck::Painted => GameState {
            deck: create_default_deck(),
            hand_size: base.hand_size + 2,
            joker_slots: base.joker_slots - 1,
            ..base
        },
        _ => GameState {
            deck: create_default_deck(),
            ..base
        },
    }
}
