extern crate exitcode;

use std::process;

use procon::run;

fn main() {
    run().unwrap_or_else(|err| {
        eprintln!("{}", err.to_string());
        process::exit(exitcode::CONFIG);
    });
}
