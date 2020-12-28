#![deny(unused_must_use)]
#![feature(try_trait)]

use std::env;
use std::fmt;
use std::fmt::Display;
use std::fs::File;
use std::io::Write;
use std::io::{Error, ErrorKind::InvalidInput};
use std::option::NoneError;
use std::process;

use std::io::{self, BufRead, BufReader, Read};

use read_byte_slice::{ByteSliceIter, FallibleStreamingIterator};

pub fn get_args() -> Result<(String, Vec<String>), Error> {
    let mut args_in = env::args();
    let cmd = args_in
        .next()
        .ok_or_else(|| Error::new(InvalidInput, "Impossible: no first arg!"))?;
    let args_out: Vec<String> = args_in.collect::<Vec<String>>();
    Ok((cmd, args_out))
}

/// TODO: refactor to have a cp_file that will take File parameters directly
pub fn cp(src: &str, dst: &str) -> Result<(), Error> {
    let f_in =
        File::open(&src).user_err(&*format!("Couldn't open source: {}", &src));

    let f_in_iter = ByteSliceIter::new(f_in, 4096);
    let mut f_out = File::create(&dst)
        .user_err(&*format!("Couldn't open destination: {}", &dst));

    f_in_iter.for_each(|b_slice| {
        f_out
            .write_all(b_slice)
            .user_err(&*format!("Failure writing to {}.", &dst));
    })
}

pub fn wc(src: &str) {
    let f_in =
        File::open(&src).user_err(&*format!("Couldn't open source: {}", &src));
    println!("{}", wc_file(&f_in))
}

pub const fn is_newline(bt: u8) -> bool {
    bt == b'\n'
}

/// In Chapter 1, page 15 of Software Tools, the authors discuss the
/// hazards of boundary conditions in programming. Certainly this is still
/// a problem in Rust, but using Rust's functional programming facilities,
/// and types can help to greatly reduce the occurrence of such errors.
pub fn wc_file(f_in: &File) -> u32 {
    todo!();
    ();

    // let f_in_iter = ByteSliceIter::new(f_in, 4096);
    // f_in_iter.collect().iter().fold(0, |b_slice|
    //     b_slice.iter().fold(0, |ac, bt| if is_newline(*bt) {ac+1} else {ac})
    // )
}

// put trait FallibleStreamingIteratorRich<T> {
//     // TODO: merge upstream as method
//     // TODO: const_fn
//     fn fold_fsi(it: Self, );
// }

pub struct NoneErrorRich(NoneError);
const NONE_ERROR_RICH: NoneErrorRich = NoneErrorRich(NoneError);

//
impl fmt::Display for NoneErrorRich {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

/// Convenience function for running cp in idiomatic fashion
/// (i.e.) errors are printed to user and the program exits.
pub fn run_cp(src: &str, dst: &str) {
    cp(src, dst).user_err("Error in cp");
}

const USER_ERROR_CODE: i32 = 1;

pub fn exit(msg: &str) {
    eprintln!("{}", msg);
    process::exit(USER_ERROR_CODE)
}

pub trait SfwRes<T, E: Display> {
    fn unwrap_or_else<F: FnOnce(E) -> T>(self, op: F) -> T;

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

impl<T> SfwRes<T, NoneErrorRich> for Option<T> {
    fn unwrap_or_else<F: FnOnce(NoneErrorRich) -> T>(self, op: F) -> T {
        self.unwrap_or_else(|| op(NONE_ERROR_RICH))
    }
}

pub struct BytesIter<R: Read> {
    buf_reader: BufReader<R>,
    buf: Vec<u8>,
    /// Since Iterator returns an Option instead of an Error,
    /// we log the error here, should it occur.
    error: Option<io::Error>,
}

/// Inspired by ByteSliceIter, but using new and improved std Iterator trait
/// One downside is that we must clone the buffer due to Iterator's next signature,
/// which only permits returning `Option<Self::Item>` and *not* `&Option<Self::Item>`.
/// Thus, for applications where the returned buffer slice is only read and not
/// consumed, it may be more efficient to use ByteSliceIter or related approaches.
impl<R: Read> BytesIter<R> {
    /// The default size in std [is 8 * 1024](https://github.com/rust-lang/rust/blob/6ccfe68076abc78392ab9e1d81b5c1a2123af657/src/libstd/sys_common/io.rs#L10).
    pub fn new(reader: R, size: usize) -> BytesIter<R> {
        BytesIter {
            buf_reader: BufReader::with_capacity(size, reader),
            buf: Vec::with_capacity(size),
            error: None,
        }
    }
}

impl<R: Read> Iterator for BytesIter<R> {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        let buf_len = self.buf.len();
        if buf_len > 0 {
            self.buf_reader.consume(buf_len);
            self.buf.clear();
        }
        let buf_len = self.buf_reader.buffer().len();
        match self.buf_reader.fill_buf() {
            Ok(buf) => {
                self.buf.extend_from_slice(&buf);
                Some(self.buf.clone())
            }
            Err(err) => {
                self.error = Some(err);
                None
            }
        }
    }
}
