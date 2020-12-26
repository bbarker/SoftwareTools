#![deny(unused_must_use)]

use std::env;
use std::fmt::Display;
use std::fs::File;
use std::io::Write;
use std::io::{Error, ErrorKind::InvalidInput};
use std::process;

use read_byte_slice::{ByteSliceIter, FallibleStreamingIterator};

pub fn get_args() -> Result<(String, Vec<String>), Error> {
    let mut args_in = env::args();
    let cmd = args_in
        .next()
        .ok_or_else(|| Error::new(InvalidInput, "Impossible: no first arg!"))?;
    let args_out: Vec<String> = args_in.collect::<Vec<String>>();
    Ok((cmd, args_out))
}

pub fn cp(src: String, dst: String) -> Result<(), Error> {
    let f_in =
        File::open(&src).expect(&format!("Couldn't open source: {}", &src));

    let f_in_iter = ByteSliceIter::new(f_in, 4096);
    let mut f_out = File::create(&dst)
        .expect(&format!("Couldn't open destination: {}", &dst));

    f_in_iter.for_each(|b_slice| {
        f_out
            .write(b_slice)
            .expect(&format!("Failure writing to {}.", &dst));
    })
}

const USER_ERROR_CODE: i32 = 1;

pub trait SfwRes<T, E: Display> {
    fn unwrap_or_else<F: FnOnce(E) -> T>(self, op: F) -> T;

    // you can write a trait of your own and implement that trait for
    // Result<T, E> where E: Display
    fn user_err(self, fstr: &str) -> T
    where
        Self: Sized,
    {
        self.unwrap_or_else(|err| {
            eprintln!("{}: {}", fstr, err);
            process::exit(USER_ERROR_CODE)
        })
    }
}

impl<T, E: Display> SfwRes<T, E> for Result<T, E> {
    fn unwrap_or_else<F: FnOnce(E) -> T>(self, op: F) -> T {
        self.unwrap_or_else(op)
    }
}
