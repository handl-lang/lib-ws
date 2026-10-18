//! `ws` — Handl stdlib port contract.

/// Checkout root of this repo (build-time).
pub const ROOT: &str = env!("CARGO_MANIFEST_DIR");

/// `index.hnl` — the `ws` contract.
pub const WS_HNL: &str = include_str!("index.hnl");
