struct Solution;

use std::collections::HashMap;

#[derive(Debug)]
struct Node {
    words: Vec<String>,
    children: HashMap<char, Box<Node>>,
}

impl Node {
    fn new() -> Self {
        Self {
            words: Vec::new(),
            children: HashMap::new(),
        }
    }

    fn add(&mut self, mut chars: Vec<char>, word: String) {
        self.words.push(word.clone());
        if chars.is_empty() {
            return;
        }
        let first_char = chars.remove(0);
        self.children
            .entry(first_char)
            .or_insert(Box::new(Node::new()))
            .add(chars, word);
    }

    fn search(&mut self, mut chars: Vec<char>) -> Option<Vec<String>> {
        let first_char = chars.remove(0);
        if let Some(child) = self.children.get_mut(&first_char) {
            if chars.len() == 0 {
                child.words.sort();
                return Some(child.words[..3.min(child.words.len())].to_vec());
            } else {
                return child.search(chars);
            }
        }
        None
    }
}

#[derive(Debug)]
struct Trie(HashMap<char, Box<Node>>);

impl Trie {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn add(&mut self, word: String) {
        let mut chars: Vec<char> = word.chars().collect();
        let first_char = chars.remove(0);
        self.0
            .entry(first_char)
            .or_insert(Box::new(Node::new()))
            .add(chars, word);
    }

    fn search(&mut self, mut chars: Vec<char>) -> Option<Vec<String>> {
        let first_char = chars.remove(0);
        if let Some(node) = self.0.get_mut(&first_char) {
            if chars.is_empty() {
                node.words.sort();
                return Some(node.words[..3.min(node.words.len())].to_vec());
            } else {
                return node.search(chars);
            }
        }
        None
    }
}

impl Solution {
    pub fn suggested_products(products: Vec<String>, search_word: String) -> Vec<Vec<String>> {
        let mut trie = Trie::new();
        for p in products {
            trie.add(p);
        }
        let search_chars: Vec<char> = search_word.chars().collect();
        (1..=search_chars.len())
            .into_iter()
            .map(|end| {
                trie.search(search_chars[..end].to_vec())
                    .unwrap_or(Vec::new())
            })
            .collect()
    }
}
fn main() {
    println!(
        "{:?}",
        Solution::suggested_products(
            vec!["mobile", "mouse", "moneypot", "monitor", "mousepad"]
                .into_iter()
                .map(|v| v.to_owned())
                .collect(),
            "mouse".to_owned()
        )
    );
}
