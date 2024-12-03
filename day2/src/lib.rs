use core::panic;
use std::io::{stdin, BufRead, BufReader};

pub type Report = Vec<i32>;

/// Read lists from stdin
pub fn read_reports() -> Vec<Report> {
    // Open bufreader on stdin
    let reader = BufReader::new(stdin());

    // Parse all lines from input
    reader
        .lines()
        .map(|line| match line {
            Ok(line) => match parse_line(&line) {
                Ok(res) => res,
                Err(_) => panic!("Parse error!"),
            },
            Err(e) => panic!("I/O error: {}", e),
        })
        .collect()
}

/// Process line of input
fn parse_line(line: &str) -> Result<Report, ()> {
    let mut res = Vec::new();
    // Divide line by space
    let split = line.split_whitespace();

    // Parse each word in a line
    for word in split {
        // Parse word
        let num: i32 = word.parse().or(Err(()))?;

        res.push(num);
    }

    Ok(res)
}

// Check if a sequence of parameters is safe
pub fn is_safe(report: &Report) -> bool {
    for i in 1..report.len() {
        let diff = report[i] - report[i - 1];

        // Check distance
        if diff.abs() > 3 || diff.abs() < 1 {
            return false;
        }

        // Check monotonous
        if i >= 2 {
            if report[i - 1] - report[i - 2] < 0 {
                if diff > 0 {
                    return false;
                }
            } else {
                if diff < 0 {
                    return false;
                }
            }
        }
    }

    return true;
}

#[cfg(test)]
mod tests {
    use crate::is_safe;

    use super::parse_line;

    #[test]
    fn parse_line_ok() {
        let tests = [
            ("2 4 6 2 4", [2, 4, 6, 2, 4]),
            ("0 0 0 0 0", [0, 0, 0, 0, 0]),
            ("1 2 3 4 5", [1, 2, 3, 4, 5]),
            ("9 5 6 2 4", [9, 5, 6, 2, 4]),
        ];

        for (input, exp) in tests {
            let res = parse_line(input).unwrap();
            assert_eq!(res, exp);
        }
    }

    #[test]
    fn is_safe_test() {
        let tests = [
            (vec![7, 6, 4, 2, 1], true),
            (vec![1, 2, 7, 8, 9], false),
            (vec![9, 7, 6, 2, 1], false),
            (vec![1, 3, 2, 4, 5], false),
            (vec![8, 6, 4, 4, 1], false),
            (vec![1, 3, 6, 7, 9], true),
        ];

        for (input, exp) in tests {
            let res = is_safe(&input);
            assert_eq!(res, exp);
        }
    }
}
