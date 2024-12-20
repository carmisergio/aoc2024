fn main() {
    let target = [2, 4, 1, 5, 7, 5, 1, 6, 0, 3, 4, 2, 5, 5, 3, 0];

    // Print result
    println!("Result: {}", find_a_val(&target));
}

fn find_a_val(target: &[u8]) -> i64 {
    let mut a = 0;
    'outer: for (idx, _) in target.iter().rev().enumerate() {
        for modifier in 0.. {
            let new_a = a * 8
                + if modifier % 2 == 0 {
                    modifier / 2
                } else {
                    -(modifier / 2)
                };

            if new_a < 0 {
                continue;
            }

            if check_full_output(new_a, &target[target.len() - idx - 1..]) {
                a = new_a;
                continue 'outer;
            }
        }
    }
    a
}

fn check_full_output(mut a: i64, target: &[u8]) -> bool {
    let mut idx = 0;
    // dbg!(a);
    // dbg!(target);
    loop {
        if idx >= target.len() || compute_output(a) != target[idx] {
            return false;
        }
        a /= 8;
        idx += 1;

        if a == 0 {
            break;
        }
    }
    idx == target.len()
}

fn compute_output(a: i64) -> u8 {
    let b = a % 8;
    let b = b ^ 5;
    let c = a / (2i64.pow(b as u32));
    let b = b ^ 6;
    let b = b ^ c;
    (b % 8) as u8
}
