use std::{
    io::{stdin, BufReader},
    process,
};

use day15::{get_map_cell, parse_input, set_map_cell, Cell, Move, Position};

fn main() {
    // Read input
    let (mut board, moves, mut robot_pos) =
        parse_input(BufReader::new(stdin())).unwrap_or_else(|e| {
            eprintln!("Error reading input: {}", e);
            process::exit(1);
        });

    // Apply all moves
    for m in moves {
        apply_move(&mut board, &mut robot_pos, m);
    }

    println!("Result: {}", compute_gps_sum(&board));
}

fn apply_move(board: &mut Vec<Vec<Cell>>, robot_pos: &mut Position, m: Move) {
    // Count boxes that have to be moved
    let n_boxes = match count_boxes_to_move(board, *robot_pos, m) {
        Some(n) => n,
        None => return,
    };

    // Move boxes
    move_boxes(board, *robot_pos, m, n_boxes);

    // Move robot
    robot_pos.apply_move(m);
}

fn count_boxes_to_move(board: &mut Vec<Vec<Cell>>, robot_pos: Position, m: Move) -> Option<usize> {
    let mut cursor = robot_pos;
    for i in 0.. {
        // Move cursor
        cursor.apply_move(m);

        if let Some(cell) = get_map_cell(board, cursor) {
            match cell {
                Cell::Wall => break,
                Cell::Empty => return Some(i),
                _ => {}
            }
        }
    }
    None
}

fn move_boxes(board: &mut Vec<Vec<Cell>>, robot_pos: Position, m: Move, n_boxes: usize) {
    let mut cursor = robot_pos;

    // Move cursor to end
    for _ in 0..n_boxes {
        cursor.apply_move(m);
    }

    // Go backward moving boxes
    for _ in 0..n_boxes {
        let val = get_map_cell(board, cursor);
        if let Some(val) = val {
            set_map_cell(board, cursor.moved(m), val);
            set_map_cell(board, cursor, Cell::Empty);
        }
        cursor.apply_move(m.reverse());
    }
}

fn compute_gps_sum(board: &Vec<Vec<Cell>>) -> usize {
    let mut sum = 0;

    for (y, row) in board.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if let Cell::Box = cell {
                sum += 100 * y + x;
            }
        }
    }

    sum
}
