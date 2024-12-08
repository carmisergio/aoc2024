use std::{
    io::{stdin, BufReader},
    process,
};

use day7::parse_input;

fn main() {
    let equations = parse_input(BufReader::new(stdin())).unwrap_or_else(|e| {
        eprintln!("Read error: {}", e);
        process::exit(1);
    });

    let mut sum = 0;
    for (target, mut factors) in equations {
        factors.reverse();
        if is_solvable(target, &factors) {
            sum += target;
        }
    }

    println!("Result: {}", sum);
}

fn is_solvable(target: u64, factors: &[u64]) -> bool {
    // Empty target is an error
    if factors.len() < 1 {
        return false;
    }

    // Done!
    if factors.len() == 1 {
        return target == factors[0];
    }

    let next = factors[0];
    let factors = &factors[1..];

    // Addition
    if target >= next && is_solvable(target - next, factors) {
        return true;
    }

    // Multiplication
    if target % next == 0 && is_solvable(target / next, factors) {
        return true;
    }

    // Concatenation
    if let Ok(target) = unconcatenate(target, next) {
        if is_solvable(target, factors) {
            return true;
        }
    }

    return false;
}

fn unconcatenate(cat: u64, b: u64) -> Result<u64, ()> {
    let exp = ((b + 1) as f64).log10().ceil() as u32;
    let pow = (10 as u64).pow(exp);

    if cat % pow == b {
        Ok(cat / pow)
    } else {
        Err(())
    }
}

#[cfg(test)]
mod tests {
    use crate::unconcatenate;

    #[test]
    fn test_unconcatenate() {
        let tests = [
            ((1020, 20), Ok(10)),
            ((156, 6), Ok(15)),
            ((12349, 9), Ok(1234)),
            ((12341000, 1000), Ok(1234)),
            ((123456, 999), Err(())),
        ];

        for ((cat, b), exp) in tests {
            let res = unconcatenate(cat, b);
            assert_eq!(res, exp);
        }
    }
}
