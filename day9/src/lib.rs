use core::fmt;
use std::{
    cmp::Ordering,
    error::Error,
    fmt::{Display, Formatter},
    io::{self, ErrorKind, Read},
};

pub trait IntoAsciiChars<T: Read> {
    fn ascii_chars(&mut self) -> AsciiChars<T>;
}

pub struct AsciiChars<'a, T: Read> {
    inner: &'a mut T,
}

impl<'a, T: Read> AsciiChars<'a, T> {
    pub fn new(inner: &'a mut T) -> Self {
        Self { inner }
    }
}

impl<'a, T: Read> Iterator for AsciiChars<'a, T> {
    type Item = Result<char, io::Error>;
    fn next(&mut self) -> Option<Self::Item> {
        let mut byte = [0];
        match self.inner.read_exact(&mut byte) {
            Ok(_) => Some(Ok(byte[0] as char)),
            Err(e) => {
                if let ErrorKind::UnexpectedEof = e.kind() {
                    None
                } else {
                    Some(Err(e))
                }
            }
        }
    }
}

impl<T: Read> IntoAsciiChars<T> for T {
    fn ascii_chars(&mut self) -> AsciiChars<T> {
        AsciiChars::new(self)
    }
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

pub fn build_array(len: usize, files: &[Span]) -> Vec<Option<usize>> {
    let mut res = vec![None; len];

    for (file_id, span) in files.iter().enumerate() {
        for i in 0..span.len {
            res[span.pos + i] = Some(file_id)
        }
    }

    res
}

pub fn compute_checksum(arr: &[Option<usize>]) -> usize {
    arr.iter()
        .enumerate()
        .map(|(idx, el)| match el {
            Some(block_id) => idx * block_id,
            None => 0,
        })
        .sum()
}

#[derive(Debug, Clone, Copy)]
pub struct Span {
    pub pos: usize,
    pub len: usize,
}

impl Span {
    pub fn new(pos: usize, len: usize) -> Self {
        Self { pos, len }
    }
}

impl PartialEq for Span {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
    }
}

impl Eq for Span {}

impl PartialOrd for Span {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.pos.cmp(&other.pos))
    }
}

impl Ord for Span {
    fn cmp(&self, other: &Self) -> Ordering {
        self.pos.cmp(&other.pos)
    }
}
