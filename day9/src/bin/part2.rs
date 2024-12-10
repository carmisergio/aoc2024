use std::{
    collections::BTreeSet,
    error::Error,
    io::{stdin, BufRead, BufReader},
    process,
};

use day9::{build_array, compute_checksum, IntoAsciiChars, ParseError, Span};

fn main() {
    // Read input
    let (arr_len, (files, empty_spaces)) =
        parse_input(BufReader::new(stdin())).unwrap_or_else(|e| {
            eprintln!("Unable to read input: {}", e);
            process::exit(1);
        });

    // Compact filesystem without fragmentation
    let files = compact_nofragment(files, empty_spaces);

    // Build file array
    let arr = build_array(arr_len, &files);

    // Compute checksum
    let res = compute_checksum(&arr);

    // Print result
    println!("Checksum: {}", res);
}

fn parse_input(mut input: impl BufRead) -> Result<(usize, (Vec<Span>, Vec<Span>)), Box<dyn Error>> {
    let mut pos = 0;
    let mut files: Vec<Span> = Vec::new();
    let mut empty_spaces: Vec<Span> = Vec::new();

    for (idx, c) in input.ascii_chars().enumerate() {
        let len = match c?.to_digit(10) {
            Some(len) => Ok::<usize, Box<dyn Error>>(len as usize),
            None => return Err(Box::new(ParseError::new())),
        }?;

        let len = len as usize;

        if idx % 2 == 0 {
            // File
            files.push(Span::new(pos, len));
        } else {
            empty_spaces.push(Span::new(pos, len));
        }

        pos += len;
    }

    // Compute array size
    let arr_size = if let Some(span) = files.last() {
        span.pos + span.len
    } else {
        0
    };

    Ok((arr_size, (files, empty_spaces)))
}

fn compact_nofragment(mut files: Vec<Span>, empty_spaces: Vec<Span>) -> Vec<Span> {
    let mut empty_spaces = BTreeSet::from_iter(empty_spaces);

    for i in (0..files.len()).rev() {
        // Find suitable empty space
        let space = get_empty_space(&mut empty_spaces, files[i]);

        let space = match space {
            Some(space) => space,
            None => continue,
        };

        // Add empty space where the file was
        empty_spaces.insert(files[i]);

        // Modify file
        files[i].pos = space.pos;

        if space.len > files[i].len {
            // Divide space
            empty_spaces.insert(Span::new(
                space.pos + files[i].len,
                space.len - files[i].len,
            ));
        }
    }

    files
}

fn get_empty_space(spaces: &mut BTreeSet<Span>, file: Span) -> Option<Span> {
    let space = spaces
        .iter()
        .filter(|space| space.len >= file.len && space.pos < file.pos)
        .map(|el| *el)
        .next()?;

    spaces.remove(&space);

    Some(space)
}
