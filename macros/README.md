# unix-ts-macros: A macro to quickly generate unix-ts timestamps.

unix-ts-macros simplifies the creation of timestamps into a procedural macro:
`ts`. This is an implementation crate for `unix-ts`, which is what you should
actually add as a dependency.

## Usage

Add the `unix-ts` crate to your `Cargo.toml` file like usual:

```toml
[dependencies]
unix-ts = "0.2"
```

You can create a timestamp with the `ts!` macro, which takes the Unix timestamp
as an argument:

```
use unix_ts_macros::ts;

// The argument is the number of seconds since the Unix epoch.
let t = ts!(1335020400);

// Fractional seconds are also allowed.
let t2 = ts!(1335020400.25);
```
