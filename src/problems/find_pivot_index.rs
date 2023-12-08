/*
Given an array of integers nums, calculate the pivot index of this array.

The pivot index is the index where the sum of all the numbers strictly to the left of the index is equal to the sum of all the numbers strictly to the index's right.

If the index is on the left edge of the array, then the left sum is 0 because there are no elements to the left. This also applies to the right edge of the array.

Return the lef tmost pivot index. If no such index exists, return -1.
*/

//TODO

pub fn pivot_index(nums: Vec<i32>) -> i32 {
    1
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_find_pivot_index() {
        let index = vec![1, 7, 3, 6, 5, 6];

        assert_eq!(pivot_index(index), 3)
    }

    #[test]
    fn test_find_pivot_index_two() {
        let index = vec![1, 2, 3];

        assert_eq!(pivot_index(index), -1) // -1
    }

    #[test]
    fn test_find_pivot_index_three() {
        let index = vec![2, 1, -1];
        assert_eq!(pivot_index(index), 0) // -1
    }
}
