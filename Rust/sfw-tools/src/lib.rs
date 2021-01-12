//! # Design
//!
//! In the spirit of Software Tools, the aim is to make components re-usable
//! in three ways:
//!
//!  1. Implement core features as functions, so they can be re-used within Rust.
//!     These functions should generally return a `Result` type, so the caller
//!     can decide how to deal with the error.
//!  2. Executable commands with a simple interface that typically act as thin
//!     wrappers around the library functions, or perhaps combine the library
//!     functions in interesting ways.
//!  3. As well designed code that can be copied as repurposed when necessary.
//!
//! A fourth avenue may be explored, which is to adopt the
//! [nushell](https://github.com/rjbs/Sweater) approach to transfering
//! tabular data between commands.
//!
//! For a related project that also follows Software Tools in Rust, and
//! may serve as an interesting comparison, see
//! [Sweater](https://github.com/rjbs/Sweater).
//!
//! ## Functional facilities
//!
//! Higher-order-functions (HOFs) are frequently used to reduce code
//! complexity, verbosity, and the risk of errors. Primary examples are
//! `map`, `for_each` (like `map` but effectful), and `fold`. As pointed
//! out in Software Tools, pp 21, *"The best programs are designed in
//! terms of loosely coupled functions that each does a simple task."*
//!
//! ## Currently Implemented Tools
//! - [x] `cp`
//! - [x] `wc`
//! - [ ] `detab`
//!
//! ## Dependencies
//!
//! Since the goal is to make the software both as self-contained and
//! as illustrative as possible, we've tried to rely on very few dependencies.
//! The following exceptions exist:
//!
//! - [fp-core](https://docs.rs/fp-core)
//!   This is what one would typically find as part the standard library
//!   in a functional language, so we have included it here. Though Rust is functional
//!   in a sense — it has lamda functions (i.e. Rust closures) and the stand library
//!   has many higher-order functions (HOFs) — its standard library doesn't include
//!   traits that are commonly found to be helpful abstracts in functional languages.
//!   We will use a few of these where it is particularly illustrative or sensible,
//!   but will stick with idiomatic Rust where that is obviously simpler.
//! - [seahorse](https://docs.rs/seahorse)
//!   Seahorse is a minimal argument parser. Judging by some results
//!   returned by Google, [clap](https://clap.rs) is far more popular, but
//!   has additional dependencies; we are striving for being as portable
//!   as possible, so the minimality seemed to line up with that
//!   goal. Additionally, Clap doesn't appear to allow passing in argument
//!   lists directly, which is useful for maintaining separate commands
//!   that build on each other. In any case, argument parsing is only used
//!   very late in the application logic, and most of the API could be used
//!   without worrying about it.
//!
//! ### Currently unused
//!
//! - [im](https://docs.rs/im)
//!   Immutable data structures that implement structural sharing can be
//!   even more performant than `std`'s mutable structures for large
//!   data types, and while Rust makes mutation far safer than most languages,
//!   mutation can still result in confusion at times, so in the cases where
//!   clarity is more important than performance (or performance doesn't
//!   matter much, e.g. one-ops), it may be preferable to use immutable data
//!   structures.
//!
//!
//! ## Build
//!
//! Currently, do generate small builds the following commands
//! are required.
//!
//! 1. (only once per environment) Make source code for the standard library available:
//!
//! ```plain
//! rustup component add rust-src --toolchain nightly
//! ```
//!
//! 2.
//!
//! ```plain
//! cargo +nightly build -Z build-std --target x86_64-unknown-linux-gnu --release
//! ```
//!
//! 3. (optional) `strip` binary - see links in notes
//!
//! ## Misc Notes
//!
//! ### Using todo!() to
//!
//! Using `todo!()` from `std::todo` is a helpful way to incrementally
//! develop a feature while still getting feedback from the
//! compiler. [**TODO**: show example]
//!
//! A [caveat](https://github.com/rust-lang/rfcs/issues/3045) is that
//! currently you need code in the function after the `todo!()`, even
//! if it doesn't match the type. For instance, we can use a function
//! like:
//!
//! ```
//! pub fn some_num() -> i32 {
//!     todo!(); ();
//! }
//! ```
//!
//! Most beneficial is that `rustc` will warn you if you a `todo!()` is
//! left in your code, since it would result in a panic if that execution
//! path were to occur.
//!
//!
//! ### Optimizing for size
//!
//! * https://github.com/johnthagen/min-sized-rust
//!
//! ## Project administration
//!
//! ### Git hooks
//!
//! #### Cargo-Husky
//!
//! We use [cargo-husky](https://github.com/rhysd/cargo-husky) to keep in
//! line; it enforces several checks with a `pre-push` hook. Sometimes it
//! is a bit restrictive, so if we need to push
//! in-progress work to a branch, we can use
//! `git push --no-verify -u origin feature_branch`.
//! Cargo-husky expects certain files to be at the root of the repository,
//! thus the symlinks.
//!
//! #### pre-commit
//!
//! We include the following, less stringent checks for pre-commit.
//!
//! ```bash
//! #!/bin/sh
//!
//! # Put in your Rust repository's .git/hooks/pre-commit to ensure you never
//! # breaks rustfmt.                                                                                                                      
//! #
//! # WARNING: rustfmt is a fast moving target so ensure you have the version that
//! #          all contributors have.
//!
//! for FILE in `git diff --cached --name-only`; do
//!     if [[ -f "$FILE" ]] && [[ $FILE == *.rs ]] \
//!            && ! rustup run nightly rustfmt --unstable-features \
//!                 --skip-children $FILE; then
//!         echo "Commit rejected due to invalid formatting of \"$FILE\" file."
//!         exit 1
//!     fi
//! done
//!
//! cd Rust/sfw-tools && cargo readme > README.md && git add README.md
//! ```
//! As can be seen this also gnerates the README from doc comments in `lib.rs`.
//!

#![deny(unused_must_use)]
#![feature(try_trait)]

use std::env;
use std::io::Error;

use seahorse::App;

pub mod bytes_iter;
pub use bytes_iter::BytesIter;

pub mod constants;
pub use constants::*;

pub mod error;
pub use error::*;

pub mod util;
pub use util::*;

// Following are re-exports for specific functionality //

pub mod copying;
pub use copying::*;

pub mod counting;
pub use counting::*;

pub mod tabs;
pub use tabs::*;

pub fn get_args() -> Result<(String, Vec<String>), Error> {
    let mut args_in = env::args();
    let cmd = args_in.next().sfw_err("Impossible: no first arg!")?;
    let args_out: Vec<String> = args_in.collect::<Vec<String>>();
    Ok((cmd, args_out))
}

/// This is a wrapper around the Seahorse `App.run` that emits
/// a nicer user error message if there are no aguments provided.
pub fn run_app(app: App, args: Vec<String>, arg_err: &str) {
    match args.len() {
        0 => user_exit(&format!("{}: Zero arguments in run_app", arg_err)),
        _ => app.run(args),
    }
}
