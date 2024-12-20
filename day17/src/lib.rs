use core::fmt;
use std::{
    error::Error,
    fmt::{Display, Formatter},
    io::{self, BufRead},
    str::FromStr,
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

#[derive(Debug, Clone, Copy)]
pub struct Registers {
    pub a: i64,
    pub b: i64,
    pub c: i64,
}

pub type OpCode = u8;

/// Parse input data
pub fn parse_input(input: impl BufRead) -> Result<(Registers, Vec<OpCode>), Box<dyn Error>> {
    let lines = input.lines().collect::<Result<Vec<String>, io::Error>>()?;
    let mut lines = lines.iter();

    let regs = {
        let reg_lines: Vec<&String> = lines.by_ref().take_while(|line| !line.is_empty()).collect();

        if reg_lines.len() != 3 {
            return Err(Box::new(PError::new()));
        }

        // Parse registers
        let (a, _) = parse_register_value('A')(reg_lines[0])?;
        let (b, _) = parse_register_value('B')(reg_lines[1])?;
        let (c, _) = parse_register_value('C')(reg_lines[2])?;

        Registers { a, b, c }
    };

    let prog = {
        let prog_line = lines.next().or_err(PError::new())?;
        let (prog, _) = parse_program(prog_line)?;
        prog
    };

    Ok((regs, prog))
}

fn parse_register_value<'a>(reg_id: char) -> impl Fn(&str) -> PResult<i64> + 'a {
    move |input| {
        let (_, input) = parse_tag(&format!("Register {}: ", reg_id))(input)?;
        let (val, input) = parse_number(input)?;

        Ok((val, input))
    }
}

fn parse_program(input: &str) -> PResult<Vec<OpCode>> {
    let (_, input) = parse_tag(&format!("Program: "))(input)?;
    let (program, input) = parse_many0(|input| {
        let (op, input) = parse_number(input)?;
        let (_, input) = parse_opt(parse_tag(","))(input)?;
        Ok((op, input))
    })(input)?;

    Ok((program, input))
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
fn parse_number<T: FromStr>(input: &str) -> PResult<T> {
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
    let num: T = match num.parse() {
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

trait OptToResult<T, E> {
    fn or_err(self, err: E) -> Result<T, E>;
}

impl<T, E> OptToResult<T, E> for Option<T> {
    fn or_err(self, err: E) -> Result<T, E> {
        if let Some(val) = self {
            Ok(val)
        } else {
            Err(err)
        }
    }
}

#[derive(Debug)]
pub struct ExecutionError {}

impl ExecutionError {
    pub fn new() -> Self {
        Self {}
    }
}

impl Display for ExecutionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "execution error")
    }
}

impl Error for ExecutionError {}
