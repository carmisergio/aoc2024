use day3::{extract_operations, read_input, Operation};

fn main() {
    // Read input from stdin
    let input = read_input();

    // Get multiplications
    let ops = extract_operations(&input);

    // Compute result
    let mut sum = 0;
    let mut enabled = true;

    for op in ops {
        match op {
            Operation::Mul(a, b) => {
                if enabled {
                    sum += a as i32 * b as i32
                }
            }
            Operation::Do => enabled = true,
            Operation::Dont => enabled = false,
        }
    }

    println!("Result: {}", sum);
}
