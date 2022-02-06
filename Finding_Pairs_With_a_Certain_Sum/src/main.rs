use std::collections::HashMap;
struct FindSumPairs {
    nums1: Vec<i32>,
    nums2: Vec<i32>,
    nums2_count: HashMap<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FindSumPairs {
    fn new(nums1: Vec<i32>, nums2: Vec<i32>) -> Self {
        let nums2_count = nums2.iter().fold(HashMap::new(), |mut m, v| {
            *m.entry(*v).or_insert(0) += 1;
            m
        });
        Self {
            nums1,
            nums2,
            nums2_count,
        }
    }

    fn add(&mut self, index: i32, val: i32) {
        let ori = self.nums2[index as usize];
        let new = ori + val;
        self.nums2[index as usize] = new;
        *self.nums2_count.get_mut(&ori).unwrap() -= 1;
        *self.nums2_count.entry(new).or_insert(0) += 1;
    }

    fn count(&self, tot: i32) -> i32 {
        self.nums1
            .iter()
            .map(|n1| *self.nums2_count.get(&(tot - *n1)).or(Some(&0)).unwrap())
            .sum()
    }
}

/**
 * Your FindSumPairs object will be instantiated and called as such:
 * let obj = FindSumPairs::new(nums1, nums2);
 * obj.add(index, val);
 * let ret_2: i32 = obj.count(tot);
 */
fn main() {
    println!("Hello, world!");
}
