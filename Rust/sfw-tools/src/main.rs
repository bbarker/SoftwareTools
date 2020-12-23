#![deny(unused_must_use)]

use sfwtools::cp;
use std::collections::VecDeque;
use std::env;

fn main() -> () {
    let (cmd, args) = get_args();
    print!("cmd = {}, args = {:?}", cmd, args);

    // let src = args.next().expect("cp: missing source");
    // let dst = args.next().expect("cp: missing destination");
    // match cp(src, dst) {
    //     Ok(_) => (),
    //     Err(err) => panic!("Failure in cp: {}", err),
    // }
}

fn get_args() -> (String, VecDeque<String>) {
    let mut args_in: VecDeque<String> = env::args().collect();
    let cmd = args_in.pop_front().expect("0 main args, Impossible!");
    (cmd, args_in)
}