use std::{
    io::{stdin, BufReader},
    process,
};

use day12::{get_map_cell, get_map_cell_unchecked, parse_input, Position};

fn main() {
    // Read input
    let plots = parse_input(BufReader::new(stdin())).unwrap_or_else(|e| {
        eprintln!("Error reading input: {}", e);
        process::exit(1);
    });

    // Calculate result
    let result = compute_result(plots);

    println!("Result: {}", result);
}

fn compute_result(plots: Vec<Vec<char>>) -> u64 {
    if plots.len() == 0 {
        return 0;
    }

    // Visited map
    let mut visited = vec![vec![false; plots[0].len()]; plots.len()];

    let mut sum = 0;

    // Process cells
    for y in 0..plots.len() {
        for x in 0..plots[y].len() {
            let (perimeter, area) = explore_area(&plots, &mut visited, Position::new_usize(x, y));
            sum += perimeter * area;
        }
    }

    sum
}

fn explore_area(map: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, pos: Position) -> (u64, u64) {
    if *get_map_cell_unchecked(visited, pos) {
        // Already visited cells don't produce any area
        return (0, 0);
    }

    // Set cell as visited
    visited[pos.y as usize][pos.x as usize] = true;

    //  Get current cell value
    let cell = *get_map_cell_unchecked(map, pos);

    // Counters
    let mut perimeter = 0;
    let mut area = 1;

    // Process top
    let (new_perimeter, new_area) =
        explore_neighbour(map, visited, Position::new(pos.x, pos.y - 1), cell);
    perimeter += new_perimeter;
    area += new_area;

    // Process bottom
    let (new_perimeter, new_area) =
        explore_neighbour(map, visited, Position::new(pos.x, pos.y + 1), cell);
    perimeter += new_perimeter;
    area += new_area;

    // Process left
    let (new_perimeter, new_area) =
        explore_neighbour(map, visited, Position::new(pos.x - 1, pos.y), cell);
    perimeter += new_perimeter;
    area += new_area;

    // Process right
    let (new_perimeter, new_area) =
        explore_neighbour(map, visited, Position::new(pos.x + 1, pos.y), cell);
    perimeter += new_perimeter;
    area += new_area;

    (perimeter, area)
}

fn explore_neighbour(
    map: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    pos: Position,
    cell: char,
) -> (u64, u64) {
    if let Some(neighbour) = get_map_cell(map, pos) {
        if *neighbour == cell {
            return explore_area(map, visited, pos);
        }
    }

    // Cell is different or outside of the map, add perimeter
    (1, 0)
}
