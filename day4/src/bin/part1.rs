use std::process;

use day4::{next_position, read_board, Direction};

fn main() {
    // Read board
    let board = read_board().unwrap_or_else(|e| {
        eprintln!("Read error: {}", e);
        process::exit(1);
    });

    let count = count_occurrences(&board, "XMAS");

    println!("Number of occurrences: {}", count);
}

fn count_occurrences(board: &[Vec<char>], string: &str) -> usize {
    let mut count = 0;

    // Iterate over all start positions
    for y in 0..board.len() {
        for x in 0..board[0].len() {
            count += check_startposition(board, (x, y), string);
        }
    }

    count
}

fn check_startposition(board: &[Vec<char>], pos: (usize, usize), string: &str) -> usize {
    // Directions to check
    let dirs = [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
        Direction::DiagMajorBw,
        Direction::DiagMajorFw,
        Direction::DiagMinorBw,
        Direction::DiagMinorFw,
    ];

    // Count valid directions
    dirs.iter().fold(0, |acc, dir| {
        if let Ok(_) = check_direction(board, pos, dir, string) {
            acc + 1
        } else {
            acc
        }
    })
}

fn check_direction(
    board: &[Vec<char>],
    (x, y): (usize, usize),
    dir: &Direction,
    string: &str,
) -> Result<(), ()> {
    let mut pos = Ok((x, y));

    // Check all characters
    for c in string.chars() {
        // Check if position is valid and extract cooridntes
        let (x, y) = pos?;

        // Check character at position
        if board[y][x] != c {
            return Err(());
        }

        // Compute next option
        pos = next_position((x, y), dir, board[0].len(), board.len());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_direction_test() {
        let board = [
            vec!['A', 'X', 'X', 'X', 'X'],
            vec!['A', 'B', 'C', 'X', 'X'],
            vec!['B', 'X', 'C', 'X', 'X'],
            vec!['C', 'X', 'X', 'B', 'A'],
            vec!['C', 'X', 'X', 'B', 'A'],
        ];

        let tests = [
            (((0, 1), Direction::Right, "ABC"), Ok(())),
            (((0, 1), Direction::Down, "ABC"), Ok(())),
            (((4, 4), Direction::DiagMajorBw, "ABC"), Ok(())),
            (((2, 2), Direction::Up, "CCX"), Ok(())),
            (((0, 1), Direction::Left, "ABC"), Err(())),
        ];

        for ((pos, dir, string), exp) in tests {
            let res = check_direction(&board, pos, &dir, string);
            assert_eq!(res, exp);
        }
    }

    #[test]
    fn check_startposition_test() {
        let board = [
            vec!['A', 'X', 'X', 'X', 'X'],
            vec!['A', 'B', 'C', 'X', 'X'],
            vec!['B', 'X', 'C', 'X', 'X'],
            vec!['C', 'X', 'X', 'B', 'A'],
            vec!['C', 'X', 'X', 'B', 'A'],
        ];

        let tests = [
            (((0, 1), "ABC"), 2),
            (((4, 4), "ABC"), 1),
            (((3, 2), "ABC"), 0),
        ];

        for ((pos, string), exp) in tests {
            let res = check_startposition(&board, pos, string);
            assert_eq!(res, exp);
        }
    }

    #[test]
    fn count_occurrences_test() {
        let board = [
            vec!['A', 'X', 'X', 'X', 'X'],
            vec!['A', 'B', 'C', 'X', 'X'],
            vec!['B', 'X', 'C', 'X', 'X'],
            vec!['C', 'X', 'X', 'B', 'A'],
            vec!['C', 'X', 'X', 'B', 'A'],
        ];

        let tests = [("ABC", 4), ("ABCX", 1)];

        for (string, exp) in tests {
            let res = count_occurrences(&board, string);
            assert_eq!(res, exp);
        }
    }
}
