use core::fmt;
use std::{
    error::Error,
    fmt::{Display, Formatter},
    io::BufRead,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn rotate_right(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub x: isize,
    pub y: isize,
}

impl Position {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn move_direction(&self, dir: Direction) -> Position {
        let (dx, dy) = match dir {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };

        Self::new(self.x + dx, self.y + dy)
    }
}

pub type GuardState = (Position, Direction);

#[derive(Debug, Clone, Copy)]
pub enum LabMapCell {
    Obstacle,
    Empty,
}

#[derive(Debug)]
enum ParseError {
    NoGuard,
    MultipleGuards,
}

impl Error for ParseError {}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::NoGuard => write!(f, "guard not present"),
            ParseError::MultipleGuards => write!(f, "multiple guards"),
        }
    }
}

pub type LabMap = Vec<Vec<LabMapCell>>;

fn parse_labmap_line(
    line: &str,
) -> Result<(Vec<LabMapCell>, Option<(usize, Direction)>), ParseError> {
    // Not all lines contain a guard state
    let mut guard = None;

    // Parse row
    let row = line
        .chars()
        .enumerate()
        .map(|(idx, c)| {
            // Check if is guard
            let guard_dir = match c {
                '<' => Some(Direction::Left),
                '>' => Some(Direction::Right),
                '^' => Some(Direction::Up),
                'v' => Some(Direction::Down),
                _ => None,
            };

            if let Some(dir) = guard_dir {
                guard = Some((idx, dir));
            }

            // Parse cell
            Ok(match c {
                '#' => LabMapCell::Obstacle,
                _ => LabMapCell::Empty,
            })
        })
        .collect::<Result<Vec<LabMapCell>, ParseError>>()?;

    Ok((row, guard))
}

pub fn parse_input(reader: impl BufRead) -> Result<(LabMap, GuardState), Box<dyn Error>> {
    let mut map = Vec::new();

    let mut guard_state = None;

    // Parse all rows
    for (idx, line) in reader.lines().enumerate() {
        let line = line?;
        let (row, guard) = parse_labmap_line(&line)?;

        if let Some((x, dir)) = guard {
            if let Some(_) = guard_state {
                return Err(Box::new(ParseError::MultipleGuards));
            }

            guard_state = Some((Position::new(x as isize, idx as isize), dir));
        }

        map.push(row);
    }

    let guard_state = match guard_state {
        Some(s) => s,
        None => return Err(Box::new(ParseError::NoGuard)),
    };

    Ok((map, guard_state))
}

pub fn labmap_get(labmap: &LabMap, pos: Position) -> Option<LabMapCell> {
    if pos.y < 0
        || pos.y >= labmap.len() as isize
        || pos.x < 0
        || pos.x >= labmap[pos.y as usize].len() as isize
    {
        None
    } else {
        Some(labmap[pos.y as usize][pos.x as usize])
    }
}

pub fn print_board(labmap: &LabMap, guard_state: GuardState) {
    let (pos, dir) = guard_state;

    println!("----");

    for (y, row) in labmap.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            let c = if x as isize == pos.x && y as isize == pos.y {
                match dir {
                    Direction::Left => '<',
                    Direction::Right => '>',
                    Direction::Up => '^',
                    Direction::Down => 'v',
                }
            } else {
                match cell {
                    LabMapCell::Empty => '.',
                    LabMapCell::Obstacle => '#',
                }
            };

            print!("{c}");
        }
        println!();
    }
}
