// TODO: distinguish between byte-based and unicode-based compression

use std::collections::HashMap;
use std::convert::TryFrom;
use std::fs::File;
use std::io::{Error, Read, Write};

use seahorse::{App, Command, Context};
use tailcall::tailcall;

use crate::bytes_iter::BytesIter;
use crate::constants::*;
use crate::error::*;
use crate::util::write_u8;

const THRESH: usize = 5;
const RCODE: u8 = b'0';
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
        .description("compress: adjacent redundancy compression\
        ; output to STDOUT is the default")
        .usage(COMPRESS_USAGE)
        .action(run_compress_seahorse_action)
}

pub fn run_compress_seahorse_action(ctxt: &Context) {
    let args = &mut ctxt.args.iter();
    let src = args.next().user_err("compress: missing source");
    let f_out: Box<dyn Write> = match args.next() {
        Some(dst) => Box::new(
            File::create(&dst)
                .user_err(&*format!("Couldn't open destination: {}", &dst)),
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
    let f_in = File::open(&src).sfw_err("Couldn't open source")?;
    let f_in_iter = BytesIter::new(f_in, MAX_CHUNK_SIZE);
    let mut out_buf: Vec<u8> = Vec::with_capacity(MAX_CHUNK_SIZE);
    compress_go(
        &mut f_out,
        f_in_iter,
        vec![].into_iter(),
        &mut out_buf,
        0,
    )
}

// This implementation does not compress across boundaries in byte chunks,
// If this were desired, then a folded approach, as is used for word counts,
// might be desirable.
#[tailcall]
fn compress_go<'a, R, W>(
    f_out: &mut W,
    mut bytes_iter: BytesIter<R>,
    mut buf_iter: std::vec::IntoIter<u8>,
    out_buf: &mut Vec<u8>,
    tab_pos: usize,
) -> Result<(), Error>
where
    R: Read,
    W: Write,
{
    match buf_iter.next() {
        Some(char) => {
            let char_streak = &mut buf_iter.take_while(|c| c == char).collect::<Vec<u8>>();
            char_streak.add(char);

            if char_streak.len() >= THRESH {
                write_u8(f_out, RCODE)?;
                write_u8(f_out, char)?;
                let streak_len = u8::try_from(char_streak.len())?;
                write_u8(f_out, streak_len)
            } else {
                // TODO

                // let spc_count = tab_pos_to_space(tab_cnf, tab_pos);
                // f_out.write_all(&SPACE_ARRAY[0..spc_count])?;
                // tab_pos + 1
            }

            compress_go(tab_cnf, f_out, bytes_iter, buf_iter, tab_pos_new)
        }
        None => {
            match bytes_iter.next() {
                Some(buf_new) => {
                    let buf_iter = buf_new?.into_iter(); //shadow
                    compress_go(tab_cnf, f_out, bytes_iter, buf_iter, tab_pos)
                }
                None => Ok(()), /* Finished */
            }
        }
    }
}

pub fn decompress_app() -> App {
    App::new("decompress")
        .author("Brandon Elam Barker")
        .action(run_decompress_seahorse_action)
        .command(run_decompress_seahorse_cmd())
}
const decompress_USAGE: &str = "decompress SOURCE_FILE DEST_FILE";

pub fn run_decompress_seahorse_cmd() -> Command {
    Command::new("decompress")
        .description(
            "decompress: replace spaces with tabs in a file\
            ; output to STDOUT is the default",
        )
        .usage(decompress_USAGE)
        .action(run_decompress_seahorse_action)
}

pub fn run_decompress_seahorse_action(ctxt: &Context) {
    let args = &mut ctxt.args.iter();
    let src = args.next().user_err("decompress: missing source");
    let f_out: Box<dyn Write> = match args.next() {
        Some(dst) => Box::new(
            File::create(&dst)
                .user_err(&*format!("Couldn't open destination: {}", &dst)),
        ),
        None => Box::new(std::io::stdout()),
    };
    run_decompress(&src, f_out);
}

/// Convenience function for running decompress in idiomatic fashion
/// (i.e.) errors are printed to user and the program exits.
pub fn run_decompress(src: &str, dst: Box<dyn Write>) {
    decompress(src, dst).user_err("Error in decompress");
}

pub fn decompress<W: Write>(src: &str, mut f_out: W) -> Result<(), Error> {
    let f_in = File::open(&src).sfw_err("Couldn't open source")?;
    let f_in_iter = BytesIter::new(f_in, MAX_CHUNK_SIZE);
    decompress_go(
        &TabConf::TabConstant(2),
        &mut f_out,
        f_in_iter,
        vec![].into_iter(),
        0,
        0,
    )
}

#[tailcall]
fn decompress_go<'a, R, W>(
    tab_cnf: &TabConf,
    f_out: &mut W,
    mut bytes_iter: BytesIter<R>,
    mut buf_iter: std::vec::IntoIter<u8>,
    tab_pos: usize,
    spc_count: usize,
) -> Result<(), Error>
where
    R: Read,
    W: Write,
{
    match buf_iter.next() {
        Some(byte) => {
            let (tab_pos, spc_count) = match byte {
                b' ' => (tab_pos + 1, spc_count + 1),
                b'\n' => (0, 0),
                b'\t' => (tab_pos + 1, spc_count),
                _ => (tab_pos, spc_count),
            };
            let spaces_for_tab = tab_pos_to_space(tab_cnf, tab_pos);
            let (tab_pos, spc_count) = if spc_count == spaces_for_tab {
                write_u8(f_out, b'\t')?;
                (tab_pos + 1, 0)
            } else {
                match byte {
                    b' ' => (tab_pos, spc_count),
                    _ => {
                        f_out.write_all(
                            &(0..spc_count).map(|_| b' ').collect::<Vec<u8>>(),
                        )?;
                        write_u8(f_out, byte)?;
                        (tab_pos, 0)
                    }
                }
            };
            decompress_go(tab_cnf, f_out, bytes_iter, buf_iter, tab_pos, spc_count)
        }
        None => {
            match bytes_iter.next() {
                Some(buf_new) => {
                    let buf_iter = buf_new?.into_iter(); //shadow
                    decompress_go(
                        tab_cnf, f_out, bytes_iter, buf_iter, tab_pos,
                        spc_count,
                    )
                }
                None => Ok(()), /* Finished */
            }
        }
    }
}
