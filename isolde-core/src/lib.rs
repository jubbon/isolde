//! # Isolde Core
//!
//! Core library for Isolde v2 - ISOLated Development Environment template system.

#![warn(missing_docs)]
#![warn(clippy::all)]

pub mod error;
pub mod config;
pub mod template;
pub mod marketplace;
pub mod generator;

// Re-export common types
pub use error::{Error, Result};

/// Isolde core library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }
}
