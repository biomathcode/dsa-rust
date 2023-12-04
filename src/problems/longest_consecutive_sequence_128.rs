use std::collections::HashSet;

pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    // create a hashset

    let mut num_set: HashSet<i32> = nums.into_iter().collect();
    let mut longest_sequence = 0;

    // pointer for longest_sequence

    // loop over each el, check if its -1 exists then while loop to check all the +1 elements

    for &num in &num_set {
        if !num_set.contains(&(num - 1)) {
            let mut current_num = num;
            let mut current_streak = 1;

            while num_set.contains(&(current_num + 1)) {
                current_num += 1;
                current_streak += 1;
            }

            longest_sequence = longest_sequence.max(current_streak);
        }
    }

    longest_sequence
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_longest_consecutive_sequence() {
        let input = vec![100, 4, 200, 1, 3, 2];

        assert_eq!(longest_consecutive(input), 4)
    }

    #[test]
    fn test_longest_consecutive_sequence_2() {
        let input = vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1];

        assert_eq!(longest_consecutive(input), 9);
    }

    #[test]
    fn test_wrong() {
        let input = vec![9, 1, 4, 7, 3, -1, 0, 5, 8, -1, 6];

        assert_eq!(longest_consecutive(input), 7)
    }
}
