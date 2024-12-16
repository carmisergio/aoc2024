use std::{
    collections::HashMap,
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

fn compute_result(machines: &[Machine]) -> u32 {
    machines
        .iter()
        .map(|machine| {
            let mut memo = HashMap::new();
            calculate_price(machine, &mut memo, Position { x: 0, y: 0 }).unwrap_or(0)
        })
        .sum()
}

fn calculate_price(
    machine: &Machine,
    memo: &mut HashMap<Position, Option<u32>>,
    cur_pos: Position,
) -> Option<u32> {
    // Overshot
    if cur_pos.x > machine.prize_pos.x || cur_pos.y > machine.prize_pos.y {
        return None;
    }

    // Found prize
    if cur_pos == machine.prize_pos {
        // It costs nothing
        return Some(0);
    }

    // Check memo
    if let Some(memo) = memo.get(&cur_pos) {
        // println!("Memo hit!");
        return *memo;
    }

    let cost_a = calculate_price(machine, memo, machine.btn_a.apply_movement(cur_pos));
    let cost_b = calculate_price(machine, memo, machine.btn_b.apply_movement(cur_pos));

    let val = if let Some(cost_a) = cost_a {
        let cost_a = cost_a + 3;
        Some(if let Some(cost_b) = cost_b {
            cost_a.min(cost_b + 1)
        } else {
            cost_a
        })
    } else {
        if let Some(cost_b) = cost_b {
            Some(cost_b + 1)
        } else {
            None
        }
    };

    memo.insert(cur_pos, val);

    return val;
}
