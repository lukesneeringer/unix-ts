# unix-ts: Convertible Unix timestamps for Rust

[![ci](https://github.com/lukesneeringer/unix-ts/actions/workflows/ci.yaml/badge.svg)](https://github.com/lukesneeringer/unix-ts/actions/workflows/ci.yaml)
[![codecov](https://codecov.io/gh/lukesneeringer/unix-ts/branch/main/graph/badge.svg?token=fDZ23KbbUo)](https://codecov.io/gh/lukesneeringer/unix-ts)
[![release](https://img.shields.io/crates/v/unix-ts.svg)](https://crates.io/crates/unix-ts)
[![docs](https://img.shields.io/badge/docs-release-blue)](https://docs.rs/unix-ts/)

unix-ts is a library for dealing with timestamps. It supports lightweight creation and manipulation
of timestamps.

The goal is to serve as a glue library that can take a timestamp and convert to whatever other
formats are needed.

## Usage

Add the crate to your `Cargo.toml` file like usual:

```toml
[dependencies]
unix-ts = "1"
```

You can create a timestamp with the `ts!` macro, which takes the Unix timestamp as an argument:

```rs
use unix_ts::ts;

// The argument is the number of seconds since the Unix epoch.
let t = ts!(1335020400);

// Fractional seconds are also allowed.
let t2 = ts!(1335020400.25);
```

For whole-second timestamps, you can also use the `from` method:

```rs
use unix_ts::Timestamp;

let t = Timestamp::from(1335020400);
```

For milliseconds, microseconds, or nanoseconds, there are specific `from` methods available:

```rs
use unix_ts::Timestamp;

let t = Timestamp::from_nanos(1335020400_000_000_000i64);
```

Finally, the `new` method accepts `seconds` and `nanos`. This is generally less convenient than the
macro, though, because you have to convert fractional seconds to nanos by yourself.

### Reading timestamps

There are three methods available for reading timestamps:

- `seconds() -> i64`: Returns the whole seconds value of the timestamp.
- `at_precision(e) -> i128`: Returns the timestamp as an integer at greater precision than the
  second. The `e` value represents the power of 10; therefore, `at_precision(3)` would return the
  value in milliseconds.
- `subsec(e) -> u32`: Returns the subsecond value at the given precision. The `e` value represents
  the power of 10; therefore, `subsec(3)` would return the sub-second value in milliseconds.

### Converting timestamps

Timestamps can currently be converted into integers (with the loss of the subsecond), or
`std::time::Duration`. This is done by implementing the Rust `From` trait (so you can use the
`from` or `into` methods).
