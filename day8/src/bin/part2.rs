use std::{
    collections::{HashMap, HashSet},
    io::{stdin, BufReader},
    process,
};

use day8::{gcd, parse_input, Antenna, Pairs, ParseResult, Position};

fn main() {
    // Parse input
    let input = parse_input(BufReader::new(stdin())).unwrap_or_else(|e| {
        eprintln!("Error reading input: {}", e);
        process::exit(1);
    });

    // Compute solution
    let result = compute_solution(input);

    // Print result
    println!("Result: {}", result);
}

fn compute_solution(input: ParseResult) -> usize {
    // Divide antennas into groups of frequencies
    let buckets = separate_antennas_by_frequency(&input.antennas);

    // Keep track of unique antinode positions
    let mut antinodes = HashSet::<Position>::new();

    // Find antinodes
    for (_, antennas) in buckets {
        for (a, b) in Pairs::new(&antennas) {
            antinodes.extend(find_antinodes(a, b, input.board_width, input.board_height).iter());
        }
    }

    // Count only antinodes within the board bounds
    antinodes.len()
}

// Separate antennas into buckets based on frequency
fn separate_antennas_by_frequency(antennas: &[Antenna]) -> HashMap<char, Vec<Position>> {
    let mut result: HashMap<char, Vec<Position>> = HashMap::new();

    for antenna in antennas {
        let bucket = result.get_mut(&antenna.frequency);

        if let Some(bucket) = bucket {
            bucket.push(antenna.position);
        } else {
            result.insert(antenna.frequency, vec![antenna.position]);
        }
    }

    result
}

// Find antinodes produced by a pair of antennas
fn find_antinodes(
    a: Position,
    b: Position,
    board_width: usize,
    board_height: usize,
) -> Vec<Position> {
    let mut antinodes = Vec::new();

    // Compute distance deltas
    let dist_x = b.x - a.x;
    let dist_y = b.y - a.y;

    // Compute CCD
    let gcd = gcd(dist_x, dist_y);

    // Find deltas
    let delta_x = dist_x / gcd;
    let delta_y = dist_y / gcd;

    // Forward direction
    antinodes.extend((0..).map_while(|i| {
        // Compute new position
        let x = a.x + delta_x * i;
        let y = a.y + delta_y * i;

        // Check if position is in bounds
        if x >= 0 && x < board_width as isize && y >= 0 && y < board_height as isize {
            Some(Position::new(x, y))
        } else {
            None
        }
    }));

    // Backward direction
    antinodes.extend((1..).map_while(|i| {
        // Compute new position
        let x = a.x - delta_x * i;
        let y = a.y - delta_y * i;

        // Check if position is in bounds
        if x >= 0 && x < board_width as isize && y >= 0 && y < board_height as isize {
            Some(Position::new(x, y))
        } else {
            None
        }
    }));

    antinodes
}
