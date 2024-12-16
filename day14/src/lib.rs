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

#[derive(Debug)]
pub struct Robot {
    pub x: i64,
    pub y: i64,
    pub vel_x: i64,
    pub vel_y: i64,
}

/// Parse input data
pub fn parse_input(input: impl BufRead) -> Result<Vec<Robot>, Box<dyn Error>> {
    input
        .lines()
        .map(|line| {
            let line = line?;
            let (robot, _) = parse_robot(&line)?;
            Ok(robot)
        })
        .collect()
}

fn parse_robot(input: &str) -> PResult<Robot> {
    let (_, input) = parse_tag("p=")(input)?;
    let (x, input) = parse_i64(input)?;
    let (_, input) = parse_tag(",")(input)?;
    let (y, input) = parse_i64(input)?;
    let (_, input) = parse_tag(" v=")(input)?;
    let (vel_x, input) = parse_i64(input)?;
    let (_, input) = parse_tag(",")(input)?;
    let (vel_y, input) = parse_i64(input)?;

    Ok((Robot { x, y, vel_x, vel_y }, input))
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
fn parse_i64(input: &str) -> PResult<i64> {
    // Find end of digits
    let mut end: usize = 0;
    for c in input.chars() {
        if !c.is_ascii_digit() && c != '-' {
            break;
        }
        end += 1;
    }

    // Construct number slice
    let num = &input[0..end];

    // Parse number
    let num: i64 = match num.parse() {
        Ok(res) => res,
        Err(_) => return Err(PError::new()),
    };

    Ok((num, &input[end..]))
}
