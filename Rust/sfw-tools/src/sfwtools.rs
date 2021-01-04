#![deny(unused_must_use)]
#![feature(try_trait)]

use std::env;
use std::fs::File;
use std::io::{Error, Write};

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

pub mod counting;
pub use counting::*;

pub fn get_args() -> Result<(String, Vec<String>), Error> {
    let mut args_in = env::args();
    let cmd = args_in.next().sfw_err("Impossible: no first arg!")?;
    let args_out: Vec<String> = args_in.collect::<Vec<String>>();
    Ok((cmd, args_out))
}

/// Convenience function for running cp in idiomatic fashion
/// (i.e.) errors are printed to user and the program exits.
pub fn run_cp(src: &str, dst: &str) {
    cp(src, dst).user_err("Error in cp");
}

pub fn cp(src: &str, dst: &str) -> Result<(), Error> {
    let f_in = File::open(&src).sfw_err("Couldn't open source")?;
    let mut f_in_iter = BytesIter::new(f_in, DEFAULT_BUF_SIZE);
    let mut f_out = File::create(&dst)
        .sfw_err(&*format!("Couldn't open destination: {}", &dst))?;

    f_in_iter.try_for_each(|b_slice_res| match b_slice_res {
        Ok(b_slice) => f_out.write_all(&b_slice),
        Err(err) => Err(err),
    })
}

/// This is a wrapper around the Seahorse `App.run` that emits
/// a nicer user error message if there are no aguments provided.
pub fn run_app(app: App, args: Vec<String>, arg_err: &str) {
    match args.len() {
        0 => user_exit(&format!("{}: Zero arguments in run_app", arg_err)),
        _ => app.run(args),
    }
}
