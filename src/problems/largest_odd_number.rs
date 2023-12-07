// Intuition
// Odd number should end with odd digit at the end. Now when you found an odd digit, you need to append any digits in front of it.

// So basically you need to go from right to left until you find an odd character

// Complexity
// Time complexity: O(n)
// Space complexity: O(1)

pub fn largest_odd_number(num: String) -> String {
    let data = num.as_bytes();

    print!("{:?}", data);

    for i in (0..num.len()).rev() {
        if data[i] % 2 == 1 {
            return num[..i + 1].to_string();
        }
    }

    return "".to_string();
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_largest_odd_number() {
        let input = String::from("52");

        assert_eq!(largest_odd_number(input), "5");
    }

    #[test]
    fn test_largest_odd_number_2() {
        let input = String::from("4206");

        assert_eq!(largest_odd_number(input), "")
    }

    #[test]
    fn test_largest_odd_number_wrong() {
        let input = String::from("35427");
        assert_eq!(largest_odd_number(input), "35427")
    }

    #[test]
    fn test_largest_odd_number_larger() {
        let input_number = String::from("239537672423884969653287101");
        let result = largest_odd_number(input_number);

        assert_eq!(result, "239537672423884969653287101");
    }
}
