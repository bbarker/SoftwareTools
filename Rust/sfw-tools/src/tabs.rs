use std::collections::HashMap;
use std::fs::File;
use std::io::{Error, Read, Write};

// use seahorse::{App, Command, Context};
// use byteorder::WriteBytesExt;
use tailcall::tailcall;

use crate::bytes_iter::BytesIter;
use crate::constants::*;
use crate::error::*;
use crate::util::write_u8;

pub fn detab(src: &str, dst: &str) -> Result<(), Error> {
    let f_in = File::open(&src).sfw_err("Couldn't open source")?;
    let mut f_in_iter = BytesIter::new(f_in, DEFAULT_BUF_SIZE);
    let mut f_out = File::create(&dst)
        .sfw_err(&*format!("Couldn't open destination: {}", &dst))?;

    f_in_iter.try_for_each(|b_slice_res| match b_slice_res {
        Ok(b_slice) => f_out.write_all(&b_slice),
        Err(err) => Err(err),
    })
}

pub enum TabConf {
    TabConstant(usize),
    TabMap(usize, HashMap<usize, usize>),
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
    tab_pos_last: usize,
) -> Result<(), Error>
where
    R: Read,
    W: Write,
{
    match buf_iter.next() {
        Some(byte) => {
            let tab_pos_new = match byte {
                b'\t' => {
                    let tab_pos_new = tab_pos_last + 1;
                    let spc_count = tab_pos_to_space(tab_cnf, tab_pos_new);
                    f_out.write_all(&SPACE_ARRAY[0..spc_count])?;
                    tab_pos_new
                }
                b'\n' => {
                    write_u8(f_out, byte)?;
                    0
                }
                _ => {
                    write_u8(f_out, byte)?;
                    tab_pos_last
                }
            };
            detab_go(tab_cnf, f_out, bytes_iter, buf_iter, tab_pos_new)
        }
        None => {
            match bytes_iter.next() {
                Some(buf_new) => {
                    let buf_iter = buf_new?.into_iter(); //shadow
                    detab_go(tab_cnf, f_out, bytes_iter, buf_iter, tab_pos_last)
                }
                None => Ok(()), /* Finished */
            }
        }
    }
}
