pub fn tribonacci(n: i32) -> i32 {
    let mut t = vec![0, 1, 1];

    if n < 3 {
        return t[n as usize];
    } else {
        for _ in 3..(n + 1) as usize {
            let next_tribonacci = t[0] + t[1] + t[2];

            t[0] = t[1];
            t[1] = t[2];
            t[2] = next_tribonacci;
        }
        return t[2];
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_tribonacci_number() {
        let input = 4;

        assert_eq!(tribonacci(input), 4);
    }
}
