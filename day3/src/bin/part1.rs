use day3::{extract_operations, read_input, Operation};

fn main() {
    // Read input from stdin
    let input = read_input();

    // Get multiplications
    let mults = extract_operations(&input);

    // Compute result
    let sum = mults.iter().fold(0, |acc, op| {
        if let Operation::Mul(a, b) = op {
            acc + *a as i32 * *b as i32
        } else {
            acc
        }
    });

    println!("Result: {}", sum);
}
