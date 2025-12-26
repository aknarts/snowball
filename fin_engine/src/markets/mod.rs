//! Market-specific implementations
//!
//! Each submodule implements the `MarketProfile` trait for a specific country.

#[cfg(feature = "czech")]
pub mod czech;

#[cfg(feature = "usa")]
pub mod usa;

#[cfg(feature = "uk")]
pub mod uk;
