use std::error::Error;
use std::fs;
use std::env;
use std::process;


pub fn run(config_param: Config) -> Result<(), Box<dyn Error>>
{
    let contents = fs::read_to_string(config_param.file_path)?;

    let _results = if config_param.ignore_case
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
    pub fn parse_config(mut args_iterator: impl Iterator<Item = String>) 
        -> Result<Config, &'static str>
    {
        let query = match args_iterator.next()
        {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args_iterator.next()
        {
            Some(arg) => arg,
            None => return Err("Didn't get a fucking file path string"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config 
                {query,
                file_path,
                ignore_case})
    }
}

pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str>
{
    contents.lines().filter(|line| line.contains(query)).collect()
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

