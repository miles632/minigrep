mod testing;

use std::env;
use std::process;
use std::fs;

use minigrep::Config;

fn main()
{
    // stores command line args
    let _args: Vec<String> = env::args().collect();

    for i in _args{
        println!("{}", i)
    }

    let config1 = 
    Config::parse_config(env::args()).unwrap_or_else(|_err| 
    {
        eprintln!("Problem parsing arguments {_err}");
        process::exit(1);
    });

    let _contents = match fs::read_to_string(&config1.file_path){
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("Error reading file: {}", err);
            process::exit(1);
        }
    };


    if let Err(e) =  minigrep::run(config1)
    {
        eprintln!("Encountered runtime error: {e}");
        process::exit(1);
    }
}