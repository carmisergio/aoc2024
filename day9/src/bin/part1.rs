use std::{
    error::Error,
    io::{stdin, BufRead, BufReader},
    process,
};

use day9::{build_array, compute_checksum, IntoAsciiChars, ParseError, Span};

fn main() {
    // Read input
    let (arr_len, files) = parse_input(BufReader::new(stdin())).unwrap_or_else(|e| {
        eprintln!("Unable to read input: {}", e);
        process::exit(1);
    });

    // Build array from files list
    let mut arr = build_array(arr_len, &files);

    // Fragmentate
    fragment(&mut arr);

    // Compute checksum
    let res = compute_checksum(&arr);

    // Print result
    println!("Checksum: {}", res);
}

fn parse_input(mut input: impl BufRead) -> Result<(usize, Vec<Span>), Box<dyn Error>> {
    let mut pos = 0;
    let mut files: Vec<Span> = Vec::new();

    for (idx, c) in input.ascii_chars().enumerate() {
        let len = match c?.to_digit(10) {
            Some(len) => Ok::<usize, Box<dyn Error>>(len as usize),
            None => return Err(Box::new(ParseError::new())),
        }?;

        let len = len as usize;

        if idx % 2 == 0 {
            // File
            files.push(Span::new(pos, len));
        }

        pos += len;
    }

    // Compute array size
    let arr_size = if let Some(span) = files.last() {
        span.pos + span.len
    } else {
        0
    };

    Ok((arr_size, files))
}

fn fragment(arr: &mut [Option<usize>]) {
    // Indexes
    let mut pick = arr.len() - 1;
    let mut place = 0;

    while pick != place {
        // Find element to pick
        let mut pick_el = None;
        for i in 0..pick {
            if let None = arr[pick - i] {
                continue;
            }

            pick = pick - i;

            pick_el = arr[pick];
            arr[pick] = None;

            break;
        }

        // Stop if no element to pick
        if let None = pick_el {
            break;
        }

        // Find spot to place
        for i in place..arr.len() {
            if let None = arr[i] {
                place = i;
                break;
            }
        }

        // Place element in new spot
        arr[place] = pick_el;
    }
}
