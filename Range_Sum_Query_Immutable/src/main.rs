struct NumArray {
    presum: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        Self {
            presum: nums.into_iter().fold(vec![0], |mut l, v| {
                l.push(*l.last().unwrap() + v);
                l
            }),
        }
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.presum[right as usize + 1] - self.presum[left as usize]
    }
}

/**
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * let ret_1: i32 = obj.sum_range(left, right);
 */
fn main() {
    let a = NumArray::new(vec![-2, 0, 3, -5, 2, -1]);
    assert_eq!(a.sum_range(0, 2), 1);
    assert_eq!(a.sum_range(2, 5), -1);
    assert_eq!(a.sum_range(0, 5), -3);
}
