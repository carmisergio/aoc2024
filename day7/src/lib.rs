use core::fmt;
use std::{
    error::Error,
    fmt::{Display, Formatter},
    io::BufRead,
};

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

pub fn parse_input(reader: impl BufRead) -> Result<Vec<(u64, Vec<u64>)>, Box<dyn Error>> {
    reader
        .lines()
        .map(|line| {
            let line = line?;
            let (res, _) = parse_input_line(&line)?;
            Ok(res)
        })
        .collect()
}

fn parse_input_line(input: &str) -> PResult<(u64, Vec<u64>)> {
    let (target, input) = parse_u64(input)?;
    let (_, input) = parse_tag(":")(input)?;
    let (factors, input) = parse_many0(parse_factor)(input)?;

    Ok(((target, factors), input))
}

fn parse_factor(input: &str) -> PResult<u64> {
    let (_, input) = parse_tag(" ")(input)?;
    let (n, input) = parse_u64(input)?;
    Ok((n, input))
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
fn parse_u64(input: &str) -> PResult<u64> {
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
    let num: u64 = match num.parse() {
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
