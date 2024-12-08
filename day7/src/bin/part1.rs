use std::{
    io::{stdin, BufReader},
    process,
};

use day7::parse_input;

fn main() {
    let equations = parse_input(BufReader::new(stdin())).unwrap_or_else(|e| {
        eprintln!("Read error: {}", e);
        process::exit(1);
    });

    let mut sum = 0;
    for (target, mut factors) in equations {
        factors.reverse();
        if is_solvable(target, &factors) {
            sum += target;
        }
    }

    println!("Result: {}", sum);
}

fn is_solvable(target: u64, factors: &[u64]) -> bool {
    // Empty target is an error
    if factors.len() < 1 {
        return false;
    }

    // Done!
    if factors.len() == 1 {
        return target == factors[0];
    }

    let next = factors[0];
    let factors = &factors[1..];

    // Addition
    if target >= next && is_solvable(target - next, factors) {
        return true;
    }

    // Multiplication
    if target % next == 0 && is_solvable(target / next, factors) {
        return true;
    }

    return false;
}
