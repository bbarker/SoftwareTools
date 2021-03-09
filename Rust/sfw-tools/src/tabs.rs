use std::collections::HashMap;
use std::fs::File;
use std::io::{Error, Read, Write};

use seahorse::{App, Command, Context};
use tailcall::tailcall;

use crate::bytes_iter::BytesIter;
use crate::constants::*;
use crate::error::*;
use crate::util::write_u8;

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum TabConf {
    TabConstant(usize),
    TabMap(usize, HashMap<usize, usize>),
}

pub fn detab_app() -> App {
    App::new("detab")
        .author("Brandon Elam Barker")
        .action(run_detab_seahorse_action)
        .command(run_detab_seahorse_cmd())
}

const DETAB_USAGE: &str = "detab [SOURCE_FILE] [DEST_FILE]";

pub fn run_detab_seahorse_cmd() -> Command {
    Command::new("detab")
        .description("detab: remove tabs from a file; output to STDOUT")
        .usage(DETAB_USAGE)
        .action(run_detab_seahorse_action)
}

pub fn run_detab_seahorse_action(ctxt: &Context) {
    let args = &mut ctxt.args.iter();
    let src = args.next().user_err("detab: missing source");
    let f_out: Box<dyn Write> = match args.next() {
        Some(dst) => Box::new(
            File::create(&dst)
                .user_err(&*format!("Couldn't open destination: {}", &dst)),
        ),
        None => Box::new(std::io::stdout()),
    };
    run_detab(&src, f_out);
}

/// Convenience function for running detab in idiomatic fashion
/// (i.e.) errors are printed to user and the program exits.
pub fn run_detab(src: &str, dst: Box<dyn Write>) {
    detab(src, dst).user_err("Error in detab");
}

pub fn detab<W: Write>(src: &str, mut f_out: W) -> Result<(), Error> {
    let f_in = File::open(&src).sfw_err("Couldn't open source")?;
    let f_in_iter = BytesIter::new(f_in, DEFAULT_BUF_SIZE);
    detab_go(
        &TabConf::TabConstant(2),
        &mut f_out,
        f_in_iter,
        vec![].into_iter(),
        0,
    )
}

// TODO: const
pub fn tab_pos_to_space(tab_config: &TabConf, pos: usize) -> usize {
    match tab_config {
        TabConf::TabConstant(spcs) => *spcs,
        TabConf::TabMap(tab_def, tmap) => *tmap.get(&pos).unwrap_or(tab_def),
    }
}

// TODO: in outer function use, a BufWriter:
//https://stackoverflow.com/a/47184074/3096687

// TODO: we need dynamically allocated, fixed-sized arrays:
//       https://github.com/rust-lang/rust/issues/48055
//
// A good soultion would be to allocate a vector of spaces,
// grow as necessary, and take a slice. But for now, we can simply
// allocate a constant array:

const SPACE_ARRAY: [u8; 256] = [b' '; 256];

#[tailcall]
fn detab_go<'a, R, W>(
    tab_cnf: &TabConf,
    f_out: &mut W,
    mut bytes_iter: BytesIter<R>,
    mut buf_iter: std::vec::IntoIter<u8>,
    tab_pos: usize,
) -> Result<(), Error>
where
    R: Read,
    W: Write,
{
    match buf_iter.next() {
        Some(byte) => {
            let tab_pos_new = match byte {
                b'\t' => {
                    let spc_count = tab_pos_to_space(tab_cnf, tab_pos);
                    f_out.write_all(&SPACE_ARRAY[0..spc_count])?;
                    tab_pos + 1
                }
                b'\n' => {
                    write_u8(f_out, byte)?;
                    0
                }
                _ => {
                    write_u8(f_out, byte)?;
                    tab_pos
                }
            };
            detab_go(tab_cnf, f_out, bytes_iter, buf_iter, tab_pos_new)
        }
        None => {
            match bytes_iter.next() {
                Some(buf_new) => {
                    let buf_iter = buf_new?.into_iter(); //shadow
                    detab_go(tab_cnf, f_out, bytes_iter, buf_iter, tab_pos)
                }
                None => Ok(()), /* Finished */
            }
        }
    }
}
