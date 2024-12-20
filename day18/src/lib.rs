use core::fmt;
use std::{
    cmp::Ordering,
    collections::BTreeSet,
    error::Error,
    fmt::{Display, Formatter},
    io::BufRead,
    str::FromStr,
};

// Pare result type
type PResult<'a, T> = Result<(T, &'a str), PError>;

// Parse error
#[derive(PartialEq, Debug, Clone)]
pub struct PError {}

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
pub fn parse_input(input: impl BufRead) -> Result<Vec<Position>, Box<dyn Error>> {
    input
        .lines()
        .map(|line| {
            let line = line?;
            let (byte, _) = parse_byte(&line)?;
            Ok(byte)
        })
        .collect()
}

pub fn parse_byte(input: &str) -> PResult<Position> {
    let (x, input) = parse_number(input)?;
    let (_, input) = parse_tag(",")(input)?;
    let (y, input) = parse_number(input)?;
    Ok((Position::new(x, y), input))
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
        write!(f, "{},{}", self.x, self.y)
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

pub trait DisplayChar {
    fn display_char(&self) -> char;
}

impl DisplayChar for bool {
    fn display_char(&self) -> char {
        match self {
            true => '#',
            false => '.',
        }
    }
}

// Calculate distance
pub fn calculate_distance(
    matrix: &Matrix2D<bool>,
    start: Position,
    end: Position,
) -> Option<usize> {
    let dist = dijkstra(matrix, start);
    dist.get(end)?
}

#[derive(Debug, Clone, Copy)]
pub struct TempDistanceNode {
    pub pos: Position,
    pub dist: usize,
}

impl PartialEq for TempDistanceNode {
    fn eq(&self, other: &TempDistanceNode) -> bool {
        self.pos == other.pos
    }
}

impl Eq for TempDistanceNode {}

impl PartialOrd for TempDistanceNode {
    fn partial_cmp(&self, other: &TempDistanceNode) -> Option<Ordering> {
        if self.pos == other.pos {
            return Some(Ordering::Equal);
        }

        if self.dist > other.dist {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Less)
        }
    }
}

impl Ord for TempDistanceNode {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.pos == other.pos {
            return Ordering::Equal;
        }
        if self.dist > other.dist {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }
}

// Compute distance from start to end
pub fn dijkstra(matrix: &Matrix2D<bool>, start: Position) -> Matrix2D<Option<usize>> {
    let mut tmp_distances: BTreeSet<TempDistanceNode> = BTreeSet::new();
    let mut final_distances: Matrix2D<Option<usize>> =
        Matrix2D::new(matrix.width(), matrix.height(), None);

    // Start from start node
    tmp_distances.insert(TempDistanceNode {
        pos: start,
        dist: 0,
    });

    // Iterate until there are unexplored nodes in the best path
    while let Some(node) = tmp_distances.pop_first() {
        // Check if we can go there
        if let Some(cell) = matrix.get(node.pos) {
            // There's a corrupted cell
            if cell {
                continue;
            }

            // The distance has already been fixed
            if let Some(Some(_)) = final_distances.get(node.pos) {
                continue;
            }
        } else {
            continue;
        }

        // Fix this node's distance
        final_distances.set(node.pos, Some(node.dist));

        let directions = [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ];

        for dir in directions {
            // Moved
            let pos = node.pos.moved(dir);

            // Calculate new distance

            // Add new tmporary distance node
            tmp_distances.insert(TempDistanceNode {
                pos,
                dist: node.dist + 1,
            });
        }
    }

    final_distances
}

// Create memory map
pub fn construct_map(bytes: &[Position], width: usize, height: usize) -> Matrix2D<bool> {
    let mut map = Matrix2D::new(width, height, false);

    // Simulate falling
    for byte in bytes {
        map.set(*byte, true);
    }

    map
}
