pub fn trap(height: Vec<i32>) -> i32 {
    todo!()
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_trap_rain_water() {
        let input = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        let output = 6;

        assert_eq!(trap(input), output)
    }

    fn test_trap_rain_two() {
        let input = vec![4, 2, 0, 3, 2, 5];
        let output = 9;

        assert_eq!(trap(input), output)
    }
}
