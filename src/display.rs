use std::fmt::Display;

use crate::Timestamp;

impl Display for Timestamp {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match f.precision() {
      Some(p) => {
        let float = self.seconds as f64 + self.nanos as f64 / 1_000_000_000.0;
        write!(f, "{:.*}", p, float)
      },
      None => write!(f, "{}", self.seconds),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_display() {
    let t = Timestamp::from(1335020400);
    assert_eq!(format!("{:.02}", t), "1335020400.00");
    assert_eq!(format!("{}", t), "1335020400");
  }
}
