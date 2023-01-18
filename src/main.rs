extern crate exitcode;

use std::process;

use procon::run;

fn main() {
    let message = run().unwrap_or_else(|err| {
        eprintln!("{}", err.to_string());
        process::exit(exitcode::CONFIG);
    });
    println!("{}", message);
}
