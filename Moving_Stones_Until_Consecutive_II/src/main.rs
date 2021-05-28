struct Solution;

impl Solution {
    pub fn num_moves_stones_ii(mut stones: Vec<i32>) -> Vec<i32> {
        stones.sort();
        let len = stones.len();
        let mut i = 0_usize;
        let mut j = 1_usize;
        let mut max = 0;
        while j < len {
            let left = stones[i];
            let right = stones[j];
            if right - left + 1 <= len as i32 {
                max = max.max(j - i + 1);
                j += 1;
            } else {
                i += 1;
            }
        }
        let min = len - max;
        let mut max = 0;
        for i in (0..stones.len() - 1).rev() {
            if stones[i + 1] - stones[i] > 1 {
                max += stones[i + 1] - stones[i];
                stones[i] = stones[i + 1] - 1;
            }
        }
        vec![min as i32, max]
    }
}
fn main() {
    println!("{:?}", Solution::num_moves_stones_ii(vec![6, 5, 4, 3, 10]));
}
