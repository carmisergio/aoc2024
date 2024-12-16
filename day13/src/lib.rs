use core::fmt;
use std::{
    error::Error,
    fmt::{Display, Formatter},
    io::{self, BufRead},
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
pub struct Button {
    pub x: i64,
    pub y: i64,
}

impl Button {
    pub fn apply_movement(&self, pos: Position) -> Position {
        Position {
            x: pos.x + self.x,
            y: pos.y + self.y,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub struct Position {
    pub x: i64,
    pub y: i64,
}

#[derive(Debug, Clone)]
pub struct Machine {
    pub btn_a: Button,
    pub btn_b: Button,
    pub prize_pos: Position,
}

/// Parse input data
pub fn parse_input(input: impl BufRead) -> Result<Vec<Machine>, Box<dyn Error>> {
    let lines = input.lines().collect::<Result<Vec<String>, io::Error>>()?;

    let machines = lines
        .iter()
        .group_until(|line| line.is_empty())
        .map(|group| {
            // Wrong number of lines
            if group.len() != 3 {
                return Err(PError::new());
            }

            let (btn_a, _) = parse_button('A')(group[0])?;
            let (btn_b, _) = parse_button('B')(group[1])?;
            let (prize_pos, _) = parse_prize_location(group[2])?;

            Ok(Machine {
                btn_a,
                btn_b,
                prize_pos,
            })
        })
        .collect::<Result<Vec<Machine>, PError>>()?;

    Ok(machines)
}

fn parse_button<'a>(button_id: char) -> impl Fn(&str) -> PResult<Button> + 'a {
    move |input| {
        let (_, input) = parse_tag(&format!("Button {}:", button_id))(input)?;
        let (delta_x, input) = parse_value("X+")(input)?;
        let (delta_y, input) = parse_value("Y+")(input)?;

        Ok((
            Button {
                x: delta_x,
                y: delta_y,
            },
            input,
        ))
    }
}

fn parse_prize_location(input: &str) -> PResult<Position> {
    let (_, input) = parse_tag("Prize:")(input)?;
    let (x, input) = parse_value("X=")(input)?;
    let (y, input) = parse_value("Y=")(input)?;

    Ok((Position { x, y }, input))
}

fn parse_value<'a>(prefix: &'a str) -> impl Fn(&str) -> PResult<i64> + 'a {
    move |input| {
        let (_, input) = parse_tag(" ")(input)?;
        let (_, input) = parse_tag(prefix)(input)?;
        let (val, input) = parse_i64(input)?;
        let (_, input) = parse_opt(parse_tag(","))(input)?;

        Ok((val, input))
    }
}

struct GroupUntil<I, F> {
    iter: I,
    f: F,
}

impl<I, F> GroupUntil<I, F> {
    pub fn new(iter: I, f: F) -> Self {
        Self { iter, f }
    }
}

impl<I: Iterator, F> Iterator for GroupUntil<I, F>
where
    F: FnMut(&I::Item) -> bool,
{
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut group = vec![];

        loop {
            if let Some(next) = self.iter.next() {
                if !(self.f)(&next) {
                    group.push(next);
                    continue;
                }
            }

            break;
        }

        if group.len() == 0 {
            return None;
        }

        Some(group)
    }
}

trait ToGroupUntil<I, F> {
    fn group_until(self, f: F) -> GroupUntil<I, F>;
}

impl<I: Iterator, F> ToGroupUntil<I, F> for I
where
    F: FnMut(&I::Item) -> bool,
{
    fn group_until(self, f: F) -> GroupUntil<I, F> {
        GroupUntil::new(self, f)
    }
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
        if !c.is_ascii_digit() {
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

fn parse_opt<'a, P, T>(parser: P) -> impl Fn(&'a str) -> PResult<'a, Option<T>>
where
    P: Fn(&str) -> PResult<T>,
{
    move |input| match parser(input) {
        Ok((val, input)) => Ok((Some(val), input)),
        Err(_) => Ok((None, input)),
    }
}
