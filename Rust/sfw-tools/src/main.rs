#![deny(unused_must_use)]

use sfwtools::cp;
use std::env;
use std::io::{Error, ErrorKind::InvalidInput};
use std::process;

fn main() -> () {
    let (cmd, args) = get_args().unwrap_or_else(|err| {
        println!("Argument error: {}", err);
        process::exit(1)
    });
    print!("cmd = {}, args = {:?}", cmd, args);

    // let src = args.next().expect("cp: missing source");
    // let dst = args.next().expect("cp: missing destination");
    // match cp(src, dst) {
    //     Ok(_) => (),
    //     Err(err) => panic!("Failure in cp: {}", err),
    // }
}

fn get_args() -> Result<(String, Vec<String>), Error> {
    let mut args_in = env::args();
    let cmd = args_in
        .next()
        .ok_or_else(|| Error::new(InvalidInput, "Impossible: no first arg!"))?;
    let args_out: Vec<String> = args_in.collect::<Vec<String>>();
    Ok((cmd, args_out))
}
