extern crate exitcode;

use log::{error, info};
use std::process;

use procon::run;

fn main() {
    let message = run().unwrap_or_else(|err| {
        error!("{}", err.to_string());
        process::exit(exitcode::CONFIG);
    });
    info!("{}", message);
}
