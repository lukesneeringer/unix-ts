use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Rem;
use std::ops::Sub;
use std::ops::SubAssign;

use crate::Timestamp;

trait Int {}
impl Int for f64 {}
impl Int for f32 {}
impl Int for i64 {}
impl Int for i32 {}
impl Int for i16 {}
impl Int for i8 {}
impl Int for isize {}
impl Int for u32 {}
impl Int for u16 {}
impl Int for u8 {}
impl Int for usize {}

impl<T: Into<i64> + Int> From<T> for Timestamp {
  /// Create a new timestamp for the given number of seconds.
  fn from(secs: T) -> Self {
    Timestamp { seconds: secs.into(), nanos: 0 }
  }
}

impl<T: Into<i64> + Int> Add<T> for Timestamp {
  type Output = Self;

  /// Add the provided duration to the timestamp.
  fn add(self, other: T) -> Timestamp {
    Timestamp::new(self.seconds + other.into(), self.nanos)
  }
}

impl<T: Into<i64> + Int> AddAssign<T> for Timestamp {
  /// Add the provided duration to the timestamp, in-place.
  fn add_assign(&mut self, other: T) {
    self.seconds += other.into();
  }
}

impl<T: Into<i64> + Int> Sub<T> for Timestamp {
  type Output = Self;

  /// Subtract the provided duration to the timestamp.
  fn sub(self, other: T) -> Timestamp {
    Timestamp::new(self.seconds - other.into(), self.nanos)
  }
}

impl<T: Into<i64> + Int> SubAssign<T> for Timestamp {
  /// Subtract the provided duration to the timestamp, in-place.
  fn sub_assign(&mut self, other: T) {
    self.seconds -= other.into();
  }
}

impl<T: Into<i64> + Int> Rem<T> for Timestamp {
  type Output = Self;

  /// Subtract the provided duration to the timestamp.
  fn rem(self, other: T) -> Timestamp {
    Timestamp::new(self.seconds % other.into(), self.nanos)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_from() {
    let ts: Timestamp = 1335020400.into();
    assert_eq!(ts.seconds, 1335020400);
    assert_eq!(ts.nanos, 0);
  }

  #[test]
  fn test_add() {
    let ts = Timestamp::new(1335020400, 0) + 86400;
    assert_eq!(ts.seconds, 1335020400 + 86400);
    assert_eq!(ts.nanos, 0);
  }

  #[test]
  fn test_add_assign() {
    let mut ts = Timestamp::new(1335020400, 500_000_000);
    ts += 86400;
    assert_eq!(ts.seconds, 1335020400 + 86400);
    assert_eq!(ts.nanos, 500_000_000);
  }

  #[test]
  fn test_sub() {
    let ts = Timestamp::new(1335020400, 0) - 86400;
    assert_eq!(ts.seconds, 1335020400 - 86400);
    assert_eq!(ts.nanos, 0);
  }

  #[test]
  fn test_sub_assign() {
    let mut ts = Timestamp::new(1335020400, 500_000_000);
    ts -= 86400;
    assert_eq!(ts.seconds, 1335020400 - 86400);
    assert_eq!(ts.nanos, 500_000_000);
  }

  #[test]
  fn test_rem() {
    let ts = Timestamp::new(86500, 12);
    assert_eq!(ts % 86400, Timestamp::new(100, 12))
  }
}
