use std::{
    error::Error,
    io::{self, stdin, BufRead, BufReader},
    process,
};

use day15::{Move, ParseError, Position};

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

    // Calculate GPS sum
    println!("Result: {}", calculate_gps_sum(&board));
}

fn apply_move(board: &mut Vec<Vec<char>>, robot_pos: &mut Position, m: Move) {
    match m {
        Move::Up => {
            apply_move_up(board, robot_pos);
        }
        Move::Down => {
            apply_move_down(board, robot_pos);
        }
        Move::Left => {
            apply_move_left(board, robot_pos);
        }
        Move::Right => {
            apply_move_right(board, robot_pos);
        }
    }
}

fn apply_move_left(board: &mut Vec<Vec<char>>, robot_pos: &mut Position) {
    // Count boxes going left
    let mut x1 = None;
    for x in (0..robot_pos.x).rev() {
        let cell = board[robot_pos.y as usize][x as usize];
        if cell == '.' {
            x1 = Some(x);
            break;
        } else if cell == '#' {
            break;
        }
    }

    if let Some(x1) = x1 {
        // We can do the move

        // Move boxes
        for x in x1..robot_pos.x {
            board[robot_pos.y as usize][x as usize] = board[robot_pos.y as usize][(x + 1) as usize];
        }

        // Move robot
        board[robot_pos.y as usize][robot_pos.x as usize] = '.';
        robot_pos.apply_move(Move::Left);
    }
}

fn apply_move_right(board: &mut Vec<Vec<char>>, robot_pos: &mut Position) {
    // Count boxes going left
    let mut x1 = None;
    for x in ((robot_pos.x as usize) + 1)..board[0].len() {
        let cell = board[robot_pos.y as usize][x];
        if cell == '.' {
            x1 = Some(x);
            break;
        } else if cell == '#' {
            break;
        }
    }

    if let Some(x1) = x1 {
        // We can do the move

        // Move boxes
        for x in ((robot_pos.x as usize + 1)..=x1).rev() {
            board[robot_pos.y as usize][x] = board[robot_pos.y as usize][x - 1];
        }

        // Move robot
        board[robot_pos.y as usize][robot_pos.x as usize] = '.';
        robot_pos.apply_move(Move::Right);
    }
}

fn apply_move_up(board: &mut Vec<Vec<char>>, robot_pos: &mut Position) {
    if move_up(
        board,
        robot_pos.y as usize - 1,
        robot_pos.x as usize,
        robot_pos.x as usize,
    ) {
        robot_pos.apply_move(Move::Up);
    }
}

fn apply_move_down(board: &mut Vec<Vec<char>>, robot_pos: &mut Position) {
    if move_down(
        board,
        robot_pos.y as usize + 1,
        robot_pos.x as usize,
        robot_pos.x as usize,
    ) {
        robot_pos.apply_move(Move::Down);
    }
}

fn move_up(board: &mut Vec<Vec<char>>, y: usize, range_xa: usize, range_xb: usize) -> bool {
    let mut xa = board[0].len() - 1;
    let mut xb = 0;
    // Compute new push range, while checking if the whole range is free
    let mut all_free = true;
    for x in range_xa..=range_xb {
        let cell = board[y][x];
        if cell == '[' {
            xa = xa.min(x);
            xb = xb.max(x + 1);
            all_free = false;
        } else if cell == ']' {
            xa = xa.min(x - 1);
            xb = xb.max(x);
            all_free = false;
        } else if cell == '#' {
            // Wall, we can't move!
            return false;
        }
    }

    if all_free {
        return true;
    }

    // Move row above
    if !move_up(board, y - 1, xa, xb) {
        return false;
    }

    // Do move
    for x in xa..=xb {
        board[y - 1][x] = board[y][x];
        board[y][x] = '.';
    }

    return true;
}

fn move_down(board: &mut Vec<Vec<char>>, y: usize, range_xa: usize, range_xb: usize) -> bool {
    let mut xa = board[0].len() - 1;
    let mut xb = 0;
    // Compute new push range, while checking if the whole range is free
    let mut all_free = true;
    for x in range_xa..=range_xb {
        let cell = board[y][x];
        if cell == '[' {
            xa = xa.min(x);
            xb = xb.max(x + 1);
            all_free = false;
        } else if cell == ']' {
            xa = xa.min(x - 1);
            xb = xb.max(x);
            all_free = false;
        } else if cell == '#' {
            // Wall, we can't move!
            return false;
        }
    }

    if all_free {
        return true;
    }

    // Move row above
    if !move_down(board, y + 1, xa, xb) {
        return false;
    }

    // Do move
    for x in xa..=xb {
        board[y + 1][x] = board[y][x];
        board[y][x] = '.';
    }

    return true;
}

fn parse_input(
    input: impl BufRead,
) -> Result<(Vec<Vec<char>>, Vec<Move>, Position), Box<dyn Error>> {
    let lines = input.lines().collect::<Result<Vec<String>, io::Error>>()?;
    let mut lines = lines.iter();

    let mut robot_pos = None;

    // Parse board
    let board = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .enumerate()
        .map(|(y, row)| {
            let mut newrow = vec![];

            for (x, c) in row.chars().enumerate() {
                newrow.extend_from_slice(&match c {
                    '@' => {
                        if let Some(_) = robot_pos {
                            return Err(ParseError::new());
                        }
                        robot_pos = Some(Position::new_usize(x * 2, y));

                        ['.', '.']
                    }
                    '#' => ['#', '#'],
                    'O' => ['[', ']'],
                    '.' => ['.', '.'],
                    _ => return Err(ParseError::new()),
                })
            }

            Ok(newrow)
        })
        .collect::<Result<Vec<Vec<char>>, ParseError>>()?;

    // Parse moves
    let mut moves = vec![];
    for line in lines {
        moves.append(
            &mut line
                .chars()
                .map(|c| Move::from_char(c))
                .collect::<Result<Vec<Move>, ParseError>>()?,
        );
    }

    let robot_pos = match robot_pos {
        Some(pos) => pos,
        None => {
            return Err(Box::new(ParseError::new()));
        }
    };

    Ok((board, moves, robot_pos))
}

// fn print_board(board: &Vec<Vec<char>>, robot_pos: Position) {
//     for (y, line) in board.iter().enumerate() {
//         for (x, c) in line.iter().enumerate() {
//             if x as isize == robot_pos.x && y as isize == robot_pos.y {
//                 print!("@");
//             } else {
//                 print!("{}", c);
//             }
//         }
//         println!()
//     }
// }

fn calculate_gps_sum(board: &Vec<Vec<char>>) -> usize {
    let mut sum = 0;
    for (y, row) in board.iter().enumerate() {
        for (x, cell) in row.iter().enumerate().step_by(2) {
            if *cell == '[' {
                sum += y * 100 + x;
            } else if *cell == ']' {
                sum += y * 100 + x - 1
            }
        }
    }

    sum
}
