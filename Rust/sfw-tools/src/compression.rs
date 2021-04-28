// TODO: distinguish between byte-based and unicode-based compression

/*
This entire file used tabs.rs as a starting point, and the structure remains similar.
Instead of detab/entab, we have compress/expand. Both are filters, and both use
many of the same local structure, data structures, user interfaces, etc.
 */

use std::convert::TryFrom;
use std::fs::File;
use std::io::{Error, ErrorKind::Other, Read, Write};
use std::iter::Peekable;

use peeking_take_while::PeekableExt;
use seahorse::{App, Command, Context};
use tailcall::tailcall;

use crate::bytes_iter::BytesIter;
use crate::error::*;
use crate::iter_extra::*;
use crate::util::write_u8;

const THRESH: usize = 5;
const RCODE: u8 = 0;
const MAX_CHUNK_SIZE: usize = 255;

pub fn compress_app() -> App {
    App::new("compress")
        .author("Brandon Elam Barker")
        .action(run_compress_seahorse_action)
        .command(run_compress_seahorse_cmd())
}

const COMPRESS_USAGE: &str = "compress SOURCE_FILE DEST_FILE";

pub fn run_compress_seahorse_cmd() -> Command {
    Command::new("compress")
        .description(
            "compress: adjacent redundancy compression\
        ; output to STDOUT is the default",
        )
        .usage(COMPRESS_USAGE)
        .action(run_compress_seahorse_action)
}

pub fn run_compress_seahorse_action(ctxt: &Context) {
    let args = &mut ctxt.args.iter();
    let src = args.next().user_err("compress: missing source");
    let f_out: Box<dyn Write> = match args.next() {
        Some(dst) => Box::new(
            File::create(&dst)
                .user_err(&format!("Couldn't open destination: {}", &dst)),
        ),
        None => Box::new(std::io::stdout()),
    };
    run_compress(&src, f_out);
}

/// Convenience function for running compress in idiomatic fashion
/// (i.e.) errors are printed to user and the program exits.
pub fn run_compress(src: &str, dst: Box<dyn Write>) {
    compress(src, dst).user_err("Error in compress");
}

pub fn compress<W: Write>(src: &str, mut f_out: W) -> Result<(), Error> {
    let f_in =
        File::open(&src).sfw_err(&format!("Couldn't open source '{}'", src))?;
    let f_in_iter = BytesIter::new(f_in, MAX_CHUNK_SIZE);
    let mut out_buf: Vec<u8> = Vec::with_capacity(MAX_CHUNK_SIZE);
    compress_go(
        &mut f_out,
        f_in_iter,
        vec![].into_iter().peekable(),
        &mut out_buf,
    )
}

// This implementation does not compress across boundaries in byte chunks,
// If this were desired, then a folded approach, as is used for word counts,
// might be desirable.
#[tailcall]
fn compress_go<'a, R, W>(
    f_out: &mut W,
    mut bytes_iter: BytesIter<R>,
    mut buf_iter: Peekable<std::vec::IntoIter<u8>>,
    out_buf: &mut Vec<u8>,
) -> Result<(), Error>
where
    R: Read,
    W: Write,
{
    match buf_iter.next() {
        Some(char) => {
            let char_streak = &mut buf_iter
                .by_ref()
                .peeking_take_while(|c| *c == char)
                .collect::<Vec<u8>>();
            char_streak.push(char);
            if char_streak.len() >= THRESH {
                write_buf_out(out_buf, f_out)?; // Write out non-streak buffer
                write_u8(f_out, RCODE)?;
                write_u8(f_out, char)?;
                let char_streak_len = char_streak.len();
                let streak_len =
                    u8::try_from(char_streak_len).map_err(|_| {
                        Error::new(
                            Other,
                            format!(
                                "Couldn't convert char_streak_len '{}' to a u8",
                                char_streak_len
                            ),
                        )
                    })?;
                write_u8(f_out, streak_len)?;
            } else {
                if out_buf.len() + char_streak.len() > MAX_CHUNK_SIZE {
                    write_buf_out(out_buf, f_out)?;
                }
                out_buf.append(char_streak);
            }
            compress_go(f_out, bytes_iter, buf_iter, out_buf)
        }
        None => {
            match bytes_iter.next() {
                Some(buf_new) => {
                    let buf_iter = buf_new?.into_iter().peekable(); //shadow
                    compress_go(f_out, bytes_iter, buf_iter, out_buf)
                }
                None => write_buf_out(out_buf, f_out), /* Finished */
            }
        }
    }
}

fn write_buf_out<W: Write>(
    out_buf: &mut Vec<u8>,
    f_out: &mut W,
) -> Result<(), Error> {
    let out_len = out_buf.len();
    let out_len = u8::try_from(out_len).map_err(|_| {
        Error::new(
            Other,
            format!("Couldn't convert out_len '{}' to a u8", out_len),
        )
    })?;
    write_u8(f_out, out_len)?;
    f_out.write_all(out_buf)?;
    out_buf.clear();
    Ok(())
}

pub fn expand_app() -> App {
    App::new("expand")
        .author("Brandon Elam Barker")
        .action(run_expand_seahorse_action)
        .command(run_expand_seahorse_cmd())
}
const EXPAND_USAGE: &str = "expand SOURCE_FILE DEST_FILE";

pub fn run_expand_seahorse_cmd() -> Command {
    Command::new("expand")
        .description(
            "expand: replace spaces with tabs in a file\
            ; output to STDOUT is the default",
        )
        .usage(EXPAND_USAGE)
        .action(run_expand_seahorse_action)
}

pub fn run_expand_seahorse_action(ctxt: &Context) {
    let args = &mut ctxt.args.iter();
    let src = args.next().user_err("expand: missing source");
    let f_out: Box<dyn Write> = match args.next() {
        Some(dst) => Box::new(
            File::create(&dst)
                .user_err(&format!("Couldn't open destination: {}", &dst)),
        ),
        None => Box::new(std::io::stdout()),
    };
    run_expand(&src, f_out);
}

/// Convenience function for running expand in idiomatic fashion
/// (i.e.) errors are printed to user and the program exits.
pub fn run_expand(src: &str, dst: Box<dyn Write>) {
    expand(src, dst).user_err("Error in expand");
}

pub fn expand<W: Write>(src: &str, mut f_out: W) -> Result<(), Error> {
    let f_in = File::open(&src).sfw_err("Couldn't open source")?;
    let f_in_iter = BytesIter::new(f_in, MAX_CHUNK_SIZE);
    expand_go(&mut f_out, f_in_iter, vec![].into_iter())
}

#[tailcall]
fn expand_go<'a, R, W>(
    f_out: &mut W,
    mut bytes_iter: BytesIter<R>,
    mut buf_iter: std::vec::IntoIter<u8>,
) -> Result<(), Error>
where
    R: Read,
    W: Write,
{
    match buf_iter.next() {
        Some(byte) => {
            match byte {
                0 => {
                    let repeat_char = buf_iter
                        .next()
                        .sfw_err("Couldn't read repeat character")?;
                    let repeat_count = buf_iter
                        .next()
                        .sfw_err("Couldn't read repeat count")?;
                    f_out.write_all(
                        &(0..repeat_count)
                            .map(|_| repeat_char)
                            .collect::<Vec<u8>>(),
                    )?;
                }
                read_size => {
                    let read_size = read_size as usize;
                    let non_repeat_string =
                        buf_iter.by_ref().safe_take(read_size)?;
                    f_out.write_all(&non_repeat_string)?
                }
            };
            expand_go(f_out, bytes_iter, buf_iter)
        }
        None => {
            match bytes_iter.next() {
                Some(buf_new) => {
                    let buf_iter = buf_new?.into_iter(); //shadow
                    expand_go(f_out, bytes_iter, buf_iter)
                }
                None => Ok(()), /* Finished */
            }
        }
    }
}
