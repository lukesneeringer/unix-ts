use std::convert::TryInto;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Sub;
use std::ops::SubAssign;

use crate::Timestamp;

impl Add<std::time::Duration> for Timestamp {
  type Output = Self;

  /// Add the provided duration to the timestamp.
  fn add(self, other: std::time::Duration) -> Timestamp {
    let s: i64 = other.as_secs() as i64;
    Timestamp::new(self.seconds + s, self.nanos + other.subsec_nanos())
  }
}

impl AddAssign<std::time::Duration> for Timestamp {
  /// Add the provided duration to the timestamp, in-place.
  fn add_assign(&mut self, other: std::time::Duration) {
    let delta: i64 = other.as_secs() as i64;
    self.seconds += delta;
    self.nanos += other.subsec_nanos();
    while self.nanos >= 1_000_000_000 {
      self.seconds += 1;
      self.nanos -= 1_000_000_000;
    }
  }
}

impl Sub<std::time::Duration> for Timestamp {
  type Output = Self;

  /// Subtract the provided duration from the timestamp.
  fn sub(self, other: std::time::Duration) -> Timestamp {
    let other_sec: i64 = other.as_secs().try_into().unwrap();
    if other.subsec_nanos() > self.nanos {
      return Timestamp::new(
        self.seconds - other_sec - 1,
        self.nanos + 1_000_000_000 - other.subsec_nanos(),
      );
    }
    Timestamp::new(self.seconds - other_sec, self.nanos - other.subsec_nanos())
  }
}

impl SubAssign<std::time::Duration> for Timestamp {
  /// Subtract the provided duration to the timestamp, in-place.
  fn sub_assign(&mut self, other: std::time::Duration) {
    let delta: i64 = other.as_secs().try_into().unwrap();
    self.seconds -= delta;
    if other.subsec_nanos() > self.nanos {
      self.seconds -= 1;
      self.nanos += 1_000_000_000;
    }
    self.nanos -= other.subsec_nanos();
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_add() {
    let ts = Timestamp::new(1335020400, 0);
    let dur = std::time::Duration::new(86400, 0);
    assert_eq!(ts + dur, Timestamp::new(1335020400 + 86400, 0));
  }

  #[test]
  fn test_add_assign() {
    let mut ts = Timestamp::new(1335020400, 0);
    ts += std::time::Duration::new(86400, 0);
    assert_eq!(ts, Timestamp::new(1335020400 + 86400, 0));
  }

  #[test]
  fn test_add_assign_nano_overflow() {
    let mut ts = Timestamp::new(1335020400, 500_000_000);
    ts += std::time::Duration::new(0, 750_000_000);
    assert_eq!(ts.seconds, 1335020401);
    assert_eq!(ts.nanos, 250_000_000);
  }

  #[test]
  fn test_sub() {
    let ts = Timestamp::new(1335020400, 0);
    let dur = std::time::Duration::new(86400, 0);
    assert_eq!(ts - dur, Timestamp::new(1335020400 - 86400, 0));
  }

  #[test]
  fn test_sub_nano_overflow() {
    let ts = Timestamp::new(1335020400, 500_000_000) - std::time::Duration::new(0, 750_000_000);
    assert_eq!(ts.seconds, 1335020399);
    assert_eq!(ts.nanos, 750_000_000);
  }

  #[test]
  fn test_sub_assign() {
    let mut ts = Timestamp::new(1335020400, 0);
    ts -= std::time::Duration::new(86400, 0);
    assert_eq!(ts.seconds, 1335020400 - 86400);
    assert_eq!(ts.nanos, 0);
  }

  #[test]
  fn test_sub_assign_nano_overflow() {
    let mut ts = Timestamp::new(1335020400, 500_000_000);
    ts -= std::time::Duration::new(0, 750_000_000);
    assert_eq!(ts.seconds, 1335020399);
    assert_eq!(ts.nanos, 750_000_000);
  }
}
