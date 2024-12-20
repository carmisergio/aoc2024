use std::{
    collections::HashMap,
    io::{stdin, BufReader},
    process,
};

use day19::parse_input;

fn main() {
    // Read input
    let (patterns, targets) = parse_input(BufReader::new(stdin())).unwrap_or_else(|e| {
        eprintln!("Error reading input: {}", e);
        process::exit(1);
    });

    let mut memo = HashMap::new();

    // Count reachable targets
    let res: u32 = targets
        .iter()
        .map(|target| ways_to_reach(target, &patterns, &mut memo))
        .sum();

    // Print result
    println!("Result: {}", res);
}

/// Check if a target can be reached
fn ways_to_reach(target: &str, patterns: &[String], memo: &mut HashMap<String, u32>) -> u32 {
    // We have reached the target
    if target.is_empty() {
        return 1;
    }

    // Check memoization map
    if let Some(res) = memo.get(target) {
        return *res;
    }

    // Match patterns
    let mut res = 0;
    for pattern in patterns {
        if let Some(remaining) = match_pattern(target, pattern) {
            res += ways_to_reach(remaining, patterns, memo);
        }
    }

    // Insert into memoization map
    memo.insert(target.to_owned(), res);

    res
}

/// Match pattern from target, return remaining characters
fn match_pattern<'a>(target: &'a str, pattern: &str) -> Option<&'a str> {
    if pattern.len() <= target.len() && &target[0..pattern.len()] == pattern {
        Some(&target[pattern.len()..])
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn match_pattern_1() {
        let tests = [
            (("abcdef", "abcd"), Some("ef")),
            (("abcdef", "abcde"), Some("f")),
            (("g", "ubr"), None),
            (("ciaone", "test"), None),
        ];

        for ((target, pattern), exp) in tests {
            let res = match_pattern(target, pattern);
            assert_eq!(res, exp);
        }
    }
}
