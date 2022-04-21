struct Solution;

use std::collections::BinaryHeap;

struct KthLargest {
    k: usize,
    heap: BinaryHeap<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut heap: BinaryHeap<i32> = nums.into_iter().collect();
        while heap.len() > k as usize {
            heap.pop();
        }
        Self {
            k: k as usize,
            heap: heap,
        }
    }

    fn add(&mut self, val: i32) -> i32 {
        self.heap.push(val);
        if self.heap.len() > self.k {
            self.heap.pop();
        }
        *self.heap.peek().unwrap()
    }
}

/**
 * Your KthLargest object will be instantiated and called as such:
 * let obj = KthLargest::new(k, nums);
 * let ret_1: i32 = obj.add(val);
 */

fn main() {
    let mut h = KthLargest::new(3, vec![4, 5, 8, 2]);
    println!("{}", h.heap.peek().unwrap());
    println!("{}", h.add(3));
    println!("{}", h.add(5));
    println!("{}", h.add(10));
    println!("{}", h.add(9));
    println!("{}", h.add(4));
}

// 2, 4, 5
// 2, 3, 4
// 2, 3, 4
//
