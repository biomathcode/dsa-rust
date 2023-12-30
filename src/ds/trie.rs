/*
A trie (also called a prefix tree) is a special data structure that is used for efficiently storing and retriving data with a significant amount of overlap. For example, if you were to store an English dictionary in memory, you could use an array or a hashset. However, since there is a significant amount of overlap in the English alphabet (only 26 letters in total), it would be much more efficient to store the dictionary in a tree-like structure where words that share a sequence of letters share a sequence of nodes. As a bonus, this structure allows you to search all words that begin with a given prefix like 'run' yielding 'runner', 'runs', and 'runway'.

Even though you could use an char[26] array to store the keys, I would recommend using a hashmap instead because using a char array restricts you to the English alphabet.
*/

use std::collections::HashMap;

#[derive(Default)]
struct Trie {
    children: HashMap<char, Trie>, // use a hashmap to implement trie
    is_leaf: bool,
}

impl Trie {
    fn new() -> Self {
        Trie::default()
    }

    fn insert(&mut self, word: String) {
        word.chars().fold(self, |node, c| node.children.entry(c).or_default()).is_leaf = true;
    }

    fn get(&self, word: String) -> Option<&Trie> {
        word.chars().try_fold(self, |node, c| node.children.get(&c))
    }

    fn search(&self, word: String) -> bool {
        self.get(word).map_or(false, |node| node.is_leaf)
    }

    fn starts_with(&self, prefix: String) -> bool {
        self.get(prefix).is_some()
    }
}
