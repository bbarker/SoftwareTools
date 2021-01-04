#![deny(unused_must_use)]

use sfwtools::counting::*;
use sfwtools::run_app;
use std::env;

fn main() {
    run_app(wc_app(), env::args().collect(), "wc")
}
