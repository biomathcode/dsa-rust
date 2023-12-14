use std::collections::HashSet;

pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();

    // Convert vectors to sets for efficient difference operation
    let set1: HashSet<_> = nums1.into_iter().collect();
    let set2: HashSet<_> = nums2.into_iter().collect();

    // Find the difference between the two sets in both directions
    let mut difference1: Vec<_> = set1.difference(&set2).cloned().collect();
    let mut difference2: Vec<_> = set2.difference(&set1).cloned().collect();

    difference1.sort();

    difference2.sort();

    result.push(difference1);

    result.push(difference2);

    result
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_find_difference() {
        let vec1 = vec![1, 2, 3];
        let vec2 = vec![2, 4, 6];

        let output = vec![[1, 3], [4, 6]];

        assert_eq!(find_difference(vec1, vec2), output)
    }
}
