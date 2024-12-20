use core::fmt;
use std::{
    error::Error,
    fmt::{Display, Formatter},
    io::{self, BufRead},
};

// Parse error
#[derive(PartialEq, Debug, Clone)]
pub struct PError {}

impl PError {
    pub fn new() -> Self {
        Self {}
    }
}

impl Display for PError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Parse error!")
    }
}

trait OptToResult<T, E> {
    fn or_err(self, err: E) -> Result<T, E>;
}

impl<T, E> OptToResult<T, E> for Option<T> {
    fn or_err(self, err: E) -> Result<T, E> {
        if let Some(val) = self {
            Ok(val)
        } else {
            Err(err)
        }
    }
}

#[derive(Debug)]
pub struct ExecutionError {}

impl ExecutionError {
    pub fn new() -> Self {
        Self {}
    }
}

impl Display for ExecutionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "execution error")
    }
}

impl Error for ExecutionError {}

impl std::error::Error for PError {}

pub fn parse_input(input: impl BufRead) -> Result<(Vec<String>, Vec<String>), Box<dyn Error>> {
    let mut lines = input.lines();

    // Parse available patterns
    let patterns = lines.next().or_err(PError::new())??;
    let patterns: Vec<String> = patterns.split(",").map(|p| p.trim().to_owned()).collect();

    // Parse targets
    let targets: Vec<String> = lines.collect::<Result<Vec<String>, io::Error>>()?;
    let targets = targets
        .into_iter()
        .filter(|line| !line.is_empty())
        .collect();

    Ok((patterns, targets))
}
