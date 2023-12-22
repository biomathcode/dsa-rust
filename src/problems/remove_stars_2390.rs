// Stack in Rust => vec
pub fn remove_stars(s: String) -> String {
    let mut result = String::new();
    let mut skip_next = false;

    for c in s.chars() {
        if skip_next {
            skip_next = false;
        } else if c == '*' {
            // Skip the current character and the next one
            skip_next = true;
        } else {
            result.push(c);
        }
    }

    result
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_remove_stars() {
        let s = String::from("leet**cod*e");

        assert_eq!(remove_stars(s), "lecoe");
    }
}
