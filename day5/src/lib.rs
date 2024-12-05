use core::fmt;
use std::{
    error::Error,
    fmt::{Display, Formatter},
    io::{self, BufRead},
};

// Types
pub type OrderingRule = (u16, u16);
pub type Update = Vec<u16>;

// Pare result type
type PResult<'a, T> = Result<(T, &'a str), PError>;

// Parse error
#[derive(PartialEq, Debug, Clone)]
struct PError {}

impl PError {
    pub fn new() -> Self {
        Self {}
    }
}

impl std::error::Error for PError {}

impl Display for PError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Parse error!")
    }
}

pub fn parse_input<T: BufRead>(
    reader: T,
) -> Result<(Vec<OrderingRule>, Vec<Update>), Box<dyn Error>> {
    // Read lines to a vector
    let lines = reader.lines().collect::<Result<Vec<String>, io::Error>>()?;
    let mut lines = lines.iter();

    // Parse ordering rules
    let ord_rules = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (ord_rule, _) = parse_ordering_rule(line)?;
            Ok(ord_rule)
        })
        .collect::<Result<Vec<OrderingRule>, Box<dyn Error>>>()?;

    // Parse updates
    let updates = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (update, _) = parse_many0(parse_update_page)(line)?;
            Ok(update)
        })
        .collect::<Result<Vec<Update>, Box<dyn Error>>>()?;

    Ok((ord_rules, updates))
}

fn parse_ordering_rule(input: &str) -> PResult<OrderingRule> {
    let (a, input) = parse_u16(input)?;
    let (_, input) = parse_tag("|")(input)?;
    let (b, input) = parse_u16(input)?;

    Ok(((a, b), input))
}

fn parse_update_page(input: &str) -> PResult<u16> {
    let (page, input) = parse_u16(input)?;
    // Consume trailing comma
    let (_, input) = parse_opt(parse_tag(","))(input)?;

    Ok((page, input))
}

// Parse a string of text
fn parse_tag<'a>(tag: &'a str) -> impl Fn(&str) -> PResult<()> + 'a {
    let tag = tag.to_owned();

    move |input| {
        // Check size
        if input.len() < tag.len() {
            return Err(PError::new());
        }

        if input[0..tag.len()] == tag {
            return Ok(((), &input[tag.len()..]));
        } else {
            Err(PError::new())
        }
    }
}

// Parse a number with maximum number of digits
fn parse_u16(input: &str) -> PResult<u16> {
    // Find end of digits
    let mut end: usize = 0;
    for c in input.chars() {
        if !c.is_ascii_digit() {
            break;
        }
        end += 1;
    }

    // Construct number slice
    let num = &input[0..end];

    // Parse number
    let num: u16 = match num.parse() {
        Ok(res) => res,
        Err(_) => return Err(PError::new()),
    };

    Ok((num, &input[end..]))
}

fn parse_many0<'a, P, T>(parser: P) -> impl Fn(&'a str) -> PResult<'a, Vec<T>>
where
    P: Fn(&str) -> PResult<T>,
{
    move |mut input| {
        let mut res = Vec::new();

        while let Ok((val, rem)) = parser(input) {
            input = rem;
            res.push(val);
        }

        Ok((res, input))
    }
}

fn parse_opt<'a, P, T>(parser: P) -> impl Fn(&'a str) -> PResult<'a, Option<T>>
where
    P: Fn(&str) -> PResult<T>,
{
    move |input| match parser(input) {
        Ok((val, input)) => Ok((Some(val), input)),
        Err(_) => Ok((None, input)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;

    #[test]
    fn parse_tag_test() {
        let tests = [
            (("hellothere", "hellothere"), Ok(((), ""))),
            (("abcdefghi", "abcd"), Ok(((), "efghi"))),
            (("bcdef", "abcd"), Err(PError::new())),
        ];

        for ((input, tag), exp) in tests {
            let res = parse_tag(tag)(input);
            assert_eq!(res, exp);
        }
    }

    #[test]
    fn parse_u16_test() {
        let tests = [
            ("1234", Ok((1234, ""))),
            ("0abcde", Ok((0, "abcde"))),
            ("efghi", Err(PError::new())),
        ];

        for (input, exp) in tests {
            let res = parse_u16(input);
            assert_eq!(res, exp);
        }
    }

    #[test]
    fn parse_ordering_rule_test() {
        let tests = [
            ("10|20", Ok(((10, 20), ""))),
            ("139|189abcd", Ok(((139, 189), "abcd"))),
            ("testing1|2", Err(PError::new())),
        ];

        for (input, exp) in tests {
            let res = parse_ordering_rule(input);
            assert_eq!(res, exp);
        }
    }

    #[test]
    fn parse_update_test() {
        let tests = [("10,20,30ciaone", Ok((vec![10, 20, 30], "ciaone")))];

        for (input, exp) in tests {
            let res = parse_many0(parse_update_page)(input);
            assert_eq!(res, exp);
        }
    }

    #[test]
    fn parse_input_1() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

        let exp_ord_rules = vec![
            (47, 53),
            (97, 13),
            (97, 61),
            (97, 47),
            (75, 29),
            (61, 13),
            (75, 53),
            (29, 13),
            (97, 29),
            (53, 29),
            (61, 53),
            (97, 53),
            (61, 29),
            (47, 13),
            (75, 47),
            (97, 75),
            (47, 61),
            (75, 61),
            (47, 29),
            (75, 13),
            (53, 13),
        ];

        let exp_updates = vec![
            vec![75, 47, 61, 53, 29],
            vec![97, 61, 53, 29, 13],
            vec![75, 29, 13],
            vec![75, 97, 47, 61, 53],
            vec![61, 13, 29],
            vec![97, 13, 75, 29, 47],
        ];

        let reader = BufReader::new(input.as_bytes());
        let (ord_rules, updates) = parse_input(reader).unwrap();
        assert_eq!(ord_rules, exp_ord_rules);
        assert_eq!(updates, exp_updates);
    }
}
