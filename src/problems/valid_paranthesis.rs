use std::collections::HashMap;

pub fn is_valid(s: String) -> bool {
    if s.starts_with(")") || s.starts_with("}") || s.starts_with("]") {
        return false;
    }

    let mut stack: Vec<char> = vec![];

    let e: HashMap<char, char> = HashMap::from([
        ('}', '{'),
        (')', '('),
        (']', '['),
    ]);

    for i in s.chars() {
        if e.contains_key(&i) {
            if !stack.is_empty() && stack.last().unwrap().to_owned() == e[&i] {
                stack.pop();
            } else {
                return false;
            }
        } else {
            stack.push(i);
        }
    }

    if stack.is_empty() {
        return true;
    }

    false
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_is_valid() {
        let input = String::from("()");

        assert_eq!(is_valid(input), true)
    }

    #[test]
    fn test_is_valid_two() {
        let input = String::from("([{}])");

        assert_eq!(is_valid(input), true)
    }

    #[test]
    fn test_is_valid_false() {
        let input = String::from("([{]})");

        assert_eq!(is_valid(input), false)
    }
}
