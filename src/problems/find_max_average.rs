pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
    let mut current_sum = 0;

    // Calculate the sum of the first window
    for i in 0..k {
        current_sum += nums[i as usize];
    }

    // Initialize max average with the average of the first window
    let mut max_average = (current_sum as f64) / (k as f64);

    // Slide the window through the array
    for i in k as usize..nums.len() {
        current_sum = current_sum - nums[(i as usize) - (k as usize)] + nums[i as usize];
        let current_average = (current_sum as f64) / (k as f64);
        max_average = max_average.max(current_average);
    }

    max_average
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_find_max_average() {
        let nums = vec![1, 12, -5, -6, 50, 3];
        let k = 4;

        let output = 12.75;

        assert_eq!(find_max_average(nums, k), output)
    }

    #[test]
    fn test_find_max_average_two() {
        let nums = vec![5];
        let k = 1;

        let output = 5.0;

        assert_eq!(find_max_average(nums, k), output)
    }
}
