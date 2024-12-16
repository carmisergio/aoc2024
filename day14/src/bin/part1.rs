use std::{
    io::{stdin, BufReader},
    process,
};

use day14::{parse_input, Robot};

const STEPS: u64 = 100;

// Test parameters
// const BOARD_WIDTH: i64 = 11;
// const BOARD_HEIGHT: i64 = 7;

// Real parameters
const BOARD_WIDTH: i64 = 101;
const BOARD_HEIGHT: i64 = 103;

fn main() {
    // Read input
    let robots = parse_input(BufReader::new(stdin())).unwrap_or_else(|e| {
        eprintln!("Error reading input: {}", e);
        process::exit(1);
    });

    // Compute result
    let res = compute_result(&robots);

    // Print result
    println!("Result: {}", res);
}

fn compute_result(robots: &[Robot]) -> u64 {
    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;

    for robot in robots {
        // dbg!(robot);
        let (x, y) = compute_robot_position(robot);
        // dbg!((x, y));

        if x > (BOARD_WIDTH - 1) / 2 {
            if y < (BOARD_HEIGHT - 1) / 2 {
                q1 += 1;
            } else if y > (BOARD_HEIGHT - 1) / 2 {
                q2 += 1;
            }
        } else if x < (BOARD_WIDTH - 1) / 2 {
            if y < (BOARD_HEIGHT - 1) / 2 {
                q4 += 1;
            } else if y > (BOARD_HEIGHT - 1) / 2 {
                q3 += 1;
            }
        }
    }

    // dbg!((q1, q2, q3, q4));

    q1 * q2 * q3 * q4
}

fn compute_robot_position(robot: &Robot) -> (i64, i64) {
    let x = mod_mult(robot.x, robot.vel_x, STEPS, BOARD_WIDTH);
    let y = mod_mult(robot.y, robot.vel_y, STEPS, BOARD_HEIGHT);
    (x, y)
}

fn mod_mult(start: i64, b: i64, e: u64, p: i64) -> i64 {
    let mut res = start;

    for _ in 0..e {
        res = (res + b) % p;
    }

    if res < 0 {
        res = p + res;
    }

    res
}
