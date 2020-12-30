use std::fs::File;
use std::io::Error;

use crate::bytes_iter::BytesIter;
use crate::constants::*;
use crate::error::*;
use crate::util::is_newline;

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
