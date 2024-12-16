use core::fmt;
use std::{
    error::Error,
    fmt::{Display, Formatter},
    io::{self, BufRead},
    vec,
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

    pub fn moved(&self, m: Move) -> Self {
        let (dx, dy) = m.get_deltas();
        Position::new(self.x + dx, self.y + dy)
    }

    pub fn apply_move(&mut self, m: Move) {
        let (dx, dy) = m.get_deltas();
        self.x += dx;
        self.y += dy;
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl Move {
    pub fn from_char(c: char) -> Result<Self, ParseError> {
        match c {
            '^' => Ok(Self::Up),
            'v' => Ok(Self::Down),
            '<' => Ok(Self::Left),
            '>' => Ok(Self::Right),
            _ => Err(ParseError::new()),
        }
    }

    pub fn get_deltas(&self) -> (isize, isize) {
        match self {
            Move::Up => (0, -1),
            Move::Down => (0, 1),
            Move::Left => (-1, 0),
            Move::Right => (1, 0),
        }
    }

    pub fn reverse(&self) -> Self {
        match self {
            Move::Up => Self::Down,
            Move::Down => Self::Up,
            Move::Left => Self::Right,
            Move::Right => Self::Left,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Cell {
    Box,
    Wall,
    Empty,
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Box => write!(f, "O"),
            Self::Wall => write!(f, "#"),
            Self::Empty => write!(f, " "),
        }
    }
}

pub fn parse_input(
    input: impl BufRead,
) -> Result<(Vec<Vec<Cell>>, Vec<Move>, Position), Box<dyn Error>> {
    let lines = input.lines().collect::<Result<Vec<String>, io::Error>>()?;
    let mut lines = lines.iter();

    // Robot position
    let mut robot_pos: Option<Position> = None;

    // Parse board
    let board = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    Ok(match c {
                        '#' => Cell::Wall,
                        'O' => Cell::Box,
                        '@' => {
                            // Set robot position
                            if let Some(_) = robot_pos {
                                return Err(ParseError::new());
                            }
                            robot_pos = Some(Position::new_usize(x, y));

                            Cell::Empty
                        }
                        _ => Cell::Empty,
                    })
                })
                .collect()
        })
        .collect::<Result<Vec<Vec<Cell>>, ParseError>>()?;

    // Parse moves
    let mut moves = vec![];
    for line in lines {
        moves.append(
            &mut line
                .chars()
                .map(|c| Move::from_char(c))
                .collect::<Result<Vec<Move>, ParseError>>()?,
        );
    }

    let robot_pos = match robot_pos {
        Some(pos) => pos,
        None => {
            return Err(Box::new(ParseError::new()));
        }
    };

    Ok((board, moves, robot_pos))
}

pub fn get_map_cell<T: Copy>(map: &Vec<Vec<T>>, pos: Position) -> Option<T> {
    if pos.x < 0 || pos.x >= map.len() as isize || pos.y < 0 || pos.y >= map[0].len() as isize {
        None
    } else {
        Some(map[pos.y as usize][pos.x as usize])
    }
}

pub fn set_map_cell<T: Copy>(map: &mut Vec<Vec<T>>, pos: Position, val: T) {
    if !(pos.x < 0 || pos.x >= map.len() as isize || pos.y < 0 || pos.y >= map[0].len() as isize) {
        map[pos.y as usize][pos.x as usize] = val;
    }
}

pub fn print_board(board: &Vec<Vec<Cell>>, robot_pos: Position) {
    for (y, row) in board.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if Position::new_usize(x, y) == robot_pos {
                print!("@");
            } else {
                print!("{}", cell);
            }
        }
        println!();
    }
}
