#![allow(dead_code)]

use clap::{App, Arg, ArgMatches};
use std::error::Error;
use std::path::Path;
use std::process;

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run() -> MyResult<()> {
    println!("Hello, world!");
    Ok(())
}

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn parse_args() -> ArgMatches<'static> {
    let result = App::new("catr")
        .version("0.1.0")
        .about("Rust cat")
        .author("Araldo van de Kraats <a.vandekraats@gmail.com>")
        .arg(
            Arg::with_name("number")
                .short("n")
                .long("number")
                .takes_value(false)
                .help("Number lines"),
        )
        .arg(
            Arg::with_name("number-nonblank")
                .short("b")
                .long("number-nonblank")
                .help("Number non-blank lines")
                .takes_value(false)
                .conflicts_with("number"),
        )
        .arg(
            Arg::with_name("FILE")
                .required(true)
                .value_name("FILE")
                .takes_value(true)
                .multiple(true)
                .help("Input file(s)")
                .min_values(1),
        )
        .get_matches();
    result
}

pub fn check_file_exists(filename: &str) -> () {
    if !Path::new(filename).exists() {
        eprintln!("\"{}\" {}", filename, "is not a valid file.");
        process::exit(1);
    }
}

// pub fn print_line(line: &str, parsed_args: ArgMatches) -> () {
//     if parsed_args.is_present("number") {
//         println!("     {}\t{}", line_number.to_string(), line);
//         line_number += 1;
//     } else if parsed_args.is_present("number-nonblank") {
//         if line.len() > 0 {
//             println!("     {}\t{}", line_number.to_string(), line);
//             line_number += 1;
//         } else {
//             println!("")
//         }
//     } else {
//         println!("{}", line);
//     }
// }
