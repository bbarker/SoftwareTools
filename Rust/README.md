
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