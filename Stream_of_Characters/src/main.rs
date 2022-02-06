struct Solution;

struct Node {
    char: char,
    children: Vec<Box<Node>>,
}

struct StreamChecker {
    root: Node,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StreamChecker {
    fn new(words: Vec<String>) -> Self {}

    fn query(&self, letter: char) -> bool {}
}

/**
 * Your StreamChecker object will be instantiated and called as such:
 * let obj = StreamChecker::new(words);
 * let ret_1: bool = obj.query(letter);
 */

fn main() {
    println!("Hello, world!");
}
