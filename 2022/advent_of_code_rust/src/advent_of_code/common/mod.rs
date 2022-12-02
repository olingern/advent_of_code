use colored::Colorize;
use std::fs;

pub fn read_file(location: &str) -> String {
    fs::read_to_string(location).unwrap()
}

pub fn print_banner() {
    println!(
        "{}",
        "
    \n* ========================================= *
|                                           |
|           Advent of Code 2022             |
|                                           |
* ========================================= *\n"
            .magenta()
    );
}
