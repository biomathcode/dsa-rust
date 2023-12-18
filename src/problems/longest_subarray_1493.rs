// 1493. Longest Subarray of 1's After Deleting One Element

// Deleting one

use std::cmp;

pub fn longest_subarray_ones_after_deleting_one_element(nums: Vec<i32>) -> i32 {
    let mut i = 0;

    let mut j = 0;

    let mut n = nums.len();

    let (mut zero_count, mut max_len) = (0, 0);

    while i < nums.len() {
        if nums[i as usize] == 0 {
            zero_count += 1;
        }

        while zero_count > 1 {
            if nums[j] == 0 {
                zero_count -= 1;
            }
            j += 1;
        }

        max_len = cmp::max(max_len, i - j + 1);
        i += 1;
    }

    (max_len - 1) as i32
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_longest_subarray() {
        let input = vec![0, 1, 1, 1, 0, 1, 1, 0, 1];

        assert_eq!(longest_subarray_ones_after_deleting_one_element(input), 5)
    }

    #[test]
    fn test_longest_subarray_two() {
        let input = vec![1, 1, 0, 1];

        assert_eq!(longest_subarray_ones_after_deleting_one_element(input), 3)
    }
}
