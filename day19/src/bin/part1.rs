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

    // Count reachable targets
    let res = targets
        .iter()
        .filter(|target| is_target_reachable(target, &patterns, &mut HashMap::new()))
        .count();

    // Print result
    println!("Result: {}", res);
}

/// Check if a target can be reached
fn is_target_reachable(
    target: &str,
    patterns: &[String],
    memo: &mut HashMap<String, bool>,
) -> bool {
    // We have reached the target
    if target.is_empty() {
        return true;
    }

    // Check memoization map
    if let Some(res) = memo.get(target) {
        return *res;
    }

    // Match patterns
    let mut res = false;
    for pattern in patterns {
        if let Some(remaining) = match_pattern(target, pattern) {
            if is_target_reachable(remaining, patterns, memo) {
                res = true;
                break;
            }
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
