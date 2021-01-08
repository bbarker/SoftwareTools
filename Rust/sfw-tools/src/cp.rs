#![deny(unused_must_use)]

use sfwtools::copying::*;
use sfwtools::run_app;
use std::env;

fn main() {
    let app_name: String = String::from("cp");
    let mut mod_args = env::args().collect::<Vec<String>>();
    mod_args.insert(1, app_name.clone());
    run_app(cp_app(), mod_args, &app_name)
}
