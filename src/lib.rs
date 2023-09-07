//! # 🚀 Getting started
//!

/// Module containing the [`client::Client`] struct.
pub mod client;

/// Module containing the [`error::Error`] struct.
pub mod error;

// TODO: Might be worth abandoning in favor of pure GraphQL
// pub mod v1;
// pub mod v2;

// mod utils;

mod types;

pub mod graphql;

pub use cynic;
