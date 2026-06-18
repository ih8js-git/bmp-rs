# bmp-rs

## Git Hooks (husky-rs)

This project uses [husky-rs](https://github.com/pplmx/husky-rs) to manage Git hooks automatically.

### Pre-commit hook

Runs `cargo fmt` to format all staged Rust files before each commit.

### Pre-push hook

Runs `cargo check` and `cargo test` before each push.

### Setup

Hooks install automatically when you run `cargo build` or `cargo test`. Just commit and push normally:

```sh
git commit -m "message"   # runs cargo fmt on staged files
git push                  # runs cargo check + cargo test first
```

To skip hook installation (e.g., in CI), set `NO_HUSKY_HOOKS=1`.