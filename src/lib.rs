//! `unix-ts` provides lightweight representations of Unix timestamps.
//!
//! Unix timestamps are one of the most common ways to exchange time data. A Unix timestamp is
//! simply the number of seconds (and, optionally, fractions of a second) that have elapsed since
//! January 1, 1970 at midnight UTC.

use std::time::SystemTime;

mod display;
mod integers;
mod std_duration;

pub use unix_ts_macros::ts;

/// A representation of a timestamp (seconds and nanos since the Unix epoch).
///
/// Timestamps are able to be easily converted into chrono DateTimes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Timestamp {
  /// The number of seconds since the Unix epoch.
  pub(crate) seconds: i64,

  /// The number of nanoseconds since second.
  pub(crate) nanos: u32,
}

/// Creation of timestamps.
impl Timestamp {
  /// Create a new timestamp from the given number of `seconds` and `nanos` (nanoseconds).
  ///
  /// The use of the `ts!()` macro in the `unix-ts-macros` crate is advised in lieu of calling this
  /// method directly for most situations.
  ///
  /// **Note:** For negative timestamps, the `nanos` argument is _always_ a positive offset.
  /// Therefore, the correct way to represent a timestamp of `-0.25 seconds` is to call `new(-1,
  /// 750_000_000)`.
  pub const fn new(mut seconds: i64, mut nanos: u32) -> Self {
    while nanos >= 1_000_000_000 {
      seconds += 1;
      nanos -= 1_000_000_000;
    }
    Timestamp { seconds, nanos }
  }

  /// Create a timestamp from the given number of nanoseconds.
  pub const fn from_nanos(nanos: i128) -> Self {
    let seconds: i64 = (nanos / 1_000_000_000) as i64;
    // .try_into()
    // .expect("Timestamp value out of range.");
    let nanos = if seconds >= 0 {
      (nanos % 1_000_000_000) as u32
    }
    else {
      (1_000_000_000 - (nanos % 1_000_000_000).abs()) as u32
    };
    Timestamp { seconds, nanos }
  }

  /// Create a timestamp from the given number of microseconds.
  pub const fn from_micros(micros: i64) -> Self {
    Timestamp::from_nanos(micros as i128 * 1_000)
  }

  /// Create a timestamp from the given number of milliseconds.
  pub const fn from_millis(millis: i64) -> Self {
    Timestamp::from_nanos(millis as i128 * 1_000_000)
  }

  /// The timestamp representing "right now".
  ///
  /// ## Panic
  ///
  /// Panics if the system clock is set to a time prior to the Unix epoch (January 1, 1970).
  pub fn now() -> Self {
    let now_dur = SystemTime::now()
      .duration_since(SystemTime::UNIX_EPOCH)
      .expect("System clock set prior to January 1, 1970");
    Self { seconds: now_dur.as_secs() as i64, nanos: now_dur.subsec_nanos() }
  }
}

/// Inspection of timestamps.
impl Timestamp {
  /// Return the seconds since the Unix epoch.
  /// Sub-second values are discarded.
  ///
  /// # Examples
  ///
  /// ```
  /// # use unix_ts::Timestamp;
  /// let t = Timestamp::from(1335020400);
  /// assert_eq!(t.seconds(), 1335020400);
  /// ```
  pub const fn seconds(&self) -> i64 {
    self.seconds
  }

  /// Return the time since the Unix epoch, as an integer, with the given
  /// precision.
  ///
  /// ## Arguments
  ///
  /// - `e` (`u8`) - The precision for the returned integer, as a power of 10. (ex. 3 for
  ///   milliseconds, 6 for microseconds, etc.). Must be a value between 0 and 9.
  ///
  /// ## Examples
  ///
  /// ```
  /// # use unix_ts::Timestamp;
  /// let t = Timestamp::from(1335020400);
  /// assert_eq!(t.at_precision(3), 1335020400_000);
  /// assert_eq!(t.at_precision(6), 1335020400_000_000);
  /// ```
  pub const fn at_precision(&self, e: u8) -> i128 {
    (self.seconds as i128) * 10i128.pow(e as u32)
      + (self.nanos as i128) / 10i128.pow(9 - (e as u32))
  }

  /// Return the subsecond component at the specified precision (ex. 3 for milliseconds, 6 for
  /// microseconds); max precision is 9.
  ///
  /// # Arguments
  ///
  /// - `e` (`u8`) - The precision for the returned subsecond value, as a power of 10 (ex. 3 for
  ///   milliseconds, 6 for microseconds, etc.). Must be a value between 0 and 9.
  ///
  /// # Examples
  ///
  /// ```
  /// # use unix_ts::Timestamp;
  /// let t = Timestamp::new(1335020400, 500_000_000);
  /// assert_eq!(t.subsec(1), 5);
  /// assert_eq!(t.subsec(3), 500);
  /// ```
  pub const fn subsec(&self, e: u8) -> u32 {
    self.nanos / 10u32.pow(9 - e as u32)
  }
}

#[cfg(test)]
#[allow(clippy::inconsistent_digit_grouping)]
mod tests {
  use std::time::Duration;

  use assert2::check;

  use super::*;

  #[test]
  fn test_cmp() {
    check!(Timestamp::from(1335020400) < Timestamp::from(1335024000));
    check!(Timestamp::from(1335020400) == Timestamp::from(1335020400));
    check!(Timestamp::new(1335020400, 500_000_000) < Timestamp::new(1335020400, 750_000_000));
    check!(Timestamp::new(1, 999_999_999) < Timestamp::from(2));
  }

  #[test]
  fn test_from_nanos() {
    check!(Timestamp::from_nanos(1335020400_000_000_000i128) == Timestamp::new(1335020400, 0));
    check!(
      Timestamp::from_nanos(1335020400_500_000_000i128) == Timestamp::new(1335020400, 500_000_000)
    );
    check!(Timestamp::from_nanos(-1_750_000_000) == Timestamp::new(-1, 250_000_000));
  }

  #[test]
  fn test_from_micros() {
    check!(Timestamp::from_micros(1335020400_000_000i64) == Timestamp::new(1335020400, 0));
    check!(
      Timestamp::from_micros(1335020400_500_000i64) == Timestamp::new(1335020400, 500_000_000)
    );
    check!(Timestamp::from_micros(-1_750_000) == Timestamp::new(-1, 250_000_000));
  }

  #[test]
  fn test_from_millis() {
    check!(Timestamp::from_millis(1335020400_000i64) == Timestamp::new(1335020400, 0));
    check!(Timestamp::from_millis(1335020400_500i64) == Timestamp::new(1335020400, 500_000_000));
    check!(Timestamp::from_millis(-1_750) == Timestamp::new(-1, 250_000_000));
  }

  #[test]
  fn test_seconds() {
    assert_eq!(Timestamp::from(1335020400).seconds, 1335020400);
  }

  #[test]
  fn test_at_precision() {
    let ts = Timestamp::new(1335020400, 123456789);
    assert_eq!(ts.at_precision(3), 1335020400123);
    assert_eq!(ts.at_precision(6), 1335020400123456);
    assert_eq!(ts.at_precision(9), 1335020400123456789);
  }

  #[test]
  fn test_subsec() {
    let ts = Timestamp::new(1335020400, 123456789);
    assert_eq!(ts.subsec(3), 123);
    assert_eq!(ts.subsec(6), 123456);
    assert_eq!(ts.subsec(9), 123456789);
  }

  #[test]
  fn test_add() {
    let ts = Timestamp::from(1335020400) + Duration::new(86400, 1_000_000);
    assert_eq!(ts.seconds(), 1335020400 + 86400);
    assert_eq!(ts.subsec(3), 1);
  }

  #[test]
  fn test_sub() {
    let ts = Timestamp::from(1335020400) - Duration::new(86400, 0);
    assert_eq!(ts.seconds(), 1335020400 - 86400);
    assert_eq!(ts.nanos, 0);
  }

  #[test]
  fn test_sub_nano_overflow() {
    let ts = Timestamp::from(1335020400) - Duration::new(0, 500_000_000);
    assert_eq!(ts.seconds(), 1335020399);
    assert_eq!(ts.subsec(1), 5);
  }
}
