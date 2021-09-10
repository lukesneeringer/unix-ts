# unix-ts-macros: A macro to quickly generate unix-ts timestamps.

unix-ts-macros simplifies the creation of timestamps into a procedural macro:
`ts`.

## Usage

Add the create as well as unix-ts to your `Cargo.toml` file like usual:

```toml
[dependencies]
unix-ts = "0.1"
unix-ts-macros = "0.1"
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
