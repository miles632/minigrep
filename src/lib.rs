use std::error::Error;
use std::fs;
use std::env;


pub fn run(config_param: Config) -> Result<(), Box<dyn Error>>
{
    let contents = fs::read_to_string(config_param.file_path)?;

    let results = if config_param.ignore_case
    {
        search_case_insensitive(&config_param.query, &contents);
    } else {
        search(&config_param.query, &contents);
    };

    for line in search(&config_param.query, &contents)
    {
        println!("{line}");
    }

    Ok(())
}

pub struct Config 
{
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config
{
    pub fn parse_config(args: &[String]) -> Result<Config, &'static str>
    {
        if args.len() < 3 
        {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config 
                {query,
                file_path,
                ignore_case})
    }
}

pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str>
{
    let mut results = Vec::new();

    for line in contents.lines()
    {
        if line.contains(query)
        {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &'a str, contents: &'a str) 
     ->Vec<&'a str>
{
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines()
    {
        if line.to_lowercase().contains(&query)
        {
            results.push(line);
        }
    }

    results
}

