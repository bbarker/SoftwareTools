#![deny(unused_must_use)]

use sfwtools::{exit, get_args, run_cp, SfwRes};
use std::ffi::OsStr;
use std::path::Path;

fn main() {
    let (cmd, args) = get_args().user_err("Argument error");
    let cmd_path = Path::new(&cmd);

    match cmd_path.file_name().and_then(OsStr::to_str) {
        Some("cp") => {
            let src = args.get(0).user_err("cp: missing source");
            let dst = args.get(1).user_err("cp: missing destination");
            run_cp(&src, &dst);
        }
        Some(u) => exit(&*format!("Unknown sfwtools command: {}", u)),
        None => exit("No command passed to sfwtools, exiting."),
    }

    // let src = args.next().expect("cp: missing source");
    // let dst = args.next().expect("cp: missing destination");
    // match cp(src, dst) {
    //     Ok(_) => (),
    //     Err(err) => panic!("Failure in cp: {}", err),
    // }
}
