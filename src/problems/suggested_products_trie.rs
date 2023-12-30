impl Solution {
    pub fn suggested_products(products: Vec<String>, search_word: String) -> Vec<Vec<String>> {
        #[derive(Clone)]
        struct TrieNode {
            children: Vec<Option<TrieNode>>,
            is_end: bool,
        }
        impl TrieNode {
            fn new() -> Self {
                Self {
                    children: vec![None; 26],
                    is_end: false,
                }
            }
            fn insert(&mut self, ch: char) {
                self.children[(ch as usize) - ('a' as usize)] = Some(TrieNode::new());
            }
            fn get(&self, ch: char) -> Option<&TrieNode> {
                self.children[(ch as usize) - ('a' as usize)].as_ref()
            }
            fn get_mut(&mut self, ch: char) -> Option<&mut TrieNode> {
                self.children[(ch as usize) - ('a' as usize)].as_mut()
            }
        }
        struct Trie {
            root: TrieNode,
        }
        impl Trie {
            fn new() -> Self {
                Trie {
                    root: TrieNode::new(),
                }
            }
            fn insert(&mut self, word: String) {
                let mut node = &mut self.root;
                for ch in word.chars() {
                    if node.get_mut(ch).is_none() {
                        node.insert(ch);
                    }
                    node = node.get_mut(ch).unwrap();
                }
                node.is_end = true;
            }
            fn search(&self, prefix: String) -> Vec<String> {
                let mut result = vec![];
                let mut node = &self.root;
                for ch in prefix.chars() {
                    if node.get(ch).is_none() {
                        return result;
                    }
                    node = node.get(ch).unwrap();
                }
                self.dfs(node, prefix, &mut result);
                result
            }
            fn dfs(&self, node: &TrieNode, prefix: String, result: &mut Vec<String>) {
                if result.len() == 3 {
                    return;
                }
                if node.is_end {
                    result.push(prefix.clone());
                }
                for ch in 'a'..='z' {
                    if let Some(child) = node.get(ch) {
                        self.dfs(child, format!("{}{}", prefix, ch), result);
                    }
                }
            }
        }

        let mut trie = Trie::new();
        for product in products {
            trie.insert(product);
        }

        let mut ans = vec![];
        let mut prefix = String::new();
        for ch in search_word.chars() {
            prefix.push(ch);
            ans.push(trie.search(prefix.clone()));
        }
        ans
    }
}
