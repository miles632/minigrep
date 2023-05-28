main: This is the entry point for the program. It reads the command-line arguments, 
    builds a Config struct, reads the contents of the file, and runs the run function.

Config: This module defines the Config struct, which contains the query string, file path, 
    and whether to ignore case. It also defines a build function that takes the command-line 
        arguments and returns a Result that contains the Config struct or an error message.

run: This function takes a Config struct, reads the contents of the file, 
    searches for the query string (ignoring case if specified), and prints out the lines that contain the query.

search: This function takes a query string and a string of contents, 
    and returns a vector of strings that match the query.

search_case_insensitive: This function is similar to search, 
    but it ignores the case when searching for the query.

testing: This module contains unit tests for the search and search_case_insensitive functions.

