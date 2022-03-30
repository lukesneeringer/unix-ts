//! # unix-ts: Convertible Unix timestamps for Rust
//!
//! unix-ts is a library for dealing with timestamps. It supports lightweight
//! creation of timestamps, and conversions into and out of other formats, from
//! integers to [chrono][] DateTime objects.
//!
//! The goal is to serve as a glue library that can take a timestamp and
//! convert to whatever other formats are needed.
//!
//! ## Usage
//!
//! Add the create to your `Cargo.toml` file like usual:
//!
//! ```toml
//! [dependencies]
//! unix-ts = "0.1"
//! ```
//!
//! Note: The use of the `unix-ts-macros` crate is recommended also.
//!
//!
//! For whole-second timestamps, you can also use the `from` method:
//!
//! ```
//! use unix_ts::Timestamp;
//!
//! let t = Timestamp::from(1335020400);
//! ```
//!
//! Finally, the `new` method accepts `seconds` and `nanos`. This is generally
//! less convenient than the macro, though, because you have to convert
//! fractional seconds to nanos by yourself.
//!
//! ### Reading timestamps
//!
//! There are three methods available for reading timestamps:
//!
//! - `seconds() -> i64`: Returns the whole seconds value of the timestamp.
//! - `at_precision(e) -> i128`: Returns the timestamp as an integer at greater
//!   precision than the second. The `e` value represents the power of 10;
//!   therefore, `at_precision(3)` would return the value in milliseconds.
//! - `subsec(e) -> u32`: Returns the subsecond value at the given precision.
//!   The `e` value represents the power of 10; therefore, `subsec(3)` would
//!   return the sub-second value in milliseconds.
//!
//! ### Converting timestamps
//!
//! Timestamps can currently be converted into integers (with the loss of the
//! subsecond), or `std::time::Duration`. This is done by implementing the Rust
//! `From` trait (so you can use the `from` or `into` methods).
//!
//! If the `chrono` feature is enabled, unix-ts also supports converting to
//! `chrono::DateTime` and `chrono::NaiveDateTime`. This is done through the
//! `to_datetime` and `to_naive_datetime` methods. (A `to_utc_datetime` is also
//! offered to simplify time zone specification for this common case.)
//!
//! ## Features
//!
//! All dependencies outside the standard library are optional, meaning that
//! unix-ts will not force you to also install, for example, [chrono][]
//! (although there is a good chance you should if you are dealing with time).
//!
//! Optional features:
//!
//!
//! - `chrono`: Adds converstion functions to [chrono][] `DateTime` and
//!   `NaiveDateTime`.
//!
//! [chrono]: https://crates.io/crates/chrono

#![crate_name = "unix_ts"]

mod integers;
mod std_duration;
mod timestamp;

#[cfg(feature = "chrono")]
mod chrono;

pub use timestamp::Timestamp;
pub use unix_ts_macros::ts;
