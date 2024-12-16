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

pub fn parse_input(input: impl BufRead) -> Result<Vec<Vec<char>>, Box<dyn Error>> {
    let mut width = None;
    input
        .lines()
        .map(|line| -> Result<Vec<char>, Box<dyn Error>> {
            let line = line?;
            let line: Vec<char> = line.chars().collect();

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

pub fn get_map_cell<'a, T>(map: &'a Vec<Vec<T>>, pos: Position) -> Option<&'a T> {
    if pos.x < 0 || pos.x >= map.len() as isize || pos.y < 0 || pos.y >= map[0].len() as isize {
        None
    } else {
        Some(&map[pos.y as usize][pos.x as usize])
    }
}

pub fn get_map_cell_unchecked<'a, T>(map: &'a Vec<Vec<T>>, pos: Position) -> &T {
    &map[pos.y as usize][pos.x as usize]
}
