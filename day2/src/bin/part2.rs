use day2::{is_safe, read_reports, Report};

fn main() {
    let reports = read_reports();
    let n_safe_reports = reports
        .into_iter()
        .filter(|report| {
            let safe = is_safe_damped(report);
            safe
        })
        .count();
    println!("Number of safe reports: {}", n_safe_reports);
}

/// Check safety of report with problem damper
fn is_safe_damped(report: &Report) -> bool {
    for idx in 0..report.len() {
        // Create new report with value removed
        let new_report = remove_value(report, idx);

        if is_safe(&new_report) {
            return true;
        }
    }
    return false;
}

/// Remove value from report
fn remove_value(report: &Report, idx: usize) -> Report {
    (0..report.len())
        .filter(|i| *i != idx)
        .map(|i| report[i])
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{is_safe_damped, remove_value};

    #[test]
    fn is_safe_damped_test() {
        let tests = [
            (vec![7, 6, 4, 2, 1], true),
            (vec![1, 2, 7, 8, 9], false),
            (vec![9, 7, 6, 2, 1], false),
            (vec![1, 3, 2, 4, 5], true),
            (vec![8, 6, 4, 4, 1], true),
            (vec![1, 3, 6, 7, 9], true),
        ];

        for (input, exp) in tests {
            let res = is_safe_damped(&input);
            assert_eq!(res, exp);
        }
    }

    #[test]
    fn remove_value_test() {
        let tests = [
            ((vec![7, 6, 4, 2, 1], 0), vec![6, 4, 2, 1]),
            ((vec![7, 6, 4, 2, 1], 1), vec![7, 4, 2, 1]),
            ((vec![7, 6, 4, 2, 1], 4), vec![7, 6, 4, 2]),
        ];

        for ((report, idx), exp) in tests {
            let res = remove_value(&report, idx);
            assert_eq!(res, exp);
        }
    }
}
