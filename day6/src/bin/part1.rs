use std::{
    io::{stdin, BufReader},
    vec,
};

use day6::{labmap_get, parse_input, GuardState, LabMap, LabMapCell};

fn main() {
    let (lab_map, guard_state) = parse_input(BufReader::new(stdin())).unwrap();

    let count = solve(&lab_map, guard_state);

    println!("Result: {}", count);
}

fn solve(lab_map: &LabMap, mut guard_state: GuardState) -> u32 {
    let mut visited = vec![vec![false; lab_map[0].len()]; lab_map.len()];

    while {
        let (pos, _) = guard_state;
        visited[pos.y as usize][pos.x as usize] = true;
        // print_board(lab_map, guard_state);
        guard_state = solution_step(lab_map, guard_state);
        let (pos, _) = guard_state;

        labmap_get(lab_map, pos).is_some()
    } {}

    let mut c = 0;
    for row in visited {
        for col in row {
            if col {
                c += 1;
            }
        }
    }

    c
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
