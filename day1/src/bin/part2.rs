use day1::common::read_lists;

fn main() {
    // Read lists
    let (mut list1, mut list2) = read_lists();

    // Process lists
    let res = compute_similarity(&mut list1, &mut list2);

    // Output result
    println!("Result: {}", res);
}

fn compute_similarity(list1: &mut [i32], list2: &mut [i32]) -> i32 {
    list1
        .iter()
        .fold(0, |acc, el| acc + el * count_occurrences(list2, *el))
}

fn count_occurrences(list: &[i32], val: i32) -> i32 {
    list.iter().filter(|el| **el == val).count() as i32
}

#[cfg(test)]
mod tests {
    use crate::{compute_similarity, count_occurrences};

    #[test]
    fn compute_similarity_test() {
        let tests = [(([3, 4, 2, 1, 3, 3], [4, 3, 5, 3, 9, 3]), 31)];

        for ((mut list1, mut list2), exp) in tests {
            let res = compute_similarity(&mut list1, &mut list2);
            assert_eq!(res, exp);
        }
    }

    #[test]
    fn count_occurrences_test() {
        let tests = [(([1, 2, 2, 3, 2, 1, 2], 2), 4)];

        for ((list, val), exp) in tests {
            let res = count_occurrences(&list, val);
            assert_eq!(res, exp);
        }
    }
}
