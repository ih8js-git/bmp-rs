# BMP Replay Format (`.bmpr`)

The `.bmpr` file format is a highly compressed, binary serialization of a complete game run. Because all engine actions are driven by deterministic 64-bit opcodes (as defined in the [ISA](./ISA.md)), we can recreate an entire playthrough flawlessly by seeding the engine and feeding it the logged instruction array.

## Overview

- **Endianness:** Little-Endian (Standard for Rust primitives).
- **Compression:** Gzip / Zlib (Fast, widely supported, and drastically reduces the already small footprint).
- **Integrity:** CRC32 Checksum appended to the end to prevent corruption or tampering.

---

## File Structure

The uncompressed binary payload is split into three main sections: **Header**, **Instruction Payload**, and **Checksum**.

### 1. Header (Metadata)

The header contains all the initialization metadata required to spin up the exact starting state of the game.

| Offset | Type      | Size    | Description                                                                                                                  |
| ------ | --------- | ------- | ---------------------------------------------------------------------------------------------------------------------------- |
| `0x00` | `[u8; 4]` | 4 bytes | **Magic Number**: `BMPR` (ASCII `0x42 0x4D 0x50 0x52`)                                                                       |
| `0x04` | `u8`      | 1 byte  | **Engine Version**: e.g., `100` for `v1.0.0` (critical for preventing desyncs if RNG logic changes in later engine patches). |
| `0x05` | `u64`     | 8 bytes | **Seed**: 64-bit unsigned integer representing the game seed.                                                                |
| `0x0D` | `u8`      | 1 byte  | **Deck ID**: The unique ID of the starting deck.                                                                             |
| `0x0E` | `u8`      | 1 byte  | **Stake ID**: The difficulty or stake ID.                                                                                    |
| `0x0F` | `u8`      | 1 byte  | **Challenge ID**: The ID for the challenge deck (optional).                                                                  |
| `0x10` | `u64`     | 8 bytes | **Player ID**: The player's unique Discord ID.                                                                               |
| `0x18` | `u64`     | 8 bytes | **Timestamp**: Unix timestamp (in seconds) of when the run was played.                                                       |
| `0x20` | `u32`     | 4 bytes | **Action Count**: The total number ($N$) of 64-bit instructions that follow.                                                 |

### 2. Instruction Payload

Immediately following the header is a flat array of the player's actions, serialized exactly as they were executed.

| Offset           | Type  | Size    | Description                     |
| ---------------- | ----- | ------- | ------------------------------- |
| `0x24`           | `u64` | 8 bytes | The 1st instruction executed.   |
| `0x2C`           | `u64` | 8 bytes | The 2nd instruction executed.   |
| ...              | ...   | ...     | ...                             |
| `0x24 + (N-1)*8` | `u64` | 8 bytes | The final instruction executed. |

### 3. Checksum (Integrity)

To ensure the replay file hasn't been corrupted or manually hex-edited to spoof an invalid high score.

| Offset | Type  | Size    | Description                                                                              |
| ------ | ----- | ------- | ---------------------------------------------------------------------------------------- |
| `End`  | `u32` | 4 bytes | **CRC32 Checksum** computed over the entire uncompressed Header and Instruction Payload. |

---

## Processing Pipeline

When saving a `.bmpr` log to disk, the engine should follow this standard pipeline:

1. **Serialize**: Pack the Header variables and the raw `u64` instruction array into a flat byte buffer using Little-Endian byte order.
2. **Checksum**: Calculate the CRC32 of the resulting uncompressed buffer and append those 4 bytes to the end.
3. **Compress**: Feed the entire buffer through a standard Gzip/Zlib encoder.
4. **Write**: Save the final compressed bytes to disk as `[run_name].bmpr`.

When loading a replay, the steps are perfectly reversed: **Decompress -> Verify CRC32 -> Parse Header -> Feed Instructions to Game Loop**.
