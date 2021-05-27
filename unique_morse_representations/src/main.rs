use std::collections::{HashMap, HashSet};

pub fn unique_morse_representations(words: Vec<String>) -> i32 {
    let morses = vec![
        ".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..", "--",
        "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-", "-.--",
        "--..",
    ];
    let letters = "abcdefghijklmnopqrstuvwxyz";
    let map: HashMap<char, &str> = letters.chars().zip(morses).collect();
    let mut count_set: HashSet<String> = HashSet::new();
    for w in words {
        let mut s = String::new();
        for c in w.chars() {
            s.push_str(&map.get(&c).unwrap());
        }
        count_set.insert(s);
    }
    count_set.len() as i32
}

fn main() {
    println!("Hello, world!");
}
