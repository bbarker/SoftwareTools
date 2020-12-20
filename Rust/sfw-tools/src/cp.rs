#![deny(unused_must_use)]

use std::env;
use sfwtools::cp;

fn main() -> () {
    let mut args = env::args();
    args.next();
    let src = args.next().expect("cp: missing source");
    let dst = args.next().expect("cp: missing destination");
    match cp(src, dst) {
        Ok(_) => (),
        Err(err) => panic!("Failure in cp: {}", err),
    }
}
