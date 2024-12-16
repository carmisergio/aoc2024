use std::{
    io::{stdin, BufReader},
    process, vec,
};

use day14::{parse_input, Robot};

// Test parameters
// const BOARD_WIDTH: i64 = 11;
// const BOARD_HEIGHT: i64 = 7;

// Real parameters
const BOARD_WIDTH: i64 = 101;
const BOARD_HEIGHT: i64 = 103;

fn main() {
    // Read input
    let mut robots = parse_input(BufReader::new(stdin())).unwrap_or_else(|e| {
        eprintln!("Error reading input: {}", e);
        process::exit(1);
    });

    // Compute result
    do_computation(&mut robots);
}

fn do_computation(robots: &mut [Robot]) {
    // for second in 1..=STEPS {
    let mut second = 1;
    loop {
        let mut board = vec![vec![0; BOARD_WIDTH as usize]; BOARD_HEIGHT as usize];
        // Compute new board positions
        for robot in robots.iter_mut() {
            // Next robot position
            robot.x = (robot.x + robot.vel_x) % BOARD_WIDTH;
            robot.y = (robot.y + robot.vel_y) % BOARD_HEIGHT;

            if robot.x < 0 {
                robot.x = BOARD_WIDTH + robot.x;
            }

            if robot.y < 0 {
                robot.y = BOARD_HEIGHT + robot.y;
            }

            board[robot.y as usize][robot.x as usize] += 1;
        }

        println!("Second = {}", second);
        print_board(&board);

        second += 1;

        if board.iter().map(|row| row.iter().max()).max() == Some(Some(&1)) {
            break;
        }
    }
    // }
}

fn print_board(board: &Vec<Vec<u8>>) {
    for row in board {
        for cell in row {
            if *cell == 0 {
                print!(" ");
            } else {
                print!("{}", cell);
            }
        }
        println!();
    }
}
