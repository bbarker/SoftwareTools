
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

## Functional facilities

Higher-order-functions (HOFs) are frequently used to reduce code
complexity, verbosity, and the risk of errors. Primary examples are
`map`, `for_each` (like `map` but effectful), and `fold`.

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

### Using todo!() to

Using `todo!()` from `std::todo` is a helpful way to incrementally
develop a feature while still getting feedback from the
compiler. [**TODO**: show example]

A [caveat](https://github.com/rust-lang/rfcs/issues/3045) is that
currently you need code in the function after the `todo!()`, even
if it doesn't match the type. For instance, we can use a function
like:

```rust
pub fn some_num() -> i32 {
    todo!(); ();
}
```

Most beneficial is that `rustc` will warn you if you a `todo!()` is
left in your code, since it would result in a panic if that execution
path were to occur.


### Optimizing for size

* https://github.com/johnthagen/min-sized-rust