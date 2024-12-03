use day2::{is_safe, read_reports};

fn main() {
    let reports = read_reports();
    let n_safe_reports = reports
        .into_iter()
        .filter(|report| {
            let safe = is_safe(report);
            safe
        })
        .count();
    println!("Number of safe reports: {}", n_safe_reports);
}
