struct Solution;

impl Solution {
    pub fn are_sentences_similar(sentence1: String, sentence2: String) -> bool {
        let words1: Vec<&str> = sentence1.split(" ").collect();
        let words2: Vec<&str> = sentence2.split(" ").collect();
        let (mut words, mut cmp) = if words1.len() < words2.len() {
            (words1, words2)
        } else {
            (words2, words1)
        };
        while !words.is_empty() {
            let mut removed = false;
            if words.first().unwrap().clone() == cmp.first().unwrap().clone() {
                words.remove(0);
                cmp.remove(0);
                removed = true;
            }
            if words.is_empty() {
                return true;
            }
            if words.last().unwrap().clone() == cmp.last().unwrap().clone() {
                words.pop();
                cmp.pop();
                removed = true;
            }
            if !removed {
                return false;
            }
        }
        words.is_empty()
    }
}

fn main() {
    println!(
        "{}",
        Solution::are_sentences_similar("A A AAa".to_owned(), "A AAa".to_owned())
    );
}
