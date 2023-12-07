pub fn move_zeroes(nums: &mut Vec<i32>) {
    let mut last_index_zero = 0;

    for i in 0..nums.len() {
        if nums[i] != 0 {
            nums.swap(i, last_index_zero);
            last_index_zero += 1;
        }
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_() {
        todo!()
    }
}
