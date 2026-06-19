# bmp-rs — Balatro Multiplayer (Rust)

**bmp-rs** is a Rust implementation of the game engine for [Balatro](https://www.playbalatro.com/) and its [Multiplayer Mod](https://balatromp.com/). It models cards, decks, jokers, consumables, scoring, vouchers, and shop logic — designed as a reusable engine, not a playable game.

---

## Project Structure

```
src/
├── main.rs                 # Entry point — demo of tarot card usage
├── game.rs                 # GameState struct + create_game_state(deck)
├── decks.rs                # Deck enum + factory functions
├── levels.rs               # Hand enum + base chips/mult per level
├── vouchers.rs             # Voucher enum + bitfield helpers
├── card/                   # Card representation, bitfield packing, parsing
│   ├── mod.rs
│   ├── core.rs             # Card, Rank, Suit, Edition, Enhancement, Seal
│   ├── operations.rs       # Bitfield getters/setters
│   └── parse.rs            # Card-to-text conversion
├── consumable/             # Tarots, Planets, Spectrals
│   ├── mod.rs
│   ├── core.rs             # Consumable enum + ConsumableState
│   ├── parse.rs            # Consumable-to-text
│   ├── tarots.rs           # 22 Tarot types + use_tarot()
│   ├── planets.rs          # 12 Planet types + use_planet()
│   └── spectrals.rs        # 18 Spectral types + seal helpers
├── joker/                  # Joker definitions
│   ├── mod.rs
│   └── jokers.rs           # Joker enum (150), JokerDef, JokerState, JOKER_DEFS
├── score/                  # Scoring pipeline
│   ├── mod.rs
│   ├── core.rs             # get_score(), hand type detection
│   ├── score_played_cards.rs  # Per-card scoring
│   ├── score_held_cards.rs    # (stub)
│   ├── score_jokers.rs     # PostHand joker scoring
│   └── jokers/
│       ├── mod.rs          # Dispatch tables (RETRIGGER_FNS, CARD_SCORE_FNS, SCORE_FNS)
│       └── *.rs            # 150 individual joker files
├── tables/                 # Cost tables
│   ├── mod.rs
│   ├── joker_cost.rs       # Joker pricing with edition/voucher discounts
│   └── input/base_cost.txt # 150 joker base costs
├── shop/                   # Shop system (stub)
│   ├── mod.rs
│   ├── core.rs             # Shop struct, ShopItem
│   └── booster_pack.rs     # (stub)
├── helper/                 # Name parsing helpers
│   ├── mod.rs
│   └── name_parsing.rs     # Balatro source string → Joker enum
└── rng/
    └── mod.rs              # (stub)
```

---

## Key Modules

### `card` — Card Representation

Cards are packed into a **16-bit bitfield** (`Card.meta`) for efficiency:

| Field       | Bits | Range |
| ----------- | ---- | ----- |
| Rank        | 4    | 0–12  |
| Suit        | 2    | 0–3   |
| Edition     | 3    | 0–4   |
| Enhancement | 4    | 0–8   |
| Seal        | 3    | 0–4   |

- `Card.chips` stores base chip value (2–11 per rank).
- `Enhancement`: Bonus, Mult, Wild, Glass, Steel, Stone, Gold, Lucky
- `Edition`: Foil (+50 chips), Holographic (+10 mult), Polychrome (×1.5 mult), Negative
- `Seal`: Gold, Red (double trigger), Blue, Purple

### `consumable` — Tarots, Planets, Spectrals

- **Tarots** (22): `use_tarot()` dispatches to functions like `use_strength` (rank up), `use_hanged_man` (destroy), `use_death` (copy). Enhancement/suit-changing tarots implemented; some are stubs.
- **Planets** (12): `use_planet()` increments the corresponding hand's planet level.
- **Spectrals** (18): Seal-applying spectrals use bitfield helpers. Some are stubs.

### `joker` — Joker Definitions

- `Joker` enum: 150 variants covering all Balatro jokers.
- `JokerDef` bitfield: rarity, trigger time, base price, flags.
- `JokerState` bitfield: id, edition, rental/perishable/eternal/pinned, scale, added_sell_value.
- Trigger times: `PreHand`, `CardScored`, `CardHeld`, `PostHand`, `Other`.

### `score` — Scoring Engine

The scoring pipeline in `get_score()`:

1. Determine hand type (Flush Five → High Card)
2. Score played cards (per-card chips, enhancements, editions, retriggers)
3. Score jokers (PostHand trigger time, edition bonuses)

**Hand type detection** handles:

- Flush (Smeared Joker, Wild cards)
- Straight (Shortcut Joker, Ace-low)
- Four Fingers (4-card hands)
- All standard poker hands

**Joker dispatch tables** in `score/jokers/mod.rs`:

- `RETRIGGER_FNS[150]` — only Sock & Buskin implemented
- `CARD_SCORE_FNS[150]` — only Greedy Joker implemented
- `SCORE_FNS[150]` — all 150 wired, but only ~5 implemented

### `decks` — Starting Decks

13 deck types (Red, Blue, Yellow, Green, Black, Magic, Nebula, Ghost, Abandoned, Checkered, Zodiac, Painted, Anaglyph, Plasma, Erratic) with per-deck modifiers (extra discard, extra hand, consumable slots, etc.).

### `levels` — Hand Levels

12 hand types ranked by value. `hand_base_chips_and_mult(level, hand)` returns base chips and mult with level scaling.

### `tables` — Shop Pricing

Joker costs computed from `base_cost.txt`: base cost + edition upcharge, discounted by Clearance Sale / Liquidation vouchers. Sell value = max(1, added_sell_value + cost / 2).

---

## Implementation Status

| Component            | Status              |
| -------------------- | ------------------- |
| Card system          | Complete            |
| Decks                | Partial (stubs)     |
| Hand detection       | Partial             |
| Level scaling        | Complete            |
| Vouchers             | Not implemented     |
| Tarot consumables    | Partial (stubs)     |
| Planet consumables   | Complete            |
| Spectral consumables | Partial (stubs)     |
| Joker definitions    | Complete (150)      |
| Joker pricing        | Complete            |
| Joker scoring        | 5 / 150 implemented |
| Shop                 | Stub                |
| RNG                  | Stub                |
| Held card scoring    | Stub                |

---

## Building & Testing

```sh
cargo build       # Build the project
cargo test        # Run all tests
cargo fmt         # Format code (pre-commit hook)
cargo check       # Check compilation (pre-push hook)
```

Git hooks (via husky-rs) run `cargo fmt` on commit and `cargo check + cargo test` on push. Skip with `NO_HUSKY_HOOKS=1`.

---

## Dependencies

- `modular-bitfield` — bitfield packing for cards, joker state
- `strum` + `strum_macros` — enum iteration and display derives
