#![allow(unused_imports)]
extern crate catr;
mod lib;
use clap::{App, Arg};
use std::fs;
use std::io::{self, BufRead, Write};
use std::process;

fn main() {
    let parsed_args = catr::parse_args();
    let stdin_files: Vec<&str> = parsed_args.values_of("FILE").unwrap().collect();

    if stdin_files[0] == "-" {
        let stdin = io::stdin();
        let mut line_number = 1;
        for line in stdin.lock().lines() {
            let line = line.unwrap();
            if parsed_args.is_present("number") {
                println!("     {}\t{}", line_number.to_string(), line);
                line_number += 1;
            } else if parsed_args.is_present("number-nonblank") {
                if line.len() > 0 {
                    println!("     {}\t{}", line_number.to_string(), line);
                    line_number += 1;
                } else {
                    println!("")
                }
            } else {
                println!("{}", line);
            }
        }
    } else {
        for stdin_file in stdin_files.iter() {
            catr::check_file_exists(stdin_file);
            let input = fs::read_to_string(stdin_file).unwrap();
            let mut last_num = 0;
            for (line_number, line) in input.lines().enumerate() {
                if parsed_args.is_present("number") {
                    println!("{:6}\t{}", line_number + 1, line);
                } else if parsed_args.is_present("number-nonblank") {
                    if line.len() > 0 {
                        last_num += 1;
                        println!("{:6}\t{}", last_num, line);
                    } else {
                        println!("")
                    }
                } else {
                    println!("{}", line);
                }
            }
        }
    }
}
