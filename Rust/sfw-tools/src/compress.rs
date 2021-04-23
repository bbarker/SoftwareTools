#![deny(unused_must_use)]

use sfwtools::compression::*;
use sfwtools::run_app;
use std::env;

fn main() {
    let app_name: String = String::from("compress");
    let mut mod_args = env::args().collect::<Vec<String>>();
    mod_args.insert(1, app_name.clone());
    run_app(compress_app(), mod_args, &app_name)
}
