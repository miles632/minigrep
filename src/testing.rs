#[cfg(test)]
mod tests
{
    use minigrep::search;
    use minigrep::search_case_insensitive;

    use super::*;

    #[test]
    fn case_sensitive()
    {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
    assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    fn case_insensitive()
    {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape."; 

        assert_eq!(vec!["Rust:", "Trust me."],
                search_case_insensitive(query, contents));
    }
}
