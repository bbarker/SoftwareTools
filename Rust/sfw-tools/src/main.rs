#![deny(unused_must_use)]

use sfwtools::copying::run_cp_seahorse_cmd;
use sfwtools::counting::run_wc_seahorse_cmd;
use sfwtools::tabs::{run_detab_seahorse_cmd, run_entab_seahorse_cmd};
use sfwtools::{run_app, run_echo_seahorse_cmd};
use std::env;

use seahorse::App;

fn main() {
    let app_name: String = String::from("sfwtools");
    let app = App::new(app_name.clone())
        .author("Brandon Elam Barker")
        .command(run_cp_seahorse_cmd())
        .command(run_wc_seahorse_cmd())
        .command(run_detab_seahorse_cmd())
        .command(run_entab_seahorse_cmd())
        .command(run_echo_seahorse_cmd());
    run_app(app, env::args().collect(), &app_name)
}
