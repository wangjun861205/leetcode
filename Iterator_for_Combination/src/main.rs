struct CombinationIterator {
    chars: Vec<char>,
    indices: Vec<usize>,
    has_next: bool,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CombinationIterator {
    fn new(characters: String, combinationLength: i32) -> Self {
        Self {
            chars: characters.chars().collect(),
            indices: (0..combinationLength as usize).collect(),
            has_next: true,
        }
    }

    fn next(&mut self) -> String {
        let mut s = String::new();
        for idx in &self.indices {
            s.push(self.chars[*idx]);
        }
        for i in (0..self.indices.len()).rev() {
            let next = *self.indices.get(i + 1).unwrap_or(&self.chars.len());
            if next - self.indices[i] > 1 {
                self.indices[i] += 1;
                let mut curr = self.indices[i];
                for j in i + 1..self.indices.len() {
                    curr += 1;
                    self.indices[j] = curr;
                }
                return s;
            }
        }
        self.has_next = false;
        s
    }

    fn has_next(&self) -> bool {
        self.has_next
    }
}

/**
 * Your CombinationIterator object will be instantiated and called as such:
 * let obj = CombinationIterator::new(characters, combinationLength);
 * let ret_1: String = obj.next();
 * let ret_2: bool = obj.has_next();
 */
fn main() {
    let mut iter = CombinationIterator::new("ahijp".to_owned(), 2);
    println!("{}", iter.next());
    println!("{}", iter.has_next());
    println!("{}", iter.next());
    println!("{}", iter.has_next());
    println!("{}", iter.next());
    println!("{}", iter.has_next());
    println!("{}", iter.next());
    println!("{}", iter.has_next());
    println!("{}", iter.next());
    println!("{}", iter.has_next());
    println!("{}", iter.next());
    println!("{}", iter.has_next());
}
