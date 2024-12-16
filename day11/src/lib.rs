use core::fmt;
use std::{
    collections::HashMap,
    error::Error,
    fmt::{Display, Formatter},
    io::BufRead,
    num::ParseIntError,
};

#[derive(Debug)]
pub struct ParseError {}

impl ParseError {
    pub fn new() -> Self {
        Self {}
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "parse error")
    }
}

impl Error for ParseError {}

pub fn parse_input(input: impl BufRead) -> Result<Vec<u64>, Box<dyn Error>> {
    let line = input.lines().next();

    if let Some(line) = line {
        let line = line?;
        let res = line
            .split_whitespace()
            .map(|num| num.parse())
            .collect::<Result<Vec<u64>, ParseIntError>>()?;
        return Ok(res);
    }

    return Err(Box::new(ParseError::new()));
}

pub fn process(input: &[u64], steps: usize) -> usize {
    let mut memo = HashMap::new();
    input
        .iter()
        .map(|n| process_value(*n, steps, &mut memo))
        .sum()
}

fn process_value(n: u64, steps: usize, memo: &mut HashMap<(u64, usize), usize>) -> usize {
    // Base case
    if steps == 0 {
        return 1;
    }

    if let Some(res) = memo.get(&(n, steps)) {
        return *res;
    }

    let res = if n == 0 {
        process_value(1, steps - 1, memo)
    } else if let Some((a, b)) = split_number(n) {
        process_value(a, steps - 1, memo) + process_value(b, steps - 1, memo)
    } else {
        process_value(n * 2024, steps - 1, memo)
    };

    memo.insert((n, steps), res);

    res
}

fn count_digits(n: u64) -> u32 {
    (n as f64).log10().floor() as u32 + 1
}

fn split_number(n: u64) -> Option<(u64, u64)> {
    let n_digits = count_digits(n);

    if n_digits % 2 != 0 {
        return None;
    }

    let pow = 10u64.pow(n_digits / 2);

    let a = n / pow;
    let b = n % pow;

    Some((a, b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_digits_1() {
        let tests = [(12, 2), (123, 3), (0, 1), (9, 1), (6666, 4), (66667, 5)];

        for (input, exp) in tests {
            let res = count_digits(input);
            assert_eq!(res, exp);
        }
    }

    #[test]
    fn split_number_1() {
        let tests = [
            (1234, Some((12, 34))),
            (123, None),
            (44, Some((4, 4))),
            (1000, Some((10, 0))),
        ];

        for (input, exp) in tests {
            let res = split_number(input);
            assert_eq!(res, exp);
        }
    }
}
