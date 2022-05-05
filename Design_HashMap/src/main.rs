struct MyHashMap {
    l: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashMap {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Self {
            l: vec![-1; 10i32.pow(6) as usize + 1],
        }
    }

    /** value will always be non-negative. */
    fn put(&mut self, key: i32, value: i32) {
        self.l[key as usize] = value;
    }

    /** Returns the value to which the specified key is mapped, or -1 if this map contains no mapping for the key */
    fn get(&self, key: i32) -> i32 {
        self.l[key as usize]
    }

    /** Removes the mapping of the specified value key if this map contains a mapping for the key */
    fn remove(&mut self, key: i32) {
        self.l[key as usize] = -1;
    }
}

/**
 * Your MyHashMap object will be instantiated and called as such:
 * let obj = MyHashMap::new();
 * obj.put(key, value);
 * let ret_2: i32 = obj.get(key);
 * obj.remove(key);
 */

fn main() {
    println!("Hello, world!");
}
