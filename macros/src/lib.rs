#![crate_name = "unix_ts_macros"]
#![doc = include_str!(concat!(env!("OUT_DIR"), "/README-rustdocified.md"))]

extern crate proc_macro;

use proc_macro::TokenStream;

/// Create a timestamp from the given Unix timestamp.
///
/// # Examples
///
/// ```
/// use unix_ts_macros::ts;
///
/// let t = ts!(1335020400);
/// assert_eq!(t.seconds(), 1335020400);
///
/// let t = ts!(1335020400.25);
/// assert_eq!(t.seconds(), 1335020400);
/// assert_eq!(t.subsec(3), 250);
///
/// let t = ts!(-86400);
/// assert_eq!(t.seconds(), -86400);
/// ```
#[proc_macro]
pub fn ts(input: TokenStream) -> TokenStream {
  let mut src = input.to_string().trim_start().trim_end().to_owned();
  if src.len() == 0 {
    panic!("No input to ts! macro.");
  }

  // If we have a sign bit, deal with it.
  let neg = src.starts_with('-');
  src = src.trim_start_matches('-').trim_start().to_owned();

  // If there is no decimal point, this is an integer;
  // return a timestamp from it.
  if !src.contains('.') {
    return format!(
      "::unix_ts::Timestamp::new({}{}, 0)",
      if neg { '-' } else { ' ' },
      src
    )
    .parse()
    .unwrap();
  }

  // If we start with a decimal point, prepend a zero.
  if src.starts_with('.') {
    src = format!("0{}", src);
  }

  // Split into two strings for whole seconds and nanos and return the
  // appropriate Timestamp.
  let src: Vec<&str> = src.split('.').collect();
  if src.len() > 2 {
    panic!("Unrecognized input to ts! macro.");
  }
  let mut seconds = src[0].parse::<i64>().unwrap();
  let mut nanos = src[1].to_owned();
  while nanos.len() < 9 {
    nanos += "0";
  }

  // If nanos is anything other than zero, we actually need to decrement
  // the seconds by one. This is because the nanos is always positive;
  // otherwise representing -0.5 seconds would be impossible.
  //
  // Note: This counter-intuitively means *adding* one here because we are
  // tracking our sign bit separately.
  if neg && nanos != "000000000" {
    seconds += 1;
  }

  // Return the new timestamp.
  return format!(
    "::unix_ts::Timestamp::new({}{}, {})",
    if neg { '-' } else { ' ' },
    seconds,
    nanos[0..9].to_string(),
  )
  .parse()
  .unwrap();
}
