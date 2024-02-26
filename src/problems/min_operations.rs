use std::collections::HashMap;

pub fn min_operations(nums: Vec<i32>) -> i32 {
    let mut hashmap = HashMap::new();

    let mut operations = 0;

    for &i in &nums {
        let count = hashmap.entry(i).or_insert(0);
        *count += 1;
    }

    for &value in hashmap.values() {
        if value == 1 {
            return -1;
        }
        operations += ((value as f64) / 3.0).ceil() as i32;
    }

    return operations;
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_() {
        let nums = vec![2, 3, 3, 2, 2, 4, 2, 3, 4];

        assert_eq!(min_operations(nums), 4)
    }
}
