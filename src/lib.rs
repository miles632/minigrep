use std::error::Error;
use std::fs;
use std::env;


pub fn run(config_param: Config) -> Result<(), Box<dyn Error>>
{
    let contents = fs::read_to_string(config_param.file_path)?;

    let _results = if config_param.ignore_case
    {
        search_case_insensitive(&config_param.query, &contents);
    } else {
        search(&config_param.query, &contents);
    };

    for (line, line_num) in search(&config_param.query, &contents)
    {
        println!("LINE: {line}, {line_num}");
    }

    Ok(())
}

pub struct Config 
{
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
    pub current_dir: bool,
}

impl Config
{
    pub fn parse_config(mut args_iterator: impl Iterator<Item = String>) 
        -> Result<Config, &'static str>
    {
        args_iterator.next();
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
        let current_dir = env::current_dir().is_ok();

        Ok  (Config 
                {query,
                file_path,
                ignore_case,
                current_dir,}
            )
    }
}

// pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str>
pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<(usize, &'a str)>
{
    // contents.lines().filter(|line| line.contains(query)).collect()
    let mut matching_lines = Vec::new();
    let mut line_num = 1;

    for line in contents.lines(){
        if line.contains(query){
            matching_lines.push((line_num, line));
        }
        line_num += 1
    }

    matching_lines
}

pub fn search_case_insensitive<'a>(query: &'a str, contents: &'a str) 
            ->Vec<(usize, &'a str)>
{
    let query = query.to_lowercase();
    let mut matching_lines = Vec::new();
    let mut line_num = 1;

    for line in contents.lines(){
        if line.to_lowercase().contains(&query){
            matching_lines.push((line_num, line));
        }
        line_num += 1 
    }

    matching_lines
}

