//! This library provides enumeration of [`ISO-4217`].
//!
//! # Example
//! ```rust
//! use std::convert::TryFrom;
//! use iso_4217::*;
//!
//! let usd: CurrencyCode = TryFrom::try_from("USD").unwrap();
//! assert_eq!(usd, CurrencyCode::USD);
//!
//! assert_eq!(usd.alpha(), "USD");
//! assert_eq!(usd.num(), 840);
//! assert_eq!(usd.digit(), Some(2));
//! assert_eq!(usd.name(), "United States dollar");
//! ```
//!
//! [`ISO-4217`]: https://en.wikipedia.org/wiki/ISO_4217

mod error;
mod impls;
mod macros;

pub use error::ParseCodeError;
pub use impls::CurrencyCode;
