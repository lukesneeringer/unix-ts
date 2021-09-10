extern crate chrono;

use chrono::DateTime;
use chrono::NaiveDateTime;
use chrono::TimeZone;
use chrono::Utc;

use crate::timestamp::Timestamp;

impl Timestamp {
  /// Convert the given timestamp to a DateTime in the given time zone.
  pub fn to_datetime<Tz: TimeZone>(&self, tz: &Tz) -> DateTime<Tz> {
    self.to_utc_datetime().with_timezone(tz)
  }

  /// Convert the given timestamp into a DateTime in UTC.
  pub fn to_utc_datetime(&self) -> DateTime<Utc> {
    DateTime::from_utc(self.to_naive_datetime(), Utc)
  }

  /// Convert the given timestamp into a NaiveDateTime.
  pub fn to_naive_datetime(&self) -> NaiveDateTime {
    NaiveDateTime::from_timestamp(self.seconds, self.nanos)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use chrono::NaiveDate;
  use chrono::Timelike;
  use chrono_tz::America::New_York;
  use chrono_tz::Australia::Sydney;

  #[test]
  fn test_to_naive_datetime() {
    let t = Timestamp::from(1335020400);
    assert_eq!(
      t.to_naive_datetime(),
      NaiveDate::from_ymd(2012, 4, 21).and_hms(15, 00, 0)
    );
  }

  #[test]
  fn test_to_datetime() {
    let t = Timestamp::from(1335020400);
    assert_eq!(
      t.to_datetime(&New_York),
      New_York
        .from_local_datetime(
          &NaiveDate::from_ymd(2012, 4, 21).and_hms(11, 0, 0)
        )
        .unwrap(),
    );
    assert_eq!(
      t.to_datetime(&Sydney),
      Sydney
        .from_local_datetime(
          &NaiveDate::from_ymd(2012, 4, 22).and_hms(1, 0, 0)
        )
        .unwrap(),
    );
  }

  #[test]
  fn test_to_utc_datetime() {
    let t = Timestamp::from(1335020400);
    assert_eq!(
      t.to_utc_datetime(),
      Utc
        .from_local_datetime(
          &NaiveDate::from_ymd(2012, 4, 21).and_hms(15, 0, 0),
        )
        .unwrap(),
    );
  }

  #[test]
  fn test_nanos() {
    let t = Timestamp::new(1335020400, 500_000_000);
    assert_eq!(t.to_utc_datetime().time().nanosecond(), 500_000_000);
    assert_eq!(t.to_utc_datetime().timestamp_subsec_millis(), 500);
  }

  #[test]
  fn test_nanos_overflow() {
    let t = Timestamp::new(1335020400, 1_500_000_000);
    assert_eq!(t.seconds, 1335020401);
    assert_eq!(t.nanos, 500_000_000);
  }
}
