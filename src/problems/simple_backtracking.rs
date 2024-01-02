fn generate_permutations(nums: &Vec<i32>) -> Vec<Vec<i32>> {
    if nums.is_empty() {
        return vec![vec![]];
    }

    let mut result = Vec::new();

    for (i, &num) in nums.iter().enumerate() {
        let mut remaining_nums = nums.clone();
        remaining_nums.remove(i);

        let mut partial_permutations = generate_permutations(&remaining_nums);

        for perm in &mut partial_permutations {
            perm.insert(0, num);
        }

        result.extend_from_slice(&partial_permutations);
    }

    result
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_backtracking() {
        let mut nums = Vec::from([1, 2, 3]);

        let output = vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],

            vec![3, 1, 2],
            vec![3, 2, 1]
        ];

        let result = generate_permutations(&mut nums);

        assert_eq!(result, output)
    }
}
