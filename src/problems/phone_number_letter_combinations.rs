pub fn letter_combinations(digits: String) -> Vec<String> {
    if digits.is_empty() {
        return vec![]; // No dial tone here!
    }

    let mut result = Vec::new();
    let phone_map = vec!["abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];

    fn backtracking(
        combination: String, //d
        next_digits: &str, //3
        phone_map: &Vec<&str>,
        output: &mut Vec<String> //
    ) {
        if next_digits.is_empty() {
            output.push(combination); // output ["dg", "dh", "di", "eg",... ]
        } else {
            let letters =
                phone_map[(next_digits.chars().nth(0).unwrap() as usize) - ('2' as usize)]; //ghl

            for letter in letters.chars() {
                // [..ghl]
                let new_combination = combination.clone() + &letter.to_string(); //d + g

                backtracking(new_combination, &next_digits[1..], phone_map, output);
            }
        }
    }
    backtracking(String::new(), &digits, &phone_map, &mut result);

    result
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_first() {
        let digits = "23".to_string();
        let output = vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"];

        assert_eq!(letter_combinations(digits), output)
    }
}
