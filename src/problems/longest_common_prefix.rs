// TODO: REPEATE THIS

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.is_empty() {
        return String::new(); // Return an empty string for an empty input.
    }

    let mut prefix = String::new();

    for (i, char_i) in strs[0].chars().enumerate() {
        if strs[1..].iter().all(|s| s.chars().nth(i) == Some(char_i)) {
            prefix.push(char_i);
        } else {
            break;
        }
    }

    prefix

    // pseudo solution
    // get all the combination of first
    // loop over and se the longest one eist or not
    // retun the longest prefix
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_longest_common_prefix() {
        let input = vec![String::from("flower"), String::from("flow"), String::from("flight")];

        assert_eq!(longest_common_prefix(input), "fl")
    }

    #[test]
    fn test_longest_common_prefix_two() {
        let input = vec![String::from("dog"), String::from("racecar"), String::from("car")];

        assert_eq!(longest_common_prefix(input), "")
    }
}
