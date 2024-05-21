#![recursion_limit = "256"]
#![cfg_attr(not(feature = "std"), no_std)]
#![deny(
    unused,
    warnings,
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms,
    unused_imports
)]
#![forbid(unsafe_code)]

// Chips for practice problems
pub mod chips;

// Circuits built from chips from chips module
pub mod circuits;
