//! # i18n library for Rust
//!
//! This library provides a simple way to add internationalization to your Rust applications by using JSON files.
mod error;
mod lingua;

pub mod prelude {
    pub use crate::error::LinguaError;
    pub use crate::lingua::Lingua;
}
