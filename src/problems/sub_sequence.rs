pub fn is_subsequence(s: String, t: String) -> bool {
    let mut iter = t.chars();
    for c in s.chars() {
        match iter.find(|&p| p == c) {
            Some(_) => (),
            None => {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_sub_sequence() {
        let s = String::from("acb");

        let t = String::from("ahbgdc");

        assert_eq!(is_subsequence(s, t), true)
    }

    #[test]
    fn test_() {
        let s = String::from("abc");

        let t = String::from("ahbgdc");

        assert_eq!(is_subsequence(s, t), true)
    }
}
