use std::{
    cmp::Ordering,
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
            let pos = Position::new_usize(x, y);

            // Ignore visited cells
            if *get_map_cell_unchecked(&visited, pos) {
                continue;
            }

            let mut v_edges = Vec::new();
            let mut h_edges = Vec::new();

            let area = explore_area(
                &plots,
                &mut visited,
                &mut v_edges,
                &mut h_edges,
                Position::new_usize(x, y),
            );

            let edges = count_unique_edges(&mut v_edges) + count_unique_edges(&mut h_edges);

            // dbg!(plots[y][x]);
            // dbg!(edges);
            // dbg!(area);
            // dbg!(&h_edges);
            // dbg!(&v_edges);
            sum += edges * area;
        }
    }

    sum
}

fn explore_area(
    map: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    v_edges: &mut Vec<(isize, isize)>,
    h_edges: &mut Vec<(isize, isize)>,
    pos: Position,
) -> u64 {
    if *get_map_cell_unchecked(visited, pos) {
        // Already visited cells don't produce any area
        return 0;
    }

    // Set cell as visited
    visited[pos.y as usize][pos.x as usize] = true;

    //  Get current cell value
    let cell = *get_map_cell_unchecked(map, pos);

    let mut area = 1;

    //// Process neighbours

    // Top
    if let Some(a) = explore_neighbour(
        map,
        visited,
        v_edges,
        h_edges,
        Position::new(pos.x, pos.y - 1),
        cell,
    ) {
        area += a;
    } else {
        h_edges.push((pos.y, pos.x));
    }

    // Bottom
    if let Some(a) = explore_neighbour(
        map,
        visited,
        v_edges,
        h_edges,
        Position::new(pos.x, pos.y + 1),
        cell,
    ) {
        area += a;
    } else {
        h_edges.push(((pos.y + 1) + 1000, pos.x));
    }

    // Left
    if let Some(a) = explore_neighbour(
        map,
        visited,
        v_edges,
        h_edges,
        Position::new(pos.x - 1, pos.y),
        cell,
    ) {
        area += a;
    } else {
        v_edges.push((pos.x, pos.y));
    }

    // Right
    if let Some(a) = explore_neighbour(
        map,
        visited,
        v_edges,
        h_edges,
        Position::new(pos.x + 1, pos.y),
        cell,
    ) {
        area += a;
    } else {
        v_edges.push(((pos.x + 1) + 1000, pos.y));
    }

    area
}

// Returns none if the neighbour is an edge
fn explore_neighbour(
    map: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    v_edges: &mut Vec<(isize, isize)>,
    h_edges: &mut Vec<(isize, isize)>,
    pos: Position,
    cell: char,
) -> Option<u64> {
    if let Some(neighbour) = get_map_cell(map, pos) {
        if *neighbour == cell {
            return Some(explore_area(map, visited, v_edges, h_edges, pos));
        }
    }

    None
}

fn count_unique_edges(edges: &mut [(isize, isize)]) -> u64 {
    edges.sort_by(|(a_group, a_cell), (b_group, b_cell)| {
        if a_group == b_group {
            if a_cell > b_cell {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        } else {
            if a_group > b_group {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        }
    });

    let mut count = 0;
    let mut last_pos: Option<(isize, isize)> = None;
    for (group, cell) in edges {
        if let Some((last_group, last_cell)) = last_pos {
            if *group != last_group || *cell != last_cell + 1 {
                // New edge!
                count += 1;
            }
        }

        last_pos = Some((*group, *cell));
    }

    if let Some(_) = last_pos {
        count += 1;
    }

    count
}
