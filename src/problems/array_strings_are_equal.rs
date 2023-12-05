pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
    let w1 = word1.join("");

    let w2 = word2.join("");

    w1 == w2
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_array_strings_are_equal() {
        let w1: Vec<String> = vec![String::from("ab"), String::from("c")];

        let w2: Vec<String> = vec![String::from("a"), String::from("bc")];

        assert_eq!(array_strings_are_equal(w1, w2), true)
    }

    #[test]
    fn test_array_strings_are_equal_two() {
        let w1: Vec<String> = vec![String::from("a"), String::from("cb")];

        let w2: Vec<String> = vec![String::from("ab"), String::from("c")];

        assert_eq!(array_strings_are_equal(w1, w2), false)
    }
}
