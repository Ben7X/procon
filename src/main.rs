extern crate exitcode;

use clap::CommandFactory;
use log::{error, info};
use procon::args::Args;
use std::process;

use procon::run;

fn main() {
    let message = run().unwrap_or_else(|err| {
        error!("{}", err.to_string());
        Args::command().print_help().unwrap();
        process::exit(exitcode::CONFIG);
    });
    info!("{}", message);
}
