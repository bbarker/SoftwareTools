#![deny(unused_must_use)]

use sfwtools::counting::*;
use sfwtools::SfwRes;
use std::env;

fn main() {
    let mut args = env::args();
    args.next();
    let src = args.next().user_err("wc: missing source");
    run_wc_lines(&src);

    // TODO: need a safe wrapper because seahorse expects at least one
    //       argument, otherwise fails with a nasty error
    wc_app().run([String::from("foo")].to_vec())
}
