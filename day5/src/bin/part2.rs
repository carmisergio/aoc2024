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

    let res = compute(input);

    println!("Result: {}", res);
}

fn compute((ord_rules, updates): (Vec<OrderingRule>, Vec<Update>)) -> usize {
    let mut res = 0;
    for update in updates {
        if !check_update(&ord_rules, &update) {
            let update = reorder_update(update, &ord_rules);
            res += update[update.len() / 2] as usize;
        }
    }

    res
}

fn validate_ord_rule(rule: &OrderingRule, update: &Update) -> Option<bool> {
    // Find element positions
    let p0 = update.iter().position(|el| *el == rule.0)?;
    let p1 = update.iter().position(|el| *el == rule.1)?;

    Some(p1 > p0)
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

fn reorder_update(mut update: Update, ord_rules: &[OrderingRule]) -> Update {
    while !check_update(&ord_rules, &update) {
        for rule in ord_rules {
            if let Some((p0, p1)) = find_positions(rule, &update) {
                if p1 < p0 {
                    let tmp = update[p1];
                    update[p1] = update[p0];
                    update[p0] = tmp;
                }
            }
        }
    }

    update
}

fn find_positions(rule: &OrderingRule, update: &Update) -> Option<(usize, usize)> {
    let p0 = update.iter().position(|el| *el == rule.0)?;
    let p1 = update.iter().position(|el| *el == rule.1)?;

    Some((p0, p1))
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn reorder_update_test() {
        let ord_rules = vec![
            (47, 53),
            (97, 13),
            (97, 61),
            (97, 47),
            (75, 29),
            (61, 13),
            (75, 53),
            (29, 13),
            (97, 29),
            (53, 29),
            (61, 53),
            (97, 53),
            (61, 29),
            (47, 13),
            (75, 47),
            (97, 75),
            (47, 61),
            (75, 61),
            (47, 29),
            (75, 13),
            (53, 13),
        ];

        let tests = [
            (vec![75, 97, 47, 61, 53], vec![97, 75, 47, 61, 53]),
            (vec![61, 13, 29], vec![61, 29, 13]),
            (vec![97, 13, 75, 29, 47], vec![97, 75, 47, 29, 13]),
            (vec![75, 47, 61, 53, 29], vec![75, 47, 61, 53, 29]),
        ];

        for (update, exp) in tests {
            let res = reorder_update(update, &ord_rules);
            assert_eq!(res, exp);
        }
    }
}
