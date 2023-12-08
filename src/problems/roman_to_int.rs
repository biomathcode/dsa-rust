pub fn roman_to_int(s: String) -> i32 {
    let mut sum = 0;
    let mut prev_value = 0;
    for i in s.chars().rev() {
        let value = match i {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => panic!("String is not defined"),
        };

        if value < prev_value {
            sum -= value;
        } else {
            sum += value;
        }

        prev_value = value;
    }

    return sum;
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_roman_to_int() {
        let input = String::from("III");

        assert_eq!(roman_to_int(input), 3);
    }

    #[test]
    fn test_roman_to_int_three() {
        let input = String::from("MCMXCIV");

        assert_eq!(roman_to_int(input), 1994);
    }

    #[test]
    fn test_roman_to_int_two() {
        let input = String::from("LVIII");

        assert_eq!(roman_to_int(input), 58);
    }
}
