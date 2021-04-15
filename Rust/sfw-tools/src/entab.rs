#![deny(unused_must_use)]

use sfwtools::run_app;
use sfwtools::tabs::*;
use std::env;

fn main() {
    let app_name: String = String::from("entab");
    let mut mod_args = env::args().collect::<Vec<String>>();
    mod_args.insert(1, app_name.clone());
    run_app(entab_app(), mod_args, &app_name)
}
