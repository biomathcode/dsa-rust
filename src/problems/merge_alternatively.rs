pub fn merge_alternately(word1: String, word2: String) -> String {
    let mut w1_iter = word1.chars().peekable();
    let mut w2_iter = word2.chars().peekable();

    let mut result = String::new();

    loop {
        if let Some(c1) = w1_iter.next() {
            result.push(c1);
        }

        if let Some(c2) = w2_iter.next() {
            result.push(c2);
        }

        if w1_iter.peek() == None && w2_iter.peek() == None {
            return result;
        }
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_merge_alternatively() {
        let word1 = String::from("abc");
        let word2 = String::from("pqr");

        assert_eq!(merge_alternately(word1, word2), "apbqcr")
    }
}
