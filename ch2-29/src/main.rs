use std::fs::File; 
use std::io::BufReader;
use std::io::prelude::*;
use regex::Regex;
use clap::{App, Arg};

fn main() {

    let args = App::new("grep-lite")
        .version("0.1")
        .about("searches for patterns")
        .arg(Arg::with_name("pattern")
                .help("The pattern to search for")
                .takes_value(true)
                .required(true))
        .arg(Arg::with_name("input")
                .help("File to search")
                .takes_value(true)
                .required(true))
        .get_matches();
    
    let pattern = args.value_of("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();

    let input = args.value_of("input").unwrap();

    if input == "-" {
        let stdin = std::io::stdin(); 
        let reader = stdin.lock(); 
        process_lines(reader, re);
    } else {
        let f = File::open(input).unwrap();
        let reader = BufReader::new(f);
        process_lines(reader, re);
    }
}

fn process_lines<T: BufRead + Sized>(reader: T, re: Regex) {
    for line_ in reader.lines().enumerate() {
        let (line_no, line_result)= line_;
        let line = line_result.unwrap();



        match re.find(&line) {
            Some(_) => println!("{}: {}", line_no+1, line),
            None => (),
        } 
    }
}