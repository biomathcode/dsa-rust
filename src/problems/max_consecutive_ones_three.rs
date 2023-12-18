pub fn longest_ones(nums: Vec<i32>, mut k: i32) -> i32 {
    let mut i: i32 = 0;
    let mut max_cons: i32 = 0;

    while i < (nums.len() as i32) {
        if nums[i as usize] == 0 {
            k -= 1;
        }
        if k < 0 {
            if nums[max_cons as usize] == 0 {
                k += 1;
            }

            max_cons += 1;
        }

        i += 1;
    }

    return i - max_cons;
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_longest_consecutive_ones() {
        let input = vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0];

        let k = 2;

        assert_eq!(longest_ones(input, k), 6)
    }

    #[test]
    fn test_longest_consecutive_ones_wrong() {
        let input = vec![0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1];

        let k = 3;

        assert_eq!(longest_ones(input, k), 10)
    }
}
