#![deny(unused_must_use)]
#![feature(try_trait)]

use std::env;
use std::fs::File;
use std::io::{Error, Write};

mod bytes_iter;
pub use bytes_iter::BytesIter;

mod error;
pub use error::*;

const DEFAULT_BUF_SIZE: usize = 4096;

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

pub const fn is_newline(bt: u8) -> bool {
    bt == b'\n'
}

/*
struct WordCount {
    characters: u32,
    words: u32,
    lines: u32,
}
*/

/// Convenience function for running wc in idiomatic fashion
/// (i.e.) errors are printed to user and the program exits.
pub fn run_wc(src: &str) {
    let wc_res = wc(src).user_err("Error in wc");
    println!("{}", wc_res);
}

// TODO: result should have WordCount output
pub fn wc(src: &str) -> Result<u32, Error> {
    let f_in = File::open(&src)
        .sfw_err(&*format!("Couldn't open source: {}", &src))?;
    wc_file(&f_in)
}

/// In Chapter 1, page 15 of Software Tools, the authors discuss the
/// hazards of boundary conditions in programming. Certainly this is still
/// a problem in Rust, but using Rust's functional programming facilities,
/// and types can help to greatly reduce the occurrence of such errors.
pub fn wc_file(f_in: &File) -> Result<u32, Error> {
    BytesIter::new(f_in, DEFAULT_BUF_SIZE).try_fold(0u32, |ac_tot, b_slice| {
        Ok(ac_tot
            + b_slice?.iter().fold(0u32, |ac, bt| {
                if is_newline(*bt) {
                    ac + 1
                } else {
                    ac
                }
            }))
    })
}
