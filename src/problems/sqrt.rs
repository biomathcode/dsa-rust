pub fn my_sqrt(x: i32) -> i32 {
    (x as f64).sqrt().floor() as i32
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_sqrt() {
        let e = 4;
        assert_eq!(my_sqrt(e), 2);
    }

    #[test]
    fn test_sqrt_two() {
        let e = 8;
        assert_eq!(my_sqrt(e), 2);
    }
}
