use std::fs::File;
use std::io::{Error, Write};

use seahorse::{App, Command, Context};

use crate::bytes_iter::BytesIter;
use crate::constants::*;
use crate::error::*;

pub fn cp_app() -> App {
    App::new("cp")
        .author("Brandon Elam Barker")
        .action(run_cp_seahorse_action)
        .command(run_cp_seahorse_cmd())
}

const CP_USAGE: &str = "cp SOURCE_FILE DEST_FILE";

pub fn run_cp_seahorse_cmd() -> Command {
    Command::new("cp")
        .description("cp: copy a file to another file")
        .usage(CP_USAGE)
        .action(run_cp_seahorse_action)
}

pub fn run_cp_seahorse_action(ctxt: &Context) {
    let mut args = ctxt.args.iter();
    let src = args.next().user_err("cp: missing source");
    let dst = args.next().user_err("cp: missing destination");
    run_cp(src, dst);
}

/// Convenience function for running cp in idiomatic fashion
/// (i.e.) errors are printed to user and the program exits.
pub fn run_cp(src: &str, dst: &str) {
    cp(src, dst).user_err("Error in cp");
}

pub fn cp(src: &str, dst: &str) -> Result<(), Error> {
    let f_in = File::open(src).sfw_err("Couldn't open source")?;
    let mut f_in_iter = BytesIter::new(f_in, DEFAULT_BUF_SIZE);
    let mut f_out = File::create(&dst)
        .sfw_err(&*format!("Couldn't open destination: {}", &dst))?;

    f_in_iter.try_for_each(|b_slice_res| match b_slice_res {
        Ok(b_slice) => f_out.write_all(&b_slice),
        Err(err) => Err(err),
    })
}
