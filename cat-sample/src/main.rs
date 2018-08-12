extern crate getopts;

use std::env;
use std::io::{BufRead, BufReader};
use std::fs::File;
use getopts::Options;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 1 {
        return
    }

    let mut options = Options::new();
    options.optflag("n", "number", "number all output lines");
    let matches = match options.parse(&args[1..]) {
        Ok(m) => { m }
        Err(e) => { panic!(e.to_string()) }
    };
    let has_n_option = matches.opt_present("n");

    let args: Vec<String> = env::args().filter(|arg| !arg.starts_with("-")).collect();
    if args.len() < 1 {
        return
    }

    let mut row_number = 0;
    for i in 1..args.len() {
        let file = match File::open(&args[i]) {
            Ok(f) => { f }
            Err(e) => { panic!(e.to_string()) }
        };
        for line in BufReader::new(file).lines() {
            let line = match line {
                Ok(l) => { l }
                Err(e) => { panic!(e.to_string()) }
            };
            let head = if has_n_option { row_number.to_string() + "\t" } else { "".to_string() };
            println!("{}{}", head, line);
            row_number += 1;
        }
    }
}
