use std::env;
use std::error::Error;
use std::fs;
use std::io;
/*
    TODO:
        -Add coloring for the matched queries in the output
        -Add coloring for line numbers in the output -- DONE!
        -Make the search functions multi threaded
        -Fix the whole god damn run-system and make it be able to run from a
        single command
        -Unfuck the recursive search function
 */

macro_rules! print_matching_lines {
    // case recursive search

    ($query:expr, $path:expr, case_recursive) => {
        use std::fmt::Write;

        //println!("RUNNING CASE RECURSIVE MACRO");
        for (file_name, line_num, line) in search_recursive($query, $path) {
            println!("file_name: {}, {}: {}", file_name, line_num, line)
        }
    };
    // case insensitive
    ($query:expr, $path:expr, $contents:expr, case_insensitive) => {
        use regex::Regex;
        use std::fmt::Write;

        let query_regex = Regex::new($query).unwrap();

        for (line_num, line) in search_case_insensitive($query, $contents) {
            // format the line_num with ANSII escape sequences
            let colored_line_num = format!("\x1b[32m{}\x1b[0m", line_num);
            println!("{}: {}", colored_line_num, line);
        }
    };
    // case sensitive search
    ($query:expr, $path:expr, $contents:expr, case_sensitive) => {
        use std::fmt::Write;

        let _contents = fs::read_to_string(&$path)?;
        for (line_num, line) in search($query, $contents) {
            let colored_line_num = format!("\x1b[32m{}\x1b[0m", line_num);
            println!("{}: {}", colored_line_num, line);
        }
    };
    // default
    ($query:expr, $path:expr, $contents:expr, $default:ident) => {
        eprintln!("Invalid argument passed to macro");
    };
}

pub fn run(config_struct: Config) -> Result<(), Box<dyn Error>> {
    /*=== INSENSITIVE CASE ===*/
    match config_struct {
        Config {
            ignore_case: true, ..
        } => {
            println!("Running case insensitive search");
            let contents = fs::read_to_string(&config_struct.file_path)?;
            search_case_insensitive(&config_struct.query, &contents);
            print_matching_lines!(
                &config_struct.query,
                &config_struct.file_path,
                &contents,
                case_insensitive
            );
            Ok(())
        }

        /*=== RECURSIVE CASE ===*/
        Config {
            case_recursive: true,
            ..
        } => {
            //println!("recursive case : dbg");
            let _matches = search_recursive(&config_struct.query, &config_struct.file_path);
            print_matching_lines!(
                &config_struct.query,
                &config_struct.file_path,
                case_recursive
            );
            Ok(())
        }

        // SENSITIVE CASE
        Config {
            ignore_case: false,
            case_recursive: false,
            ..
        } => {
            let contents = fs::read_to_string(&config_struct.file_path)?;
            search(&config_struct.query, &contents);
            //println!("Runnning case sensitive function");
            print_matching_lines!(
                &config_struct.query,
                &config_struct.file_path,
                &contents,
                case_sensitive
            );
            Ok(())
        }
        // this probably won't be necessary but oh well!
        // _ =>    { eprintln!("No specific case has been set");
        //           Err(())
        //         }
    }
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
    //pub current_dir: bool,
    pub case_recursive: bool,
}

impl Config {
    pub fn parse_config(
        mut args_iterator: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        args_iterator.next();
        let query = match args_iterator.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args_iterator.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a fucking file path string"),
        };

        let case_recursive = env::var("RECURSIVE_CASE").is_ok();
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        //let current_dir = env::current_dir().is_ok();
        Ok(Config {
            query,
            file_path,
            ignore_case,
            //current_dir,
            case_recursive,
        })
    }
}

pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<(usize, &'a str)> {
    let mut matching_lines = Vec::new();
    let mut line_num = 1;

    for line in contents.lines() {
        if line.contains(query) {
            // line_num and line are returned in a touple vector

            matching_lines.push((line_num, line));
        }
        line_num += 1
    }

    matching_lines
}

pub fn search_case_insensitive<'a>(query: &'a str, contents: &'a str) -> Vec<(usize, &'a str)> {
    let query = query.to_lowercase();
    let mut matching_lines = Vec::new();
    let mut line_num = 1;

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            matching_lines.push((line_num, line));
        }
        line_num += 1
    }

    matching_lines
}

pub fn search_recursive(query: &str, path: &str) -> Vec<(String, usize, String)> {
    let mut matching_lines = Vec::new();
    let query = query.to_lowercase();
    println!("Searching in path: {}\n", &path);

    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        println!(
            "Searching in entry: {}\n",
            &entry.file_name().to_string_lossy()
        );
        let path = entry.path();
        if path.is_dir() {
            let sub_matches = search_recursive(query.as_str(), path.to_str().unwrap());
            matching_lines.extend(sub_matches);
        } else {
            match fs::read_to_string(&path) {
                Ok(file) => {
                    println!(
                        "Reading from file: {} in path: {}",
                        path.file_name().unwrap().to_str().unwrap(),
                        &path.display()
                    );
                    for (line_num, line) in file.lines().enumerate() {
                        if line.to_lowercase().contains(&query) {
                            matching_lines.push((
                                path.to_str().unwrap().to_string(),
                                line_num + 1,
                                line.to_string(),
                            ));
                        }
                    }
                }
                Err(e) if e.kind() == io::ErrorKind::InvalidData => {
                    println!(
                        "Skipping file: {} due to invalid UTF-8 encoding",
                        path.display()
                    );
                }
                Err(_e) => {
                    panic!("Failed to read file {}", &path.display());
                }
            }
        }
    }
    matching_lines
}
