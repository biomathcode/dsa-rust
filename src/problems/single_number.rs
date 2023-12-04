pub fn single_number(nums: Vec<i32>) -> i32 {
    let mut result = 0;

    for num in nums {
        result ^= num; // XOR Bit Manipulation
    }

    result
}

#[cfg(test)]
mod test_ {
    use crate::single_number;

    use super::*;

    #[test]
    fn test_single_numbers() {
        let input = vec![2, 2, 1];

        assert_eq!(single_number(input), 1)
    }
}
