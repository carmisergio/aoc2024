use std::{
    io::{stdin, BufReader},
    vec,
};

use day6::{labmap_get, parse_input, Direction, GuardState, LabMap, LabMapCell};

fn main() {
    let (lab_map, guard_state) = parse_input(BufReader::new(stdin())).unwrap();

    let mut c = 0;
    for y in 0..lab_map.len() {
        for x in 0..lab_map[y].len() {
            let mut new = lab_map.clone();

            if let LabMapCell::Obstacle = new[y][x] {
                continue;
            }

            if x as isize == guard_state.0.x && y as isize == guard_state.0.y {
                continue;
            }

            new[y][x] = LabMapCell::Obstacle;

            if is_infinite_loop(&new, guard_state) {
                c += 1;
            }
        }

        println!("LOOP");
    }

    println!("Result: {}", c);
}

fn is_infinite_loop(lab_map: &LabMap, mut guard_state: GuardState) -> bool {
    let mut visits: Vec<Vec<Vec<Direction>>> = vec![vec![vec![]; lab_map[0].len()]; lab_map.len()];

    while {
        let (pos, dir) = guard_state;

        if visits[pos.y as usize][pos.x as usize].contains(&dir) {
            return true;
        }

        visits[pos.y as usize][pos.x as usize].push(dir);

        // print_board(lab_map, guard_state);
        guard_state = solution_step(lab_map, guard_state);
        let (pos, _) = guard_state;

        labmap_get(lab_map, pos).is_some()
    } {}

    false
}

fn solution_step(lab_map: &LabMap, guard_state: GuardState) -> GuardState {
    let (pos, mut dir) = guard_state;
    for _ in 0..4 {
        let newpos = pos.move_direction(dir);

        let cell = labmap_get(lab_map, newpos);

        if let Some(cell) = cell {
            if let LabMapCell::Obstacle = cell {
                dir = dir.rotate_right();
                continue;
            }
        }

        return (newpos, dir);
    }

    panic!("no solution");
}
