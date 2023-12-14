// such that i < j < k and nums[i] < nums[j] < nums[k].
// If no such indices exists, return false

pub fn increasing_triplet(nums: Vec<i32>) -> bool {
    let mut first_min = i32::MAX;
    let mut second_min = i32::MAX;

    for num in nums {
        if num <= first_min {
            first_min = num;
        } else if num <= second_min {
            second_min = num;
        } else {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_increasing_triplet() {
        let input = vec![1, 2, 3, 4, 5];

        assert_eq!(increasing_triplet(input), true);
    }

    #[test]
    fn test_increasing_triplet_two() {
        let input = vec![5, 4, 3, 2, 1];

        assert_eq!(increasing_triplet(input), false);
    }

    #[test]
    fn test_increasing_triplet_three() {
        let input = vec![2, 1, 5, 0, 4, 6];

        assert_eq!(increasing_triplet(input), true);
    }
}
