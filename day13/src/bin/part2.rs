use std::{
    io::{stdin, BufReader},
    process,
};

use day13::{parse_input, Machine, Position};

fn main() {
    // Read input
    let machines = parse_input(BufReader::new(stdin())).unwrap_or_else(|e| {
        eprintln!("Error reading input: {}", e);
        process::exit(1);
    });

    let res = compute_result(&machines);

    println!("Result: {}", res);
}

fn compute_result(machines: &[Machine]) -> i64 {
    machines
        .iter()
        .map(|machine| {
            let machine = Machine {
                prize_pos: Position {
                    x: machine.prize_pos.x + 10000000000000,
                    y: machine.prize_pos.y + 10000000000000,
                    // x: machine.prize_pos.x,
                    // y: machine.prize_pos.y,
                },
                btn_a: machine.btn_a,
                btn_b: machine.btn_b,
            };
            calculate_price(&machine).unwrap_or(0)
        })
        .sum()
}

fn calculate_price(machine: &Machine) -> Option<i64> {
    let term_a = machine.btn_b.x * machine.prize_pos.y - machine.btn_b.y * machine.prize_pos.x;
    let term_b = machine.btn_b.x * machine.btn_a.y - machine.btn_b.y * machine.btn_a.x;

    if term_a % term_b == 0 {
        let solution_a = term_a / term_b;
        let term_c = machine.prize_pos.y - solution_a * machine.btn_a.y;
        if term_c % machine.btn_b.y == 0 {
            // Both solutions are integers
            let solution_b = term_c / machine.btn_b.y;
            return Some(solution_a * 3 + solution_b);
        }
    }

    None
}
