use core::fmt;
use std::{
    error::Error,
    fmt::{Display, Formatter},
    io::{self, BufRead},
    vec,
};

pub trait DisplayChar {
    fn display_char(&self) -> char;
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

    pub fn moved(&self, m: Direction) -> Self {
        let (dx, dy) = m.get_deltas();
        Position::new(self.x + dx, self.y + dy)
    }

    pub fn apply_move(&mut self, m: Direction) {
        let (dx, dy) = m.get_deltas();
        self.x += dx;
        self.y += dy;
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Position ({}, {})", self.x, self.y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn get_deltas(&self) -> (isize, isize) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }

    pub fn reverse(&self) -> Self {
        match self {
            Direction::Up => Self::Down,
            Direction::Down => Self::Up,
            Direction::Left => Self::Right,
            Direction::Right => Self::Left,
        }
    }

    pub fn turn_left(&self) -> Self {
        match self {
            Direction::Up => Self::Left,
            Direction::Down => Self::Right,
            Direction::Left => Self::Down,
            Direction::Right => Self::Up,
        }
    }

    pub fn turn_right(&self) -> Self {
        match self {
            Direction::Up => Self::Right,
            Direction::Down => Self::Left,
            Direction::Left => Self::Up,
            Direction::Right => Self::Down,
        }
    }

    pub fn is_orthogonal(&self, other: &Self) -> bool {
        match self {
            Direction::Up | Direction::Down => other == &Self::Left || other == &Self::Right,
            Direction::Left | Direction::Right => other == &Self::Up || other == &Self::Down,
        }
    }
}

pub struct Matrix2D<T> {
    inner: Vec<T>,
    width: usize,
    height: usize,
}

impl<T: Copy> Matrix2D<T> {
    /// Create new matrix
    pub fn new(width: usize, height: usize, default: T) -> Self {
        Self {
            inner: vec![default; width * height],
            width,
            height,
        }
    }

    /// Get matrix cell
    pub fn get(&self, pos: Position) -> Option<T> {
        let index = self.get_index(pos)?;
        Some(self.inner[index])
    }

    /// Set matrix cell
    pub fn set(&mut self, pos: Position, val: T) {
        if let Some(index) = self.get_index(pos) {
            self.inner[index] = val;
        }
    }

    // Get matrix width
    pub fn width(&self) -> usize {
        self.width
    }

    // Get matrix height
    pub fn height(&self) -> usize {
        self.height
    }

    /// Get index into inner
    fn get_index(&self, pos: Position) -> Option<usize> {
        if !(pos.x < 0
            || pos.x >= self.width as isize
            || pos.y < 0
            || pos.y >= self.height as isize)
        {
            Some(pos.y as usize * self.width + pos.x as usize)
        } else {
            None
        }
    }
}

impl<T: Copy + DisplayChar> Matrix2D<T> {
    pub fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", self.inner[y * self.width + x].display_char())
            }
            println!();
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Cell {
    Wall,
    Empty,
}

impl TryFrom<char> for Cell {
    type Error = ParseError;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Cell::Wall),
            '.' => Ok(Cell::Empty),
            _ => Err(ParseError::new()),
        }
    }
}

impl DisplayChar for Cell {
    fn display_char(&self) -> char {
        match self {
            Self::Wall => '#',
            Self::Empty => '.',
        }
    }
}

pub fn parse_input(
    input: impl BufRead,
) -> Result<(Matrix2D<Cell>, Position, Position), Box<dyn Error>> {
    let lines = input.lines().collect::<Result<Vec<String>, io::Error>>()?;

    if lines.is_empty() {
        return Err(Box::new(ParseError::new()));
    }

    let mut matrix = Matrix2D::new(lines[0].len(), lines.len(), Cell::Empty);
    let mut start_pos = None;
    let mut end_pos = None;

    for (y, line) in lines.iter().enumerate() {
        if line.len() != matrix.width() {
            return Err(Box::new(ParseError::new()));
        }

        for (x, c) in line.chars().enumerate() {
            let cur_pos = Position::new_usize(x, y);
            match c {
                'S' => {
                    if let Some(_) = start_pos {
                        return Err(Box::new(ParseError::new()));
                    }
                    start_pos = Some(cur_pos);
                }
                'E' => {
                    if let Some(_) = end_pos {
                        return Err(Box::new(ParseError::new()));
                    }
                    end_pos = Some(cur_pos);
                }
                c => {
                    let cell = Cell::try_from(c)?;
                    matrix.set(cur_pos, cell);
                }
            }
        }
    }

    if let Some(start_pos) = start_pos {
        if let Some(end_pos) = end_pos {
            return Ok((matrix, start_pos, end_pos));
        }
    }

    Err(Box::new(ParseError::new()))
}
