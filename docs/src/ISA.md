# Instruction Set Architecture (ISA)

Welcome to the ISA documentation. This defines the byte-level operations for the engine, detailing how actions are represented as opcodes and bitfields.

---

## Opcode Table

Every instruction begins with an **8-bit (1 byte)** opcode, followed by any necessary arguments packed into subsequent bits. Opcode families are grouped logically by the game phase.

| Hex  | Binary     | Context      | Instruction     | Description                                                      |
| ---- | ---------- | ------------ | --------------- | ---------------------------------------------------------------- |
| `00` | `00000000` | Global       | Use Consumable  | Uses a consumable from the Consumable Slots.                     |
| `01` | `00000001` | Global       | Sell Joker      | Sells a Joker from the Joker Slots.                              |
| `02` | `00000010` | Global       | Sell Consumable | Sells a Consumable from the Consumable Slots.                    |
| `03` | `00000011` | Global       | Reorder Jokers  | Move Joker X to Joker Slot index Y.                              |
| `10` | `00010000` | In Blind     | Play Hand       | Submits the selected cards to score.                             |
| `11` | `00010001` | In Blind     | Discard Hand    | Discards the selected cards.                                     |
| `12` | `00010010` | In Blind     | Reorder Cards   | Move Card X to Hand index Y.                                     |
| `20` | `00100000` | Cashout      | Cashout         | Collect Reward & Interest Money, then proceed to the shop phase. |
| `30` | `00110000` | In Shop      | Buy             | Purchases an item from the shop.                                 |
| `31` | `00110001` | In Shop      | Open Booster    | Opens a purchased booster pack.                                  |
| `32` | `00110010` | In Shop      | Reroll          | Spends money to reroll the shop's contents.                      |
| `33` | `00110011` | In Shop      | Go Next         | Proceeds to the selecting blind phase.                           |
| `34` | `00110100` | In Shop      | Buy and Use     | Purchases and immediately uses a consumable from the shop.       |
| `40` | `01000000` | Blind Select | Skip            | Skips the current blind (for a Tag).                             |
| `41` | `01000001` | Blind Select | Play            | Selects and enters the blind phase.                              |
| `50` | `01010000` | In Pack      | Take Card       | Selects a playing card from the pack.                            |
| `51` | `01010001` | In Pack      | Take Joker      | Selects a Joker from the pack.                                   |
| `52` | `01010010` | In Pack      | Use Consumable  | Uses a consumable directly from the pack.                        |
| `53` | `01010011` | In Pack      | Skip            | Skips the rest of the pack selection.                            |
| `54` | `01010100` | In Pack      | Reorder Cards   | Move Card X to Hand index Y.                                     |

> [!NOTE]
> Grouping opcodes by high-nibble allows for easy contextual validation (e.g., checking `opcode >> 4 == 0x01` instantly verifies if a command is a valid "In Blind" action).

---

## Bit Layouts per Opcode

### 0x00 - 0x03: Global Actions

#### `0x00` Use Consumable

| Field        | Size    | Description                                |
| ------------ | ------- | ------------------------------------------ |
| Opcode       | 8 bits  | `0x00`                                     |
| Slot Index   | 4 bits  | The slot of the consumable to use.         |
| Target Count | 2 bits  | Number of cards targeted in hand (0 to 3). |
| Card 1 Index | 10 bits | Index of the 1st targeted card (optional). |
| Card 2 Index | 10 bits | Index of the 2nd targeted card (optional). |
| Card 3 Index | 10 bits | Index of the 3rd targeted card (optional). |

> [!NOTE]
> Consumables like Tarot cards often require selecting cards in your hand (e.g. Suit-changing Tarots allow up to 3 targets). The `Target Count` and subsequent `Card X Index` fields accommodate this.

#### `0x01` Sell Joker

| Field            | Size    | Description                   |
| ---------------- | ------- | ----------------------------- |
| Opcode           | 8 bits  | `0x01`                        |
| Joker Slot Index | 56 bits | The slot of the Joker to sell |

#### `0x02` Sell Consumable

| Field                 | Size    | Description                        |
| --------------------- | ------- | ---------------------------------- |
| Opcode                | 8 bits  | `0x02`                             |
| Consumable Slot Index | 56 bits | The slot of the consumable to sell |

#### `0x03` Reorder Jokers

This instruction supports reordering up to 8 Jokers at once. To keep the total payload size compact, the bit-width of the indices is dynamic and defined by the **Index Size** field.

| Field                 | Size                      | Description                                                   |
| --------------------- | ------------------------- | ------------------------------------------------------------- |
| Opcode                | 8 bits                    | `0x03`                                                        |
| Index Size            | 4 bits                    | Defines exactly how many bits each subsequent index will use. |
| 1st Joker (From & To) | `3` to `16` bits **each** | Original and destination index fields for the **1st** Joker.  |
| 2nd Joker (From & To) | `3` to `13` bits **each** | Original and destination index fields for the **2nd** Joker.  |
| 3rd Joker (From & To) | `3` to `8` bits **each**  | Original and destination index fields for the **3rd** Joker.  |
| 4th Joker (From & To) | `3` to `8` bits **each**  | Original and destination index fields for the **4th** Joker.  |
| 5th Joker (From & To) | `3` to `6` bits **each**  | Original and destination index fields for the **5th** Joker.  |
| 6th Joker (From & To) | `3` to `5` bits **each**  | Original and destination index fields for the **6th** Joker.  |
| 7th Joker (From & To) | `3` to `4` bits **each**  | Original and destination index fields for the **7th** Joker.  |
| 8th Joker (From & To) | `3` bits **each**         | Original and destination index fields for the **8th** Joker.  |

> [!NOTE]
> **Variable Bit Sizes Explained:**
> The **Index Size** dictates the fixed size that _every_ active index field will use for this specific instruction.
>
> The size is chosen to be the smallest bit-width mathematically possible while still allowing you to index every available slot. For example, a standard game starts with 5 Joker slots, so a minimum of 3 bits is required (providing 8 indices). As the player's capacity for slots expands throughout the game, this **Index Size** dynamically scales up to safely accommodate the highest possible index.

---

### 0x10 - 0x12: In Blind

#### `0x10` Play Hand / `0x11` Discard Hand

Both operations share the same argument structure: an array of selected cards.

| Field        | Size    | Description                               |
| ------------ | ------- | ----------------------------------------- |
| Opcode       | 8 bits  | `0x10` or `0x11`                          |
| Card Count   | 3 bits  | Number of cards selected (1 to 5).        |
| Card 1 Index | 10 bits | Index of the 1st card in hand (optional). |
| Card 2 Index | 10 bits | Index of the 2nd card in hand (optional). |
| Card 3 Index | 10 bits | Index of the 3rd card in hand (optional). |
| Card 4 Index | 10 bits | Index of the 4th card in hand (optional). |
| Card 5 Index | 10 bits | Index of the 5th card in hand (optional). |

#### `0x12` / `0x54` Reorder Cards

This instruction supports reordering up to 8 Cards at once. It is available both **In Blind (`0x12`)** and **In Pack (`0x54`)**. To keep the total payload size compact, the bit-width of the indices is dynamic and defined by the **Index Size** field.

| Field                | Size                      | Description                                                   |
| -------------------- | ------------------------- | ------------------------------------------------------------- |
| Opcode               | 8 bits                    | `0x12` or `0x54`                                              |
| Index Size           | 4 bits                    | Defines exactly how many bits each subsequent index will use. |
| 1st Card (From & To) | `3` to `16` bits **each** | Original and destination index fields for the **1st** Card.   |
| 2nd Card (From & To) | `3` to `13` bits **each** | Original and destination index fields for the **2nd** Card.   |
| 3rd Card (From & To) | `3` to `8` bits **each**  | Original and destination index fields for the **3rd** Card.   |
| 4th Card (From & To) | `3` to `8` bits **each**  | Original and destination index fields for the **4th** Card.   |
| 5th Card (From & To) | `3` to `6` bits **each**  | Original and destination index fields for the **5th** Card.   |
| 6th Card (From & To) | `3` to `5` bits **each**  | Original and destination index fields for the **6th** Card.   |
| 7th Card (From & To) | `3` bits **each**         | Original and destination index fields for the **7th** Card.   |
| 8th Card (From & To) | `3` bits **each**         | Original and destination index fields for the **8th** Card.   |

---

### 0x20: Cashout

#### `0x20` Cashout

Takes no additional arguments.

| Field  | Size   | Description |
| ------ | ------ | ----------- |
| Opcode | 8 bits | `0x20`      |

---

### 0x30 - 0x34: In Shop

#### `0x30` Buy

| Field      | Size    | Description                    |
| ---------- | ------- | ------------------------------ |
| Opcode     | 8 bits  | `0x30`                         |
| Slot Index | 56 bits | The shop slot to purchase from |

#### `0x31` Open Booster Pack

| Field      | Size   | Description                       |
| ---------- | ------ | --------------------------------- |
| Opcode     | 8 bits | `0x31`                            |
| Slot Index | 4 bits | The shop slot of the booster pack |

#### `0x32` Reroll

Takes no additional arguments.

| Field  | Size   | Description |
| ------ | ------ | ----------- |
| Opcode | 8 bits | `0x32`      |

#### `0x33` Go Next

Takes no additional arguments.

| Field  | Size   | Description |
| ------ | ------ | ----------- |
| Opcode | 8 bits | `0x33`      |

#### `0x34` Buy and Use

| Field        | Size    | Description                                |
| ------------ | ------- | ------------------------------------------ |
| Opcode       | 8 bits  | `0x34`                                     |
| Shop Index   | 24 bits | The shop slot to purchase from.            |
| Target Count | 2 bits  | Number of cards targeted in hand (0 to 3). |
| Card 1 Index | 10 bits | Index of the 1st targeted card (optional). |
| Card 2 Index | 10 bits | Index of the 2nd targeted card (optional). |
| Card 3 Index | 10 bits | Index of the 3rd targeted card (optional). |

---

### 0x40 - 0x41: Blind Select

#### `0x40` Skip / `0x41` Play

Takes no additional arguments.

| Field  | Size   | Description      |
| ------ | ------ | ---------------- |
| Opcode | 8 bits | `0x40` or `0x41` |

---

### 0x50 - 0x54: In Pack

> [!NOTE]
> Booster packs with `picks_remaining > 1` (e.g., Mega Packs) allow multiple back-to-back `Take` or `Use` commands before the pack automatically closes.

#### `0x50` Take Card / `0x51` Take Joker

| Field      | Size    | Description                        |
| ---------- | ------- | ---------------------------------- |
| Opcode     | 8 bits  | `0x50` or `0x51`                   |
| Pack Index | 56 bits | Index of the item inside the pack. |

#### `0x52` Use Consumable

| Field        | Size    | Description                                |
| ------------ | ------- | ------------------------------------------ |
| Opcode       | 8 bits  | `0x52`                                     |
| Pack Index   | 4 bits  | Index of the consumable in the pack.       |
| Target Count | 2 bits  | Number of cards targeted in hand (0 to 3). |
| Card 1 Index | 10 bits | Index of the 1st targeted card (optional). |
| Card 2 Index | 10 bits | Index of the 2nd targeted card (optional). |
| Card 3 Index | 10 bits | Index of the 3rd targeted card (optional). |

#### `0x53` Skip

Takes no additional arguments.

| Field  | Size   | Description |
| ------ | ------ | ----------- |
| Opcode | 8 bits | `0x53`      |

#### `0x54` Reorder Cards

See `0x12 Reorder Cards` in the **In Blind** section for the complete bit layout.

> [!NOTE]
> Reordering cards `In Pack` is only permitted when opening specific booster packs (e.g., Tarot or Spectral packs) where the player's hand is visible and interactable during the selection phase.
