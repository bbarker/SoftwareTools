#![deny(unused_must_use)]

use std::env;
use std::fs::File;
use std::io::Result;
use std::io::Write;

use read_byte_slice::{ByteSliceIter, FallibleStreamingIterator};

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

fn cp(src: String, dst: String) -> Result<()> {
    let f_in =
        File::open(&src).expect(&format!("Couldn't open source: {}", &src));

    let f_in_iter = ByteSliceIter::new(f_in, 4096);
    let mut f_out = File::create(&dst)
        .expect(&format!("Couldn't open destination: {}", &dst));

    f_in_iter.for_each(|b_slice| {
        f_out
            .write(b_slice)
            .expect(&format!("Failure writing to {}.", &dst));
    })
}
