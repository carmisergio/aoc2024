use std::{
    io::{stdin, BufReader},
    process,
};

use day20::{find_best_path, find_cheats, parse_input};

fn main() {
    // Read input
    let (matrix, start_pos, end_pos) = parse_input(BufReader::new(stdin())).unwrap_or_else(|e| {
        eprintln!("Error reading input: {}", e);
        process::exit(1);
    });

    // Find best path
    let best_path = find_best_path(&matrix, start_pos, end_pos).unwrap_or_else(|| {
        eprintln!("Destination unreachable!");
        process::exit(1);
    });

    // Print result
    println!("Result: {}", find_cheats(&best_path, 20, 100));
}
