use core::fmt;
use std::{
    error::Error,
    fmt::{Display, Formatter},
    io::BufRead,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: isize,
    pub y: isize,
}

impl Position {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn new_usize(x: usize, y: usize) -> Self {
        Self {
            x: x as isize,
            y: y as isize,
        }
    }
}

pub fn parse_input(input: impl BufRead) -> Result<Vec<Vec<u8>>, Box<dyn Error>> {
    let mut width = None;
    input
        .lines()
        .map(|line| -> Result<Vec<u8>, Box<dyn Error>> {
            let line = line?;
            let line: Vec<u8> = line
                .chars()
                .map(|c| -> Result<u8, Box<dyn Error>> {
                    match c.to_digit(10) {
                        Some(val) => Ok(val as u8),
                        None => Err(Box::new(ParseError::new())),
                    }
                })
                .collect::<Result<Vec<u8>, Box<dyn Error>>>()?;

            if let Some(width) = width {
                if line.len() != width {
                    return Err(Box::new(ParseError::new()));
                }
            }
            width = Some(line.len());

            Ok(line)
        })
        .collect()
}
