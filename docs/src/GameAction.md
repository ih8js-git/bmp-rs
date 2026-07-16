# Game Actions Reference

This document outlines all possible actions a player can take throughout a game run, represented by the `GameAction` enum.

---

## Global Actions
*Actions that can generally be performed across multiple game phases, such as managing your active Jokers or Consumables.*

### `MoveJoker`
Moves a Joker from one slot to another to reorder them. This is crucial for optimizing scoring triggers.
* **`from_idx`** (`u8`): The current position index of the Joker.
* **`to_idx`** (`u8`): The target position index of the Joker after reordering.

### `SellJoker`
Sells a Joker currently in the player's possession for gold.
* **`idx`** (`u8`): The slot index of the Joker being sold.

### `SellConsumable`
Sells a held consumable item (e.g., Tarot, Planet, or Spectral card) for gold.
* **`idx`** (`u16`): The slot index of the consumable being sold.

### `UseSimpleConsumable`
Activates a consumable that does not require any targets (e.g., a Planet card that immediately upgrades a hand type).
* **`idx`** (`u16`): The slot index of the consumable to use.

### `UseConsumableWithTargets`
Activates a consumable that targets specific cards in the hand or deck (e.g., a Tarot card that enhances or transforms up to 3 cards).
* **`idx`** (`u16`): The slot index of the consumable to use.
* **`amount`** (`u8`): The number of cards targeted. Elements in the `cards` array beyond this threshold are ignored.
* **`cards`** (`[u16; 3]`): An array containing the static unique deck indices of the targeted cards.

---

## Blind Selection Phase
*Actions available on the blind selection screen before entering a round.*

### `SkipBlind`
Skips the currently selected Blind to immediately receive the associated Skip Tag reward.

### `PlayBlind`
Enters the selected Blind round to begin playing hands.

---

## In-Blind Phase (Gameplay)
*Actions available while actively fighting a Blind.*

### `PlayHand`
Plays a poker hand scoring attempt using the selected cards.
* **`card_indices`** (`[u16; 5]`): An array of static unique deck indices representing the cards being played (**not** their relative hand positions).
* **`amount`** (`u8`): The actual number of cards being played. Elements in the array beyond this index threshold are ignored.

### `DiscardHand`
Discards the selected cards from the hand to draw new ones, consuming one discard charge.
* **`card_indices`** (`[u16; 5]`): An array of static unique deck indices representing the cards to be discarded.
* **`amount`** (`u8`): The actual number of cards being discarded. Elements in the array beyond this index threshold are ignored.

### `MoveCard`
Reorders the cards currently held in the player's hand.
* **`from_idx`** (`u16`): The current hand position index of the card.
* **`to_idx`** (`u16`): The target hand position index of the card after reordering.

---

## Cashout Phase
*Actions available during the scoring and reward summary after beating a Blind.*

### `Cashout`
Collects the round rewards, interest, and remaining hand bonuses, transitioning the player to the Shop phase.

---

## Shop Phase
*Actions available while browsing the store between rounds.*

### `BuyFromShop`
Buys a regular item (Joker or Consumable) currently displayed in the main shop slots that reset when you roll.
* **`idx`** (`u8`): The shop slot index of the item.

### `BuyAndUse`
Buys a consumable card from the main shop slots and immediately activates its effect without storing it in a consumable slot.
* **`idx`** (`u8`): The shop slot index of the consumable item.

### `BuyVoucher`
Buys the permanent tier voucher available for the current Ante.
* **`idx`** (`u8`): The voucher slot index.

### `BuyBoosterPack`
Buys a Booster Pack (Tarot, Planet, Standard, Buffoon, or Spectral), immediately opening it and transitioning the player into the Booster Pack phase.
* **`idx`** (`u8`): The shop slot index of the Booster Pack.

### `Reroll`
Spends gold to refresh the items currently displayed in the main shop slots.

### `GoNext`
Leaves the shop and transitions back to the Blind Selection phase.

---

## Booster Pack Phase
*Actions available exclusively while inside an opened Booster Pack.*

### `SelectFromPack`
Chooses an item out of the revealed booster pack contents to keep or use.
* **`idx`** (`u8`): The item index inside the opened booster pack.

### `SkipPack`
Closes the remaining booster pack contents without picking anything else, transitioning the player back to the Shop phase.