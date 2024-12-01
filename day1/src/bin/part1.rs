use day1::common::read_lists;

fn main() {
    // Read lists
    let (mut list1, mut list2) = read_lists();

    // Process lists
    let res = compute_difference(&mut list1, &mut list2);

    // Output result
    println!("Result: {}", res);
}

fn compute_difference(list1: &mut [i32], list2: &mut [i32]) -> i32 {
    // Sort lists
    list1.sort();
    list2.sort();

    list1
        .iter()
        .zip(list2)
        .fold(0, |acc, (el1, el2)| acc + (*el1 - *el2).abs())
}

#[cfg(test)]
mod tests {
    use crate::compute_difference;

    #[test]
    fn compute_difference_test() {
        let tests = [(([3, 4, 2, 1, 3, 3], [4, 3, 5, 3, 9, 3]), 11)];

        for ((mut list1, mut list2), exp) in tests {
            let res = compute_difference(&mut list1, &mut list2);
            assert_eq!(res, exp);
        }
    }
}
