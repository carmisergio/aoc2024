use core::{panic, str};
use std::io::{stdin, Read};

// Parse result type
type PResult<'a, T> = Result<(T, &'a str), ()>;

// Read input from stdin
pub fn read_input() -> String {
    let mut input = Vec::new();
    if let Err(_) = stdin().lock().read_to_end(&mut input) {
        panic!("Read error!");
    }

    match str::from_utf8(&input) {
        Ok(val) => val.to_owned(),
        Err(_) => panic!("UTF-8 error!"),
    }
}

#[derive(PartialEq, Debug)]
pub enum Operation {
    Mul(u16, u16),
    Do,
    Dont,
}

/// Extract valid multiplications from a corrupted
/// instruction stream
pub fn extract_operations(input: &str) -> Vec<Operation> {
    let mut ops = Vec::new();

    for i in 0..input.len() {
        let slice = &input[i..];

        // Multiplication
        if let Ok(((a, b), _)) = parse_multiplication(slice) {
            ops.push(Operation::Mul(a, b));
            continue;
        }

        // Conditional
        if let Ok((op, _)) = parse_conditional(slice) {
            ops.push(op);
            continue;
        }
    }

    ops
}

fn parse_multiplication(input: &str) -> PResult<(u16, u16)> {
    let (_, input) = parse_tag("mul(")(input)?;
    let (n1, input) = parse_u16_n(3)(input)?;
    let (_, input) = parse_tag(",")(input)?;
    let (n2, input) = parse_u16_n(3)(input)?;
    let (_, input) = parse_tag(")")(input)?;
    Ok(((n1, n2), input))
}

fn parse_conditional(input: &str) -> PResult<Operation> {
    if let Ok((_, input)) = parse_tag("do()")(input) {
        return Ok((Operation::Do, input));
    };

    if let Ok((_, input)) = parse_tag("don't()")(input) {
        return Ok((Operation::Dont, input));
    };

    Err(())
}

// Parse a string of text
fn parse_tag(tag: &str) -> impl Fn(&str) -> PResult<()> {
    // TODO: no extra allocations
    let tag = tag.to_owned();

    move |input| {
        // Check size
        if input.len() < tag.len() {
            return Err(());
        }

        if input[0..tag.len()] == tag {
            return Ok(((), &input[tag.len()..]));
        } else {
            Err(())
        }
    }
}

// Parse a number with maximum number of digits
fn parse_u16_n(n: usize) -> impl Fn(&str) -> PResult<u16> {
    move |input| {
        // Find end of digits
        let mut end: usize = 0;
        for c in input.chars() {
            if !c.is_ascii_digit() || end >= n {
                break;
            }
            end += 1;
        }

        // Construct number slice
        let num = &input[0..end];

        // Parse number
        let num: u16 = match num.parse() {
            Ok(res) => res,
            Err(_) => return Err(()),
        };

        Ok((num, &input[end..]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_operations_test() {
        let tests = [
            (
                "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5)",
                vec![
                    Operation::Mul(2, 4),
                    Operation::Mul(5, 5),
                    Operation::Mul(11, 8),
                    Operation::Mul(8, 5),
                ],
            ),
            (
                "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
                vec![
                    Operation::Mul(2, 4),
                    Operation::Dont,
                    Operation::Mul(5, 5),
                    Operation::Mul(11, 8),
                    Operation::Do,
                    Operation::Mul(8, 5),
                ],
            ),
        ];

        for (input, exp) in tests {
            let res = extract_operations(&input);
            assert_eq!(res, exp);
        }
    }

    #[test]
    fn parse_tag_test() {
        let tests = [
            (("hellothere", "hellothere"), Ok(((), ""))),
            (("abcdefghi", "abcd"), Ok(((), "efghi"))),
            (("bcdef", "abcd"), Err(())),
        ];

        for ((input, tag), exp) in tests {
            let res = parse_tag(tag)(input);
            assert_eq!(res, exp);
        }
    }

    #[test]
    fn parse_u16_n_test() {
        let tests = [
            (("1234", 5), Ok((1234, ""))),
            (("0abcde", 5), Ok((0, "abcde"))),
            (("1234", 3), Ok((123, "4"))),
            (("efghi", 5), Err(())),
        ];

        for ((input, tag), exp) in tests {
            let res = parse_u16_n(tag)(input);
            assert_eq!(res, exp);
        }
    }

    #[test]
    fn parse_multiplication_test() {
        let tests = [
            ("mul(3,5)", Ok(((3, 5), ""))),
            ("mul(999,999)hello", Ok(((999, 999), "hello"))),
            ("something", Err(())),
            ("mul(1000,10)", Err(())),
            ("", Err(())),
        ];

        for (input, exp) in tests {
            let res = parse_multiplication(input);
            assert_eq!(res, exp);
        }
    }
}
