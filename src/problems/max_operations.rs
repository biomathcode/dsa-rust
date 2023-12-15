// max  number of unique pair in an array whose sum is k;

use std::collections::{ HashMap, HashSet };

pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
    let mut count = 0;
    let mut num_count = HashMap::new();

    for &num in &nums {
        let complement = k - num;

        if num_count.get(&complement).map_or(0, |&count| count) > 0 {
            // Found a pair
            count += 1;
            *num_count.entry(complement).or_insert(0) -= 1;
        } else {
            // Increment the count of the current element
            *num_count.entry(num).or_insert(0) += 1;
        }
    }

    count
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_max_operations() {
        let nums = vec![1, 2, 3, 4];
        let k = 5;

        assert_eq!(max_operations(nums, k), 2);
    }

    #[test]
    fn test_max_operations_two() {
        let nums = vec![3, 1, 3, 4, 3];
        let k = 6;

        assert_eq!(max_operations(nums, k), 1);
    }

    #[test]
    fn test_max_operations_three() {
        let nums = vec![2, 5, 4, 4, 1, 3, 4, 4, 1, 4, 4, 1, 2, 1, 2, 2, 3, 2, 4, 2];
        let k = 3;

        assert_eq!(max_operations(nums, k), 4);
    }
}
