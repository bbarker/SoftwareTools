use std::collections::HashMap;
use std::fs::File;
use std::io::{Error, Read, Write};

// use seahorse::{App, Command, Context};
use tailcall::tailcall;

use crate::bytes_iter::BytesIter;
use crate::constants::*;
use crate::error::*;

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
pub fn tab_pos_to_space(pos: usize, tab_config: &TabConf) -> usize {
    match tab_config {
        TabConf::TabConstant(spcs) => *spcs,
        TabConf::TabMap(tab_def, tmap) => *tmap.get(&pos).unwrap_or(tab_def),
    }
}


#[tailcall]
fn detab_go<'a, I, R, W>(
    f_out: &W,
    bytes_iter: BytesIter<R>,
    buf_iter: I,
    tab_pos_last: usize,
) -> Result<(), Error> where
    I: Iterator<Item = &'a u8>,
    R: Read,
    W: Write,
{
    todo!();

    match buf_iter.next() {
        Some(byt) => {
            todo!();
            detab_go(f_out, bytes_iter, buf_iter, /*&tab_pos_new*/ todo!())
        },
        None => {
            match bytes_iter.next() {
                Some(buf_new) => {
                    let buf_test : Vec<u8> = buf_new?;
                    let buf_iter = buf_test.iter(); //shadow
                    detab_go(f_out, bytes_iter, buf_iter, /*&tab_pos_new*/ todo!())
                },
                None => todo!(),
            }
        }
    }
}

/* //Pseudo code

go(f_out: &File, bytes_iter: BytesIter, buf_iter: mut Iterator<u8>, tab_pos_last: usize) {

  f_out.write(buf_iter.take_until(|c| is_tab_or_newline(c)));

  //TODO: analyze next character for tab or newline
  let tab_or_nl = ...
  if is_newline(tab_or_nl) {

  }
  else {

  };


  if buf.is_empty() {
    let buf_new = bytes_iter.read();
    let buf_iter = buf_new.iter(); //shadow
    go(&f_out, &bytes_iter, buf_iter,  &tab_pos_new)
  }
  else { go(&f_out, &bytes_iter, &buf_iter, &tab_pos_new) }

}
*/
