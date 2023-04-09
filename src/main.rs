mod testing;

use std::env;
use std::process;
use std::fs;

use minigrep::Config;

fn main()
{
    // stores command line args
    let args: Vec<String> = env::args().collect();

    let config1 = Config::parse_config(env::args()).unwrap_or_else(|_err| {
        eprintln!("Problem parsing arguments {_err}");
        process::exit(1);
    });

    let contents = fs::read_to_string(&config1.file_path)
        .expect("Should have been able to read the file");


    if let Err(e) =  minigrep::run(config1)
    {
        eprintln!("Encountered runtime error: {e}");
        process::exit(1);
    }
}