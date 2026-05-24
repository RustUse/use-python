# Releasing

`use-python` is currently experimental and below `0.3.0`.

Run the local verification path before preparing a release:

```sh
cargo fmt --all --check
cargo check --workspace --all-features --examples
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
```

Publish focused crates before the facade crate so crates.io can resolve exact sibling versions during package verification.
