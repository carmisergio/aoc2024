use std::{
    collections::{HashMap, HashSet},
    io::{stdin, BufReader},
    process,
};

use day8::{parse_input, Antenna, Pairs, ParseResult, Position};

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
            antinodes.extend(find_antinodes(a, b).iter());
        }
    }

    // Count only antinodes within the board bounds
    antinodes
        .iter()
        .filter(|pos| {
            pos.x >= 0
                && pos.x < input.board_width as isize
                && pos.y >= 0
                && pos.y < input.board_height as isize
        })
        .count()
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
fn find_antinodes(a: Position, b: Position) -> Vec<Position> {
    let mut antinodes = Vec::new();

    // Compute distance deltas
    let delta_x = b.x - a.x;
    let delta_y = b.y - a.y;

    // Add external antinodes
    antinodes.push(Position::new(a.x - delta_x, a.y - delta_y));
    antinodes.push(Position::new(b.x + delta_x, b.y + delta_y));

    // Add internal antinodes
    if delta_x % 3 == 0 && delta_y % 3 == 0 {
        let delta_x = delta_x / 3;
        let delta_y = delta_y / 3;
        antinodes.push(Position::new(a.x + delta_x, a.y + delta_y));
        antinodes.push(Position::new(b.x - delta_x, b.y - delta_y));
    }

    antinodes
}
