pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    let max_candi = candies.iter().max().unwrap();

    println!("{}", max_candi);

    let comparison_result: Vec<bool> = candies
        .iter()
        .map(|&num| num + extra_candies >= *max_candi)
        .collect();

    comparison_result
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_kids_with_candies() {
        let candies_vec = vec![4, 2, 1, 1, 2];

        let extra_candies = 1;

        let output = vec![true, false, false, false, false];

        assert_eq!(kids_with_candies(candies_vec, extra_candies), output)
    }

    #[test]
    fn test_kids_with_candies_two() {
        let candies_vec = vec![2, 3, 5, 1, 3];

        let extra_candies = 3;

        let output = vec![true, true, true, false, true];

        assert_eq!(kids_with_candies(candies_vec, extra_candies), output)
    }
}
