#![deny(unused_must_use)]

use sfwtools::echo_app;
use std::env;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    echo_app().run(args)
}
