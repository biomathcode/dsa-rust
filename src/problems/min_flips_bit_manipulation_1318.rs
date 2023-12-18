// min bit flips required to
// satisfy a + b = c

pub fn min_flips(a: i32, b: i32, c: i32) -> i32 {
    let mut count = 0;

    let (mut a, mut b, mut c) = (a, b, c);

    while a > 0 || b > 0 || c > 0 {
        let bit_a = a % 2;
        let bit_b = b % 2;
        let bit_c = c % 2;
        a /= 2;
        b /= 2;
        c /= 2;

        // Check if the current bits are not equal to the corresponding bit in c
        if bit_c == 0 {
            count += bit_a + bit_b;
        } else if bit_a == 0 && bit_b == 0 {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_min_flips() {
        let (a, b, c) = (2, 6, 5);

        assert_eq!(min_flips(a, b, c), 3);
    }

    #[test]
    fn test_min_flips_two() {
        let (a, b, c) = (4, 2, 7);

        assert_eq!(min_flips(a, b, c), 1);
    }

    #[test]
    fn test_min_flips_three() {
        let (a, b, c) = (1, 2, 3);

        assert_eq!(min_flips(a, b, c), 0);
    }
}
