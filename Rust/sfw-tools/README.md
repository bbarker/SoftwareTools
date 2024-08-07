# sfw-tools

## Design

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
[nushell](https://github.com/rjbs/Sweater) approach to transferring
tabular data between commands.

For a related project that also follows Software Tools in Rust, and
may serve as an interesting comparison, see
[Sweater](https://github.com/rjbs/Sweater).
A more feature-rich project is [uutils coreutils](https://github.com/uutils/coreutils),
which as the name suggests, is a Rust implementation analogous to
GNU Coreutils.

### Functional facilities

Higher-order-functions (HOFs) are frequently used to reduce code
complexity, verbosity, and the risk of errors. Primary examples are
`map`, `for_each` (like `map` but effectful), and `fold`. As pointed
out in Software Tools, pp 21, *"The best programs are designed in
terms of loosely coupled functions that each does a simple task."*

Some other references that refelect functional programming values:
- page 36, a discussion on `break`: the suggestions also coincide largley
  with recursive functions.
- pages 44-45 discuss defensive programming by guarding control variables
  with safety checks. In functional programming, such control variables
  often do not appear, so safety checks are unnecessary due to the usage
  of HOFs being safe by design. Page 45 also points out that non-voluminous
  code listings are easier to debug (which I agree with, and a functional
  style typically enables this), though we also want to warn against making
  code overly terse. Experience is the best guide in this case.

### Currently Implemented Tools
- [x] `cp`
- [x] `wc`
- [x] `detab`
- [x] `entab`
- [x] `echo`
- [x] `compress`
- [ ] `expand`

### Dependencies

Since the goal is to make the software both as self-contained and
as illustrative as possible, we've tried to rely on very few dependencies.
The following exceptions exist:

- [fp-core](https://docs.rs/fp-core)
  This is what one would typically find as part the standard library
  in a functional language, so we have included it here. Though Rust is functional
  in a sense — it has lambda functions (i.e. Rust closures) and the stand library
  has many higher-order functions (HOFs) — its standard library doesn't include
  traits that are commonly found to be helpful abstracts in functional languages.
  We will use a few of these where it is particularly illustrative or sensible,
  but will stick with idiomatic Rust where that is obviously simpler.
  An interesting note is that filters are the subject of chapter 2 and much of
  the rest of the book, which are just a particular class of HOFs.
- [peeking_take_while](https://docs.rs/peeking_take_while/)
  A small library that provides the `peeking_take_while` function for
  `Peekable` iterators. This behaves more of how would would expect for
  a `take_while` function compared to the standard `take_while` implementation,
  which will "lose" the first element after a `take_while` streak ends.
- [tailcall](https://docs.rs/tailcall)
  This is a macro that enables tailcall elimination for functions that are
  tail recursive. In other words, instead of writing loops, we can sometimes
  just write a function that calls itself. Without this macro, such functions
  would eventually cause the stack to blow up.
- [seahorse](https://docs.rs/seahorse)
  Seahorse is a minimal argument parser. Judging by some results
  returned by Google, [clap](https://clap.rs) is far more popular, but
  has additional dependencies; we are striving for being as portable
  as possible, so the minimality seemed to line up with that
  goal. Additionally, Clap doesn't appear to allow passing in argument
  lists directly, which is useful for maintaining separate commands
  that build on each other. In any case, argument parsing is only used
  very late in the application logic, and most of the API could be used
  without worrying about it.

#### Currently unused

- [byteorder](docs.rs/byteorder) Library for reading/writing numbers
  in big-endian and little-endian. This is a somewhat low-level library,
  but as this is an IO-heavy library of tools, it may make sense to rely
  on it.
- [im](https://docs.rs/im)
  Immutable data structures that implement structural sharing can be
  even more performant than `std`'s mutable structures for large
  data types, and while Rust makes mutation far safer than most languages,
  mutation can still result in confusion at times, so in the cases where
  clarity is more important than performance (or performance doesn't
  matter much, e.g. one-ops), it may be preferable to use immutable data
  structures.


### Build

### Misc Notes

#### Using todo!() to

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

#### Rust on nix

```plain
nix develop
```

#### Optimizing for size

* https://github.com/johnthagen/min-sized-rust

Currently, to generate small builds the following commands
are required.

1. (only once per environment) Make source code for the standard library available:

```plain
rustup component add rust-src --toolchain nightly
```

2.

```plain
cargo +nightly build -Z build-std --target x86_64-unknown-linux-gnu --release
```

3. (optional) `strip` binary - see links in notes


### Project administration

#### Git hooks

##### Cargo-Husky

We use [cargo-husky](https://github.com/rhysd/cargo-husky) to keep in
line; it enforces several checks with a `pre-push` hook. Sometimes it
is a bit restrictive, so if we need to push
in-progress work to a branch, we can use
`git push --no-verify -u origin feature_branch`.
Cargo-husky expects certain files to be at the root of the repository,
thus the symlinks.

##### pre-commit

We include the following, less stringent checks for pre-commit.

```bash
#!/bin/sh

# Put in your Rust repository's .git/hooks/pre-commit to ensure you never
# breaks rustfmt.
#
# WARNING: rustfmt is a fast moving target so ensure you have the version that
#          all contributors have.

for FILE in `git diff --cached --name-only`; do
    if [[ -f "$FILE" ]] && [[ $FILE == *.rs ]] \
           && ! rustup run nightly rustfmt --unstable-features \
                --skip-children $FILE; then
        echo "Commit rejected due to invalid formatting of \"$FILE\" file."
        exit 1
    fi
done

cd Rust/sfw-tools && cargo readme > README.md && git add README.md
```
As can be seen this also generates the README from doc comments in `lib.rs`.


License: MPL-2.0
