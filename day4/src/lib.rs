use std::io::{self, stdin, BufRead, BufReader};

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    DiagMajorBw,
    DiagMajorFw,
    DiagMinorFw,
    DiagMinorBw,
}

impl Direction {
    pub fn offset(&self) -> (isize, isize) {
        // (x, y)
        match self {
            Self::Up => (0, -1),
            Self::Down => (0, 1),
            Self::Left => (-1, 0),
            Self::Right => (1, 0),
            Self::DiagMajorBw => (-1, -1),
            Self::DiagMajorFw => (1, 1),
            Self::DiagMinorBw => (-1, 1),
            Self::DiagMinorFw => (1, -1),
        }
    }
}
// Read input from stdin
pub fn read_board() -> Result<Vec<Vec<char>>, io::Error> {
    let reader = BufReader::new(stdin());

    reader
        .lines()
        .map(|line| Ok(line?.chars().collect::<Vec<char>>()))
        .filter(|row| {
            if let Ok(row) = row {
                !row.is_empty()
            } else {
                false
            }
        })
        .collect()
}

pub fn next_position(
    (x, y): (usize, usize),
    dir: &Direction,
    width: usize,
    height: usize,
) -> Result<(usize, usize), ()> {
    let (ofst_x, ofst_y) = dir.offset();

    // Compute new position
    let new_x = x as isize + ofst_x;
    let new_y = y as isize + ofst_y;

    // Check bounds
    if new_x < 0 || new_x >= width as isize || new_y < 0 || new_y >= height as isize {
        return Err(());
    }

    Ok((new_x as usize, new_y as usize))
}
