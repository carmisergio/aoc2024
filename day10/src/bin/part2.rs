use std::{
    io::{stdin, BufReader},
    process,
};

use day10::{parse_input, Position};

fn main() {
    // Read input
    let map = parse_input(BufReader::new(stdin())).unwrap_or_else(|e| {
        eprintln!("Error reading input: {}", e);
        process::exit(1);
    });

    // Compute result
    let result = compute_result(&map);

    // Print result
    println!("Result: {}", result);
}

fn compute_result(map: &Vec<Vec<u8>>) -> usize {
    let mut sum = 0;

    for (y, row) in map.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            if map[y][x] == 0 {
                sum += find_peaks(map, Position::new_usize(x, y));
            }
        }
    }

    sum
}

fn find_peaks(map: &Vec<Vec<u8>>, pos: Position) -> usize {
    // Check position
    if let Some(cell) = get_map_cell(&map, pos) {
        if cell == 9 {
            return 1;
        }

        let positions = [
            Position::new(pos.x + 1, pos.y),
            Position::new(pos.x - 1, pos.y),
            Position::new(pos.x, pos.y + 1),
            Position::new(pos.x, pos.y - 1),
        ];

        let mut sum = 0;
        for newpos in positions {
            if let Some(newcell) = get_map_cell(map, newpos) {
                if newcell == cell + 1 {
                    sum += find_peaks(map, newpos);
                }
            }
        }
        return sum;
    }
    return 0;
}

fn get_map_cell(map: &Vec<Vec<u8>>, pos: Position) -> Option<u8> {
    if pos.x < 0 || pos.x >= map.len() as isize || pos.y < 0 || pos.y >= map[0].len() as isize {
        None
    } else {
        Some(map[pos.y as usize][pos.x as usize])
    }
}
