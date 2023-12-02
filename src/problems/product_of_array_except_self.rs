pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();

    let mut left_prod = vec![1;n];

    let mut right_prod = vec![1;n];

    let mut left_product = 1;

    for i in 1..n {
        left_product *= nums[i - 1];
        left_prod[i] = left_product;
    }

    let mut right_product = 1;

    for i in (0..n - 1).rev() {
        right_product *= nums[i + 1];
        right_prod[i] = right_product;
    }

    let mut result = vec![0; n];

    for i in 0..n {
        result[i] = left_prod[i] * right_prod[i];
    }

    result
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_product_of_array_except_self() {
        let input = vec![1, 2, 3, 4];

        let output = vec![24, 12, 8, 6];

        assert_eq!(output, product_except_self(input), "we are testing addition with  and ");
    }
}
