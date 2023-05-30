use std::env;
use std::process;
//use std::fs;

use libs::Config;

fn main() {
    // stores command line args
    let _args: Vec<String> = env::args().collect();

    let config1 = Config::parse_config(env::args()).unwrap_or_else(|_err| {
        eprintln!("Problem parsing arguments: {_err}");
        process::exit(1);
    });

    if let Err(e) = libs::run(config1) {
        eprintln!("Encountered runtime error: {e}");
        process::exit(1);
    }
}
