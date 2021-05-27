struct Solution;

impl Solution {
    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        word1.join("") == word2.join("")
    }
}
fn main() {
    println!(
        "{}",
        Solution::array_strings_are_equal(
            vec!["a".into(), "bc".into()],
            vec!["ab".into(), "c".into()]
        )
    );
}
