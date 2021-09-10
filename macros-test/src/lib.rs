#[cfg(test)]
mod tests {
  use unix_ts::Timestamp;
  use unix_ts_macros::ts;

  #[test]
  fn integer() {
    assert_eq!(ts!(1335020400), Timestamp::new(1335020400, 0));
  }

  #[test]
  fn decimal() {
    assert_eq!(ts!(1335020400.50), Timestamp::new(1335020400, 500_000_000));
  }

  #[test]
  fn negative() {
    let t = ts!(-1000);
    assert_eq!(t.seconds(), -1000);
  }

  #[test]
  fn negative_with_nanos() {
    let t = ts!(-10000.25);
    assert_eq!(t.seconds(), -10001);
    assert_eq!(t.subsec(2), 25);
  }

  #[test]
  fn negative_no_zero() {
    let t = ts!(-.5);
    assert_eq!(t.seconds(), -1);
    assert_eq!(t.subsec(1), 5);
  }

  #[test]
  fn no_zero() {
    let t = ts!(.5);
    assert_eq!(t.seconds(), 0);
    assert_eq!(t.subsec(1), 5);
  }
}
