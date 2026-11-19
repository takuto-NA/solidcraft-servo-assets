//! Data-driven servo reference assets. Geometry is made only through host commands.

#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::todo,
    clippy::unimplemented,
    clippy::unreachable
)]

pub mod cables;
pub mod catalog;
pub mod horns;
mod insert;
pub mod manifest;
pub mod models;
pub mod outputs;
pub mod workflow;

#[cfg(target_arch = "wasm32")]
mod guest;

pub use insert::{InsertParams, Runner, insert, recipe};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    Invalid { field: String, reason: String },
    Unsupported(String),
    Host { command: String, reason: String },
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Invalid { field, reason } => write!(f, "{field}: {reason}"),
            Self::Unsupported(reason) => f.write_str(reason),
            Self::Host { command, reason } => write!(f, "{command}: {reason}"),
        }
    }
}

impl std::error::Error for Error {}

pub(crate) fn invalid(field: &str, reason: impl ToString) -> Error {
    Error::Invalid {
        field: field.into(),
        reason: reason.to_string(),
    }
}

#[cfg(test)]
mod tests;
