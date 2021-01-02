#![deny(unused_must_use)]

use sfwtools::counting::*;
use sfwtools::SfwRes;
use std::env;

fn main() {
    let mut args = env::args();
    args.next();
    let src = args.next().user_err("cp: missing source");
    run_wc_lines(&src);
}
