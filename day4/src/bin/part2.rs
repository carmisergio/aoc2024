use std::process;

use day4::{next_position, read_board, Direction};

fn main() {
    // Read board
    let board = read_board().unwrap_or_else(|e| {
        eprintln!("Read error: {}", e);
        process::exit(1);
    });

    let count = count_occurrences(&board);

    println!("Number of occurrences: {}", count);
}

fn count_occurrences(board: &[Vec<char>]) -> usize {
    let mut count = 0;

    // Iterate over all start positions
    for y in 0..board.len() {
        for x in 0..board[0].len() {
            if let Ok(_) = check_startposition(board, (x, y)) {
                count += 1;
            }
        }
    }

    count
}

fn check_startposition(board: &[Vec<char>], pos: (usize, usize)) -> Result<(), ()> {
    // Check if current character is 'A'
    let (x, y) = &pos;
    if board[*y][*x] != 'A' {
        return Err(());
    }

    // Directions to check
    let major_poss = [
        vec![(Direction::DiagMajorBw, 'M'), (Direction::DiagMajorFw, 'S')],
        vec![(Direction::DiagMajorBw, 'S'), (Direction::DiagMajorFw, 'M')],
    ];

    let minor_poss = [
        vec![(Direction::DiagMinorBw, 'M'), (Direction::DiagMinorFw, 'S')],
        vec![(Direction::DiagMinorBw, 'S'), (Direction::DiagMinorFw, 'M')],
    ];

    // Check major diagonal possibilities
    check_possibilities(board, pos, &major_poss)?;

    // Check minor diagonal possibilities
    check_possibilities(board, pos, &minor_poss[..])?;

    Ok(())
}

fn check_possibilities(
    board: &[Vec<char>],
    pos: (usize, usize),
    possibilities: &[Vec<(Direction, char)>],
) -> Result<(), ()> {
    for p in possibilities {
        if let Ok(_) = check_possibility(board, pos, p) {
            return Ok(());
        }
    }

    Err(())
}

fn check_possibility(
    board: &[Vec<char>],
    pos: (usize, usize),
    possibility: &[(Direction, char)],
) -> Result<(), ()> {
    for (dir, c) in possibility {
        // Compute position
        let (x, y) = next_position(pos, dir, board.len(), board[0].len())?;

        // Check character
        if board[y][x] != *c {
            return Err(());
        }
    }

    Ok(())
}
