struct MyHashSet {
    set: Vec<bool>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashSet {
    fn new() -> Self {
        Self {
            set: vec![false; 10i32.pow(6) as usize + 1],
        }
    }

    fn add(&mut self, key: i32) {
        self.set[key as usize] = true;
    }

    fn remove(&mut self, key: i32) {
        self.set[key as usize] = false;
    }

    fn contains(&self, key: i32) -> bool {
        self.set[key as usize]
    }
}

/**
 * Your MyHashSet object will be instantiated and called as such:
 * let obj = MyHashSet::new();
 * obj.add(key);
 * obj.remove(key);
 * let ret_3: bool = obj.contains(key);
 */
fn main() {
    println!("Hello, world!");
}
