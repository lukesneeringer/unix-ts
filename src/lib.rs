#![crate_name = "unix_ts"]
#![doc = include_str!(concat!(env!("OUT_DIR"), "/README-rustdocified.md"))]

mod integers;
mod std_duration;
mod timestamp;

#[cfg(feature = "chrono")]
mod chrono;

pub use timestamp::Timestamp;
pub use unix_ts_macros::ts;
