use std::env;
use std::fs::File;
use std::io::Write;

use read_byte_slice::{ByteSliceIter, FallibleStreamingIterator};

fn main() -> () {
    let mut args = env::args();
    args.next();
    let src = args.next().expect("cp: missing source");
    let dst = args.next().expect("cp: missing destination");
    cp(src, dst);
}

fn cp(src: String, dst: String) -> () {
    let f_in =
        File::open(&src).expect(&format!("Couldn't open source: {}", &src));

    let mut f_in_iter = ByteSliceIter::new(f_in, 4096);
    let mut f_out = File::create(&dst)
        .expect(&format!("Couldn't open destination: {}", &dst));

    loop {
        match f_in_iter.next() {
            Ok(Some(b_slice)) => {f_out
                .write(b_slice)
                .expect(&format!("Failure writing to {}.", &dst));}
            Ok(None) => {break;}
            Err(err) => panic!("Failure reading from {}: {}.", &src, err),
        }
    }
    return ();
}
