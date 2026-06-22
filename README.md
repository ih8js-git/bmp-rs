# bmp-rs

**bmp-rs** (Balatro Multiplayer - Rust) is a Rust implementation of the game engine for [Balatro](https://www.playbalatro.com/) and it's [Multiplayer Mod](https://balatromp.com/).

It models cards, decks, jokers, consumables (tarots, planets, spectrals), scoring, vouchers, and shop logic.

## Status

Most core systems are implemented — card representation, hand detection, scoring pipeline, deck types, joker definitions, consumables, vouchers, and joker pricing. Joker scoring functions are a work in progress (5 of 150 implemented).

## Building & Testing

```sh
cargo build       # Build the project
cargo test        # Run all tests
```

## Documentation

See the [mdBook docs](https://ih8js-git.github.io/bmp-rs/) for a full overview.

## Git Hooks

This project uses [husky-rs](https://github.com/pplmx/husky-rs) to manage Git hooks automatically.

- **Pre-commit**: runs `cargo fmt` on staged Rust files
- **Pre-push**: runs `cargo check` and `cargo test`

Hooks install automatically on `cargo build` or `cargo test`. To skip (e.g., in CI), set `NO_HUSKY_HOOKS=1`.

## License

[GPL-3.0](https://www.gnu.org/licenses/gpl-3.0.en.html)



