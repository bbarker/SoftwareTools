#![deny(unused_must_use)]

use sfwtools::counting::*;
use sfwtools::run_app;
use std::env;


fn main() {
    let app_name: String = String::from("wc");
    let mut mod_args = env::args().collect::<Vec<String>>();
    mod_args.insert(1, app_name.clone());
    run_app(wc_app(), mod_args, &app_name)
}
