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
