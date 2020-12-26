#![deny(unused_must_use)]

use sfwtools::{cp, get_args, SfwRes};

fn main() -> () {
    let (cmd, args) = get_args().user_err("Argument error");
    print!("cmd = {}, args = {:?}", cmd, args);

    // let src = args.next().expect("cp: missing source");
    // let dst = args.next().expect("cp: missing destination");
    // match cp(src, dst) {
    //     Ok(_) => (),
    //     Err(err) => panic!("Failure in cp: {}", err),
    // }
}
