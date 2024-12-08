use core::fmt;
use std::{
    error::Error,
    fmt::{Display, Formatter},
    io::BufRead,
};

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

#[derive(Debug)]
pub struct Antenna {
    // Frequency the antenna is emitting
    pub frequency: char,

    // Position of the antenna
    pub position: Position,
}

#[derive(Debug)]
pub struct ParseResult {
    // Board dimensions
    pub board_width: usize,
    pub board_height: usize,

    // Anntennas
    pub antennas: Vec<Antenna>,
}

/// Parse result type
#[derive(Debug)]
pub struct ParseError {
    msg: String,
}

impl ParseError {
    pub fn new(msg: &str) -> Self {
        Self {
            msg: msg.to_owned(),
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "parse error: {}", self.msg)
    }
}

impl Error for ParseError {}

/// Parse input from bufreader
pub fn parse_input(input: impl BufRead) -> Result<ParseResult, Box<dyn Error>> {
    let mut board_width = 0;
    let mut board_height = 0;
    let mut antennas = Vec::new();

    for (y, line) in input.lines().enumerate() {
        let line = line?;

        // Handle empty lines
        if line.is_empty() {
            continue;
        }

        // Update board width
        if board_width == 0 {
            board_width = line.len();
        } else if line.len() != board_width {
            return Err(Box::new(ParseError::new("malformed board")));
        }

        // Find antennas
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                antennas.push(Antenna {
                    frequency: c,
                    position: Position::new_usize(x, y),
                });
            }
        }

        // Update board height
        board_height += 1;
    }

    Ok(ParseResult {
        board_width,
        board_height,
        antennas,
    })
}

pub struct Pairs<'a, T: Copy> {
    inner: &'a [T],

    // Loop variables
    i: usize,
    j: usize,
}

impl<'a, T: Copy> Pairs<'a, T> {
    pub fn new(inner: &'a [T]) -> Self {
        Self { inner, i: 0, j: 1 }
    }
}

impl<'a, T: Copy> Iterator for Pairs<'a, T> {
    type Item = (T, T);
    fn next(&mut self) -> Option<Self::Item> {
        // No more couples
        if self.i + 1 >= self.inner.len() {
            return None;
        }

        // Get current couples
        let cur = (self.inner[self.i], self.inner[self.j]);

        // Next element
        self.j += 1;

        // End of one external element, increment i
        if self.j >= self.inner.len() {
            self.i += 1;
            self.j = self.i + 1;
        }

        Some(cur)
    }
}

pub fn gcd(a: isize, b: isize) -> isize {
    if a == 0 {
        b
    } else {
        gcd(b % a, a)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pairs_iterator_1() {
        let inner = [1, 2, 3, 4];
        let exp = vec![(1, 2), (1, 3), (1, 4), (2, 3), (2, 4), (3, 4)];
        let res: Vec<(i32, i32)> = Pairs::new(&inner).collect();
        assert_eq!(res, exp);
    }

    #[test]
    fn pairs_iterator_2() {
        let inner = [];
        let exp = vec![];
        let res: Vec<(i32, i32)> = Pairs::new(&inner).collect();
        assert_eq!(res, exp);
    }

    #[test]
    fn pairs_iterator_3() {
        let inner = [1];
        let exp = vec![];
        let res: Vec<(i32, i32)> = Pairs::new(&inner).collect();
        assert_eq!(res, exp);
    }

    #[test]
    fn gcd_1() {
        let exp = 3;
        let res = gcd(30, 9);
        assert_eq!(res, exp);
    }
}
