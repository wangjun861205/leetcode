use std::collections::HashMap;

struct MapSum {
    m: HashMap<String, i32>,
    keys: HashMap<String, i32>,
}

impl MapSum {
    fn new() -> Self {
        Self {
            m: HashMap::new(),
            keys: HashMap::new(),
        }
    }

    fn insert(&mut self, key: String, val: i32) {
        let mut s = String::new();
        if self.keys.contains_key(&key) {
            let ori = *self.keys.get(&key).unwrap();
            for c in key.chars() {
                s.push(c);
                *self.m.entry(s.clone()).or_insert(0) += val - ori;
            }
            self.keys.insert(key, val);
        } else {
            for c in key.chars() {
                s.push(c);
                *self.m.entry(s.clone()).or_insert(0) += val;
            }
            self.keys.insert(key, val);
        }
    }

    fn sum(&self, prefix: String) -> i32 {
        *self.m.get(&prefix).unwrap_or(&0)
    }
}

fn main() {
    println!("Hello, world!");
}
