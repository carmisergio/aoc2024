use std::{
    io::{stdin, BufReader},
    process,
};

use day5::{parse_input, OrderingRule, Update};

fn main() {
    let input = parse_input(BufReader::new(stdin())).unwrap_or_else(|e| {
        eprintln!("Read error: {}", e);
        process::exit(1);
    });

    let result = compute(&input);

    println!("Result: {}", result);
}

fn compute((ord_rules, updates): &(Vec<OrderingRule>, Vec<Update>)) -> usize {
    let mut res = 0;
    for update in updates {
        if check_update(&ord_rules, update) {
            res += update[update.len() / 2] as usize;
        }
    }

    res
}

fn check_update(ord_rules: &[OrderingRule], update: &Update) -> bool {
    for rule in ord_rules {
        if let Some(res) = validate_ord_rule(rule, update) {
            if !res {
                return false;
            }
        }
    }

    return true;
}

fn validate_ord_rule(rule: &OrderingRule, update: &Update) -> Option<bool> {
    // Find element positions
    let p0 = update.iter().position(|el| *el == rule.0)?;
    let p1 = update.iter().position(|el| *el == rule.1)?;

    Some(p1 > p0)
}

#[cfg(test)]
mod tests {
    use crate::validate_ord_rule;

    #[test]
    fn validate_org_rule_1() {
        let tests = [
            (((10, 20), vec![45, 10, 34, 56, 20, 12]), Some(true)),
            (((10, 20), vec![45, 20, 34, 56, 10, 12]), Some(false)),
            (((10, 30), vec![45, 20, 34, 56, 10, 12]), None),
        ];

        for ((rule, update), exp) in tests {
            let res = validate_ord_rule(&rule, &update);
            assert_eq!(res, exp);
        }
    }
}
