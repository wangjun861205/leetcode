#[derive(Clone)]
struct WordDictionary {
    is_end: bool,
    children: Vec<Option<WordDictionary>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordDictionary {
    fn new() -> Self {
        WordDictionary {
            is_end: false,
            children: vec![None; 26],
        }
    }

    fn add_word(&mut self, mut word: String) {
        let c = word.remove(0);
        if let Some(child) = &mut self.children[c as usize - 97] {
            if word.is_empty() {
                child.is_end = true;
                return;
            }
            child.add_word(word);
        } else {
            let mut wd = WordDictionary::new();
            if word.is_empty() {
                wd.is_end = true;
                self.children[c as usize - 97] = Some(wd);
                return;
            }
            wd.add_word(word);
            self.children[c as usize - 97] = Some(wd);
        }
    }

    fn search(&self, mut word: String) -> bool {
        let c = word.remove(0);
        if c == '.' {
            if word.is_empty() {
                return self.children.iter().any(|child| {
                    if let Some(cd) = child {
                        cd.is_end
                    } else {
                        false
                    }
                });
            }
            for child in &self.children {
                if let Some(cd) = child {
                    if cd.search(word.clone()) {
                        return true;
                    }
                }
            }
            return false;
        } else {
            if word.is_empty() {
                return if let Some(child) = &self.children[c as usize - 97] {
                    child.is_end
                } else {
                    false
                };
            }
            if let Some(child) = &self.children[c as usize - 97] {
                return child.search(word);
            } else {
                return false;
            }
        }
    }
}

/**
 * Your WordDictionary object will be instantiated and called as such:
 * let obj = WordDictionary::new();
 * obj.add_word(word);
 * let ret_2: bool = obj.search(word);
 */
fn main() {
    println!("Hello, world!");
}
