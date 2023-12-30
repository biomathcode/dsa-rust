#[derive(Default, Clone)]
struct TrieNode {
    children: Vec<Option<Box<TrieNode>>>,
    is_end_of_word: bool,
}

struct Trie {
    root: TrieNode,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: vec![None; 26], // Assuming lowercase English alphabet
            is_end_of_word: false,
        }
    }
}

impl Trie {
    fn new() -> Self {
        Trie {
            root: TrieNode::new(),
        }
    }

    fn insert(&mut self, word: &str) {
        let mut current = &mut self.root;
        for &ch in word.as_bytes() {
            let index = (ch - b'a') as usize;
            current = current.children[index].get_or_insert_with(|| Box::new(TrieNode::new()));
        }
        current.is_end_of_word = true;
    }

    fn search(&self, word: &str) -> bool {
        let mut current = &self.root;
        for &ch in word.as_bytes() {
            let index = (ch - b'a') as usize;
            match &current.children[index] {
                Some(node) => {
                    current = node;
                }
                None => {
                    return false;
                }
            }
        }
        current.is_end_of_word
    }

    fn starts_with_prefix(&self, prefix: &str) -> bool {
        let mut current = &self.root;
        for &ch in prefix.as_bytes() {
            let index = (ch - b'a') as usize;
            match &current.children[index] {
                Some(node) => {
                    current = node;
                }
                None => {
                    return false;
                }
            }
        }
        true
    }
}

fn main() {
    let mut trie = Trie::new();

    trie.insert("apple");
    trie.insert("app");

    println!("{}", trie.search("apple")); // Output: true
    println!("{}", trie.search("app")); // Output: true
    println!("{}", trie.search("ap")); // Output: false
    println!("{}", trie.search("orange")); // Output: false

    println!("{}", trie.starts_with_prefix("app")); // Output: true
    println!("{}", trie.starts_with_prefix("ora")); // Output: false
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_trie_with_vec() {
        let mut trie = Trie::new();

        trie.insert("apple");
        trie.insert("app");

        println!("{}", trie.search("apple")); // Output: true
        println!("{}", trie.search("app")); // Output: true
        println!("{}", trie.search("ap")); // Output: false
        println!("{}", trie.search("orange")); // Output: false

        println!("{}", trie.starts_with_prefix("app")); // Output: true
        println!("{}", trie.starts_with_prefix("ora")); // Output: false
    }
}
