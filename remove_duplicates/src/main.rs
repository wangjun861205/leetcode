struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let len = nums.len();
        if len == 0 {
            return 0;
        }
        let mut prev = nums.remove(0);
        nums.push(prev);
        for _ in 0..len - 1 {
            let curr = nums.remove(0);
            if curr != prev {
                prev = curr;
                nums.push(curr);
            }
        }
        nums.len() as i32
    }
}
fn main() {
    let mut v = vec![1, 1, 1, 2, 2, 3, 3, 3];
    println!("{}", Solution::remove_duplicates(&mut v));
    println!("{:?}", v);
}
