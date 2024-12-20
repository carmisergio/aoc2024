use std::{
    io::{stdin, BufReader},
    process,
};

#[cfg(feature = "prod")]
mod config {
    pub const WIDTH: usize = 71;
    pub const HEIGHT: usize = 71;
    pub const MAX_BYTES: usize = 1024;
}
#[cfg(not(feature = "prod"))]
mod config {
    pub const WIDTH: usize = 7;
    pub const HEIGHT: usize = 7;
    pub const MAX_BYTES: usize = 12;
}

use day18::{calculate_distance, construct_map, parse_input, Position};

fn main() {
    // Read input
    let bytes = parse_input(BufReader::new(stdin())).unwrap_or_else(|e| {
        eprintln!("Error reading input: {}", e);
        process::exit(1);
    });

    // Simulate bytes falling
    let map = construct_map(&bytes[0..config::MAX_BYTES], config::WIDTH, config::HEIGHT);

    // Calculate distance
    let start = Position::new(0, 0);
    let end = Position::new_usize(config::WIDTH - 1, config::HEIGHT - 1);
    let res = calculate_distance(&map, start, end).unwrap_or_else(|| {
        eprintln!("Couldn't reach end!");
        process::exit(1);
    });

    // Print result
    println!("Result: {}", res);
}
