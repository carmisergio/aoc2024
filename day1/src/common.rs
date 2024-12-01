use core::panic;
use std::io::{stdin, BufRead, BufReader};

/// Read lists from stdin
pub fn read_lists() -> (Vec<i32>, Vec<i32>) {
    // Open bufreader on stdin
    let reader = BufReader::new(stdin());

    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();

    // Parse all lines from input
    reader.lines().for_each(|line| {
        match line {
            Ok(line) => match parse_line(&line) {
                Ok((num1, num2)) => {
                    list1.push(num1);
                    list2.push(num2);
                }
                Err(_) => panic!("Parse error!"),
            },
            Err(e) => panic!("I/O error: {}", e),
        };
    });

    (list1, list2)
}

/// Process line of input
fn parse_line(line: &str) -> Result<(i32, i32), ()> {
    // Divide line by space
    let mut split = line.split_whitespace();

    // Get numbers
    let n1 = split.next().ok_or(())?;
    let n2 = split.next().ok_or(())?;

    // Parse numbers
    let n1: i32 = n1.parse().or(Err(()))?;
    let n2: i32 = n2.parse().or(Err(()))?;

    Ok((n1, n2))
}

#[cfg(test)]
mod tests {
    use super::parse_line;

    #[test]
    fn parse_line_ok() {
        let tests = [
            ("1234 1234", (1234, 1234)),
            ("0 0", (0, 0)),
            ("1234   1234", (1234, 1234)),
            ("999999 999999", (999999, 999999)),
        ];

        for (input, exp) in tests {
            let res = parse_line(input).unwrap();
            assert_eq!(res, exp);
        }
    }
}
