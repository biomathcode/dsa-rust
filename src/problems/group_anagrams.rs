use std::collections::HashMap;

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut el: HashMap<[u8; 26], Vec<String>> = HashMap::new();

    todo!()
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_group_anagrams_one() {
        let str = vec!["eat", "tea", "tan", "ate", "nat", "bat"];
        // let output = vec![["bat"], ["nat", "tan"], ["ate", "eat", "tea"]];

        // assert_eq!(output, group_anagrams(&str), "Done")
    }
}
