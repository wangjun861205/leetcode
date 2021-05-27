use std::rc::Rc;
use std::cell::RefCell;

struct TrieElem {
    char: char,
weight: 
}
struct Trie {

}


impl Trie {
    fn new() -> Self {
        Self {
            char: None,
            weight: -1,
            children: Rc::new(RefCell::new(None)),
        }
    }

    fn add(&mut self, word: String) {
        for c in word.chars() {

        }
    }
}

struct WordFilter {
    trie: Trie,
}



impl WordFilter {

    fn new(words: Vec<String>) -> Self {
        
    }
    
    fn f(&self, prefix: String, suffix: String) -> i32 {
        
    }
}

fn main() {
    println!("Hello, world!");
}
