use std::{
    io::{stdin, BufReader},
    process,
};

use day11::{parse_input, process};

fn main() {
    let input = parse_input(BufReader::new(stdin())).unwrap_or_else(|e| {
        eprintln!("Error reading input: {}", e);
        process::exit(1);
    });

    let res = process(&input, 25);

    println!("Result: {}", res);
}
