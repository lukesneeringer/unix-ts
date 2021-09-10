use std::ops::Add;
use std::ops::Sub;

/// A representation of a timestamp (seconds and nanos since the Unix epoch).
///
/// Timestamps are able to be easily converted into chrono DateTimes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Timestamp {
  /// The number of seconds since the Unix epoch.
  pub(crate) seconds: i64,

  /// The number of nanoseconds since the Unix epoch.
  pub(crate) nanos: u32,
}

impl Timestamp {
  /// Create a new timestamp from the given number of `seconds` and `nanos`
  /// (nanoseconds).
  ///
  /// The use of the `ts!()` macro in the `unix-ts-macros` crate is advised
  /// in lieu of calling this method directly for most situations.
  ///
  /// Note: For negative timestamps, the `nanos` argument is _always_ a
  /// positive offset. Therefore, the correct way to represent a timestamp
  /// of `-0.25 seconds` is to call `new(-1, 750_000_000)`.
  pub fn new(mut seconds: i64, mut nanos: u32) -> Timestamp {
    while nanos >= 1_000_000_000 {
      seconds += 1;
      nanos -= 1_000_000_000;
    }
    Timestamp { seconds: seconds, nanos: nanos }
  }

  /// Return the seconds since the Unix epoch.
  /// Sub-second values are discarded.
  ///
  /// # Examples
  ///
  /// ```
  /// use unix_ts::Timestamp;
  ///
  /// let t = Timestamp::from(1335020400);
  /// assert_eq!(t.seconds(), 1335020400);
  /// ```
  pub fn seconds(&self) -> i64 {
    self.seconds
  }

  /// Return the time since the Unix epoch, as an integer, with the given
  /// precision.
  ///
  /// # Arguments
  ///
  /// - `e` (`u8`) - The precision for the returned integer, as a power of 10.
  ///   (ex. 3 for milliseconds, 6 for microseconds, etc.). Must be a value
  ///   between 0 and 9.
  ///
  /// # Examples
  ///
  /// ```
  /// use unix_ts::Timestamp;
  ///
  /// let t = Timestamp::from(1335020400);
  /// assert_eq!(t.at_precision(3), 1335020400_000);
  /// assert_eq!(t.at_precision(6), 1335020400_000_000);
  /// ```
  pub fn at_precision(&self, e: u8) -> i128 {
    i128::from(self.seconds) * 10i128.pow(e.into())
      + i128::from(self.nanos) / 10i128.pow(9 - u32::from(e))
  }

  /// Return the subsecond component at the specified precision
  /// (ex. 3 for milliseconds, 6 for microseconds); max precision is 9.
  ///
  /// # Arguments
  ///
  /// - `e` (`u8`) - The precision for the returned subsecond value, as a power
  ///   of 10 (ex. 3 for milliseconds, 6 for microseconds, etc.). Must be a
  ///   value between 0 and 9.
  ///
  /// # Examples
  ///
  /// ```
  /// use unix_ts::Timestamp;
  ///
  /// let t = Timestamp::new(1335020400, 500_000_000);
  /// assert_eq!(t.subsec(1), 5);
  /// assert_eq!(t.subsec(3), 500);
  /// ```
  pub fn subsec(&self, e: u8) -> u32 {
    self.nanos / 10u32.pow(9 - u32::from(e))
  }
}

impl Add for Timestamp {
  type Output = Self;

  /// Add two timestamps to one another and return the result.
  fn add(self, other: Timestamp) -> Timestamp {
    Timestamp::new(self.seconds + other.seconds, self.nanos + other.nanos)
  }
}

impl Sub for Timestamp {
  type Output = Self;

  /// Subtract the provided timestamp from this one and return the result.
  fn sub(self, other: Timestamp) -> Timestamp {
    if other.nanos > self.nanos {
      return Timestamp::new(
        self.seconds - other.seconds - 1,
        self.nanos + 1_000_000_000 - other.nanos,
      );
    }
    Timestamp::new(self.seconds - other.seconds, self.nanos - other.nanos)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_cmp() {
    assert!(Timestamp::from(1335020400) < Timestamp::from(1335024000));
    assert!(Timestamp::from(1335020400) == Timestamp::from(1335020400));
    assert!(
      Timestamp::new(1335020400, 500_000_000)
        < Timestamp::new(1335020400, 750_000_000)
    );
    assert!(Timestamp::new(1, 999_999_999) < Timestamp::from(2));
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
    let ts = Timestamp::from(1335020400) + Timestamp::new(86400, 1_000_000);
    assert_eq!(ts.seconds(), 1335020400 + 86400);
    assert_eq!(ts.subsec(3), 1);
  }

  #[test]
  fn test_sub() {
    let ts = Timestamp::from(1335020400) - Timestamp::new(86400, 0);
    assert_eq!(ts.seconds(), 1335020400 - 86400);
    assert_eq!(ts.nanos, 0);
  }

  #[test]
  fn test_sub_nano_overflow() {
    let ts = Timestamp::from(1335020400) - Timestamp::new(0, 500_000_000);
    assert_eq!(ts.seconds(), 1335020399);
    assert_eq!(ts.subsec(1), 5);
  }
}
