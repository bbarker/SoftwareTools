
# Design

In the spirit of Software Tools, the aim is to make components re-usable
in three ways:

 1. Implement core features as functions, so they can be re-used within Rust.
    These functions should generally return a `Result` type, so the caller
    can decide how to deal with the error.
 2. Executable commands with a simple interface that typically act as thin
    wrappers around the library functions, or perhaps combine the library
    functions in interesting ways.
 3. As well designed code that can be copied as repurposed when necessary.

A fourth avenue may be explored, which is to adopt the
[nushell](https://github.com/rjbs/Sweater) approach to transfering
tabular data between commands.

For a related project that also follows Software Tools in Rust, and
may serve as an interesting comparison, see
[Sweater](https://github.com/rjbs/Sweater).

## Build

Currently, do generate small builds the following commands
are required.

1. (only once per environment) Make source code for the standard library available:

```
rustup component add rust-src --toolchain nightly
```

2.

```
cargo +nightly build -Z build-std --target x86_64-unknown-linux-gnu --release
```

3. (optional) `strip` binary - see links in notes

## Misc Notes

### Optimizing for size

* https://github.com/johnthagen/min-sized-rust